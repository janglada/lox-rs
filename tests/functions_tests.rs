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

fun sum(a, b) {
    return a + b;
}
var s = sum(5, 6;
print 4 + s;


        "#,
        )
    }

    #[test]
    fn vm_function_return_string() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"

fun areWeHavingItYet() {
    return "Yes we are";
}
return areWeHavingItYet();


        "#,
            Value::String("Yes we are".to_string()),
        )
    }

    #[test]
    fn vm_function_compile() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"

fun one(a) {
    var b = a + 200;
    var c = b + 300;
    return c;
}
return one(100);



        "#,
            Value::Number(600 as f64),
        )
    }
    #[test]
    fn vm_function() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            
print "HELLO";
fun square(x) {
    return x*x;
}
var sq = square(3);
return sq;
        "#,
            Value::Number(9 as f64),
        )
    }

    #[test]
    fn vm_fibonacci() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun fib(n) {
    print "FIB " + n;

    print n < 2;
    if (2 > n) {
        print "RETURNING..." + n ; 
        return n;
    } else {
        let f2 =  fib(n-2);
        let f1 =  fib(n-1);
        let a = "FIB (" + n;
        let b = a + ") =";
        let c = b + f1 + f2;
        print c;
        return f1 + f2;
    }
}

return   fib(6);
        "#,
            Value::Number(8 as f64),
        )
    }
}
