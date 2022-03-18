use crate::chunk::ChunkWriterTrait;
use crate::function::ObjectFunction;
use crate::opcode::Opcode;
use crate::opcode::Opcode::OpJumpIfFalse;
use crate::parser::Parser;
use crate::precedence::{ParserRule, Precedence};

use crate::token::{Token, TokenType};
use crate::value::Value;

use num_traits::FromPrimitive;

use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct Compiler {
    pub(crate) enclosing: Option<Box<Compiler>>,
    // current_compilers: &'a mut CurrentCompiler<'a>,
    pub(crate) function: Box<ObjectFunction>,
    // scope
    pub(crate) locals: ArrayVec<Local, 256>,
    pub(crate) local_count: usize,
    pub(crate) scope_depth: isize,
}

///
///
///
#[derive(Debug, Clone)]
pub struct Local {
    pub(crate) token: Option<Token>, // clone!!! noooo, just a ref..
    pub(crate) depth: isize,
}

///

impl Compiler {
    // pub fn new(name: &str, ftype: FunctionType) -> Box<Self> {
    //     const INIT: Option<Local> = None;
    //
    //     Box::new(Compiler {
    //         enclosing: None,
    //         function: ObjectFunction::new(ftype, name.to_string()),
    //         locals: Vec::with_capacity(256),
    //         local_count: 0,
    //         scope_depth: 0,
    //     })
    // }
    pub fn new2(mut func: ObjectFunction) -> Box<Self> {
        const INIT: Option<Local> = None;

        // Slot '0' is claimed by VM internal usage
        let mut locals = ArrayVec::new();
        locals.push(Local {
            token: None,
            depth: 0,
        });

        Box::new(Compiler {
            enclosing: None,
            function: Box::new(func),
            locals,
            local_count: 1,
            scope_depth: 0,
        })
    }

    pub(crate) fn add_local(&mut self, local: Local, errors: &mut Vec<&'static str>) {
        if self.locals.len() == 256 {
            errors.push("Too many local variables in function");
            return;
        }

        self.locals.push(local);
    }

    pub(crate) fn resolve_local(
        &mut self,
        token: &Token,
        errors: &mut Vec<&'static str>,
    ) -> Option<usize> {
        // let token = &self.parser.previous;
        let local = self
            .locals
            .iter()
            .enumerate()
            .rev()
            .find(|(_i, l)| Parser::identifiers_equal(token, &l.token));

        if let Some((i, l)) = local {
            // println!("RESOLVE LOCAL AT {} LEN = {}", i, self.locals.len());
            if l.depth == -1 {
                errors.push("Can't read local variable in its own initializer")
            }
            Some(i)
        } else {
            None
        }

        // local.map(|a| a.0)
    }
}

pub fn number(parser: &mut Parser, _can_assign: bool) {
    match &parser.previous.token_type {
        TokenType::Number(num) => parser
            .compiler
            .function
            .emit_constant(Value::Number(*num), parser.previous.line),
        _ => panic!("unexpected token type"),
    }
}

pub fn string(parser: &mut Parser, _can_assign: bool) {
    match &parser.previous.token_type {
        TokenType::String(str) => {
            dbg!(str);
            parser
                .compiler
                .function
                .emit_constant(Value::new_string(str), parser.previous.line)
        }
        _ => panic!("unexpected token type"),
    }
}

///
///
pub fn variable(parser: &mut Parser, can_assign: bool) {
    named_variable(parser, can_assign);
}

///
///
pub fn named_variable(parser: &mut Parser, can_assign: bool) {
    let _index = parser.identifier_constant();

    let (get_op, set_op) = if let Some(index) = parser.resolve_local() {
        (Opcode::OpGetLocal(index), Opcode::OpSetLocal(index))
    } else {
        let index = parser.identifier_constant();
        (Opcode::OpGetGlobal(index), Opcode::OpSetGlobal(index))
    };

    if can_assign && parser.match_token(TokenType::Equal) {
        parser.expression();
        parser
            .compiler
            .function
            .emit_byte(set_op, parser.previous.line)
    } else {
        parser
            .compiler
            .function
            .emit_byte(get_op, parser.previous.line)
    }
}

///
///
pub fn grouping(compiler: &mut Parser, _can_assign: bool) {
    compiler.expression();
    compiler.consume(TokenType::RightParen, "Expected ')' after expression")
}

///
///
pub fn literal(parser: &mut Parser, _can_assign: bool) {
    let token_type = &parser.previous.token_type.clone();
    match token_type {
        TokenType::False => parser
            .compiler
            .function
            .emit_byte(Opcode::OpFalse, parser.previous.line),
        TokenType::Nil => parser
            .compiler
            .function
            .emit_byte(Opcode::OpNil, parser.previous.line),
        TokenType::True => parser
            .compiler
            .function
            .emit_byte(Opcode::OpTrue, parser.previous.line),
        _ => {}
    }
}

///
///
pub fn unary(parser: &mut Parser, _can_assign: bool) {
    let token_type = &parser.previous.token_type.clone();
    // compile the operand
    parser.parse_precedence(&Precedence::Unary);

    // Emit the operator instruction
    match token_type {
        TokenType::Bang => parser
            .compiler
            .function
            .emit_byte(Opcode::OpNot, parser.previous.line),
        TokenType::Minus => parser
            .compiler
            .function
            .emit_byte(Opcode::OpNegate, parser.previous.line),
        _ => {}
    }
}
///
///
pub fn binary(parser: &mut Parser, _can_assign: bool) {
    let token_type = &parser.previous.token_type.clone();
    // compile the operand

    let rule = ParserRule::get_rule(token_type);
    let prec_u8 = (*rule.precedence) as u8;
    let precedence: Precedence = FromPrimitive::from_u8(prec_u8 + 1).unwrap();
    parser.parse_precedence(&precedence);
    //  let rule = self.getRule(token_type);
    // self.parse_precedence()

    // Emit the operator instruction
    match token_type {
        TokenType::Plus => parser
            .compiler
            .function
            .emit_byte(Opcode::OpAdd, parser.previous.line),
        TokenType::Minus => parser
            .compiler
            .function
            .emit_byte(Opcode::OPSubtract, parser.previous.line),
        TokenType::Star => parser
            .compiler
            .function
            .emit_byte(Opcode::OPMultiply, parser.previous.line),
        TokenType::Slash => parser
            .compiler
            .function
            .emit_byte(Opcode::OpDivide, parser.previous.line),

        TokenType::BangEqual => parser
            .compiler
            .function
            .emit_byte(Opcode::OpEqual, parser.previous.line),
        TokenType::EqualEqual => parser
            .compiler
            .function
            .emit_byte(Opcode::OpEqual, parser.previous.line),
        TokenType::Greater => parser
            .compiler
            .function
            .emit_byte(Opcode::OpGreater, parser.previous.line),
        TokenType::GreaterEqual => {
            parser
                .compiler
                .function
                .emit_bytes(Opcode::OpLess, Opcode::OpNot, parser.previous.line)
        }
        TokenType::Less => parser
            .compiler
            .function
            .emit_byte(Opcode::OpLess, parser.previous.line),
        TokenType::LessEqual => parser.compiler.function.emit_bytes(
            Opcode::OpGreater,
            Opcode::OpNot,
            parser.previous.line,
        ),
        _ => {}
    }
}

///
///
pub fn and(parser: &mut Parser, _can_assign: bool) {
    let end_jump = parser.emit_jump(OpJumpIfFalse(0));
    parser
        .compiler
        .function
        .emit_byte(Opcode::OpPop, parser.previous.line);
    parser.parse_precedence(&Precedence::And);
    parser.patch_jump(end_jump, &Opcode::OpJumpIfFalse(0))
}

///
///
pub fn or(parser: &mut Parser, _can_assign: bool) {
    let else_jump = parser.emit_jump(OpJumpIfFalse(0));
    let end_jump = parser.emit_jump(Opcode::OpJump(0));

    parser.patch_jump(else_jump, &Opcode::OpJumpIfFalse(0));

    parser
        .compiler
        .function
        .emit_byte(Opcode::OpPop, parser.previous.line);
    parser.parse_precedence(&Precedence::Or);
    parser.patch_jump(end_jump, &Opcode::OpJump(0));
}
///
///
pub fn call(parser: &mut Parser, _can_assign: bool) {
    let arg_count: u8 = parser.argument_list();

    parser
        .compiler
        .function
        .emit_byte(Opcode::OpCall(arg_count), parser.previous.line);
}
