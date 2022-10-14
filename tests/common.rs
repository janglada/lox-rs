use miette::{GraphicalReportHandler, GraphicalTheme, Result};
use rox::value::Value;
use rox::vm::VM;

pub fn assert_ok(vm: &mut VM, s: &'static str) -> Result<()> {
    // InterpretResult::Ok(val) => {
    //
    // }
    // InterpretResult::CompileError => {
    //     panic!("CompileError")
    // }
    // InterpretResult::RuntimeError => {
    //     panic!("RuntimeError")
    // }

    // vm.interpret(s)

    if let Err(err) = vm.interpret(s) {
        let mut out = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode())
            .with_width(80)
            .render_report(&mut out, err.as_ref())
            .unwrap();

        //println!("{}", out);
        return Err(err);
    }

    Ok(())
}

pub fn assert_ok_return_value(vm: &mut VM, s: &str, expected_value: Value) -> Result<()> {
    // let s = s.as_ref().map(|val| format!("return {}", val));
    let mut cmd = String::new();
    cmd.push_str("return ");
    cmd.push_str(s);
    return assert_ok_equals(vm, cmd.as_str(), expected_value);
}

///
///
///
pub fn assert_ok_equals(vm: &mut VM, s: &str, expected_value: Value) -> Result<()> {
    match vm.interpret(s)? {
        None => panic!("Test did not return"),
        Some(v) => {
            assert_eq!(expected_value, v);
            Ok(())
        }
    }
}

pub fn assert_runtime_error(vm: &mut VM, s: &'static str) -> Result<(), &'static str> {
    match vm.interpret(s) {
        Ok(_) => Err("Expected a runtime Error"),
        Err(_) => Ok(()),
    }
}

pub fn assert_compile_error(vm: &mut VM, s: &'static str) -> Result<(), &'static str> {
    match vm.interpret(s) {
        Ok(_) => Err("Expected a compile Error"),
        Err(_) => Ok(()),
    }

    // match vm.interpret(s) {
    //     InterpretResult::Ok(val) => {
    //         panic!(
    //             "Expected RuntimeError, found OK({})",
    //             val.unwrap_or(Value::String("empty".to_string()))
    //         )
    //     }
    //     InterpretResult::CompileError => {
    //         println!("CompileError")
    //     }
    //     InterpretResult::RuntimeError => {
    //         panic!("Expected CompileError found RuntimeError")
    //     }
    // }
}
