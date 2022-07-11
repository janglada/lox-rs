use rox;

mod common;

#[cfg(test)]
mod tests {
    use crate::common::{assert_compile_error, assert_ok, assert_ok_value, assert_runtime_error};
    use miette::{IntoDiagnostic, Result};
    use rox::value::Value;
    use rox::vm::VM;
    #[test]
    fn vm_function_compile() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun one() {
    return 1;
}
print one();

        "#,
        )
    }
    #[test]
    fn vm_function() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"            
print "HELLO";
fun square(x) {
    return x*x;
}
print square(3);
        "#,
        )
    }

    #[test]
    fn vm_fibonacci() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
fun fib(n) {
    if (n < 2) return n;
    return fib(n-2) +  fib(n-1);
}
print fib(3);
        "#,
        )
    }
}
