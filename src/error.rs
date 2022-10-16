use miette::{Diagnostic, IntoDiagnostic, SourceSpan};
use miette::{Error, NamedSource};

use std::fmt;

use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(oops::lox::compileError))]
#[error("{label}")]
pub struct LoxCompileError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    pub(crate) src: NamedSource,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    pub(crate) bad_bit: SourceSpan,

    pub(crate) label: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Runtime Error")]
#[diagnostic(code(oops::lox::runtimeError), help("A runtime error occurred"))]
pub struct LoxRuntimeError {}

impl LoxRuntimeError {
    pub fn new(msg: &str) -> Self {
        LoxRuntimeError {}
    }
}

// #[derive(Error, Debug, Diagnostic)]
// #[diagnostic(code(oops::lox::runtimeError))]
// pub struct WrongTypeError {
//     pub(crate) error: String,
// }
//
// impl WrongTypeError {
//     pub fn new(label: &str) -> Self {
//         Error::msg(label)
//     }
// }
