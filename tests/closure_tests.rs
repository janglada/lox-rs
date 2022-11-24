use rox;

mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        assert_compile_error, assert_ok, assert_ok_equals, assert_ok_return_value,
        assert_runtime_error,
    };
    use miette::{IntoDiagnostic, Result};
    use rox::value::Value;
    use rox::vm::VM;

    #[test]
    fn closure_test() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
var f;
{
  var a = "a";
  fun f_() {
    print a;
    print a;
  }
  f = f_;
}

f();
        "#,
        )
    }

    #[test]
    fn closure_test_a() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun makeClosure(value) {
    fun closure() {
        print value;
    }
    return closure;
}
var hi = makeClosure("hi");
var there = makeClosure("there");
hi();
there();
        "#,
        )
    }
}
