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
    fn vm_function_2_args() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun sum(a, b) {
    return a + b;
}
return sum(1,2);
        "#,
            Value::Number(3 as f64),
        )
    }

    #[test]
    fn vm_function_3_args() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun sum(a, b, c) {
    return a + b + c;
}
return sum(1,2,3);
        "#,
            Value::Number(6 as f64),
        )
    }

    #[test]
    fn vm_function_3_args_multiple_types() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun sum(a, b, c) {
    print a;
    print b;
    print c;
    return b;
}
return sum("AAAA",2,false);
        "#,
            Value::Number(2 as f64),
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
    fn vm_function_within_function() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"

fun thrice(a) {
    return a + double(a);
}

fun double(a) {
    return a + a;
}
return thrice(100);



        "#,
            Value::Number(300 as f64),
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
    // print "FIB n= " + n;
    if (n < 2) {
        // print "RETURN " + n;
        return n;
    } else {
        return  n + fib(n-1);
    }
}

  fib(6);
        "#,
            Value::Number(8 as f64),
        )
    }

    #[test]
    fn vm_fn_native_no_args() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
var start =  clock();
var sum = 0;
for (var i = 0; i < 10000; i = i + 1) {
    sum =  sum + i;
    print i;

}

print clock() - start;
        "#,
        )
    }

    #[test]
    fn vm_fn_native_1_arg() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"

return sin(1.5709);
        "#,
            Value::Number(1.5709_f64.sin()),
        )
    }
}
