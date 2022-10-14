use rox;

mod common;

#[cfg(test)]
mod tests {
    use crate::common::{assert_compile_error, assert_ok, assert_ok_value, assert_runtime_error};
    use miette::{IntoDiagnostic, Result};
    use rox::value::Value;
    use rox::vm::VM;

    #[test]
    fn vm_function_simple_compile() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun one(a) {
    return a + "2";
}
var c =  one("1");


        "#,
        )
    }

    #[test]
    fn vm_function_simple_sum() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun sum(a, b, c) {
    return a + b + c;
}
print 4 + sum(5, 6, 7);


        "#,
        )
    }

    #[test]
    fn vm_function_return_string() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun areWeHavingItYet() {
    return "Yes we are";
}
print areWeHavingItYet();


        "#,
        )
    }

    #[test]
    fn vm_function_compile() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

fun one(a) {
    var b = a + 200;
    var c = b + 300;
    return c;
}
print one(100);
print "A";


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
var sq = square(3);
print sq;
        "#,
        )
    }

    #[test]
    fn vm_fibonacci() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
fun fib(n) {
    print "FIB " + n;

    print n < 2;
    if (n < 2) {
        print "RETURNING..." + n ; 
        return n;
    } else {
        return fib(n-2) +  fib(n-1);
    }
}

print   fib(6);
        "#,
        )
    }
}
