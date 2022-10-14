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

pub fn assert_ok_value(vm: &mut VM, s: &'static str, _expected_value: Value) -> Result<()> {
    vm.interpret(s)

    // match vm.interpret(s) {
    //     InterpretResult::Ok(val) => {
    //         if let Some(r) = val {
    //             println!("Ok {}", r);
    //             assert_eq!(expected_value, r)
    //         } else {
    //             println!("Ok(empty)");
    //         }
    //     }
    //     InterpretResult::CompileError => {
    //         panic!("CompileError")
    //     }
    //     InterpretResult::RuntimeError => {
    //         panic!("RuntimeError")
    //     }
    // }
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
