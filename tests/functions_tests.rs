mod common;

#[cfg(test)]
mod tests {
    use crate::common::{assert_ok, assert_ok_equals};
    use miette::Result;
    use rox::value::Value;
    use rox::vm::VM;

    ///
    ///
    ///
    #[test]
    #[should_panic]
    fn vm_assert_too_many_args() -> () {
        VM::new()
            .interpret(
                r#"
fun f(a, b) {
  print a;
  print b;
}

f(1, 2, 3, 4); // expect runtime error: Expected 2 arguments but got 4.
        "#,
            )
            .expect("Should panic");
        ()
    }

    #[test]
    #[should_panic]
    fn vm_assert_missing_many_args() -> () {
        VM::new()
            .interpret(
                r#"
fun f(a, b) {
  print a;
  print b;
}

f(1); // expect runtime error: Expected 2 arguments but got 4.
        "#,
            )
            .expect("Should panic");
        ()
    }
    #[test]
    #[should_panic]
    fn vm_assert_missing_comma() -> () {
        VM::new()
            .interpret(
                r#"
fun foo(a, b c, d, e, f) {}
        "#,
            )
            .expect("Should panic");
        ()
    }
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
    fn vm_2_chained_calls() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            

fun a(x) {
    return b(x+1);
}
fun b(x) {
   return x;
}
    

return a(2);
        "#,
            Value::Number(3 as f64),
        )
    }

    #[test]
    fn vm_3_chained_calls() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            

fun a(x) {
    return b(x+1);
}
fun b(x) {
   return c(x+1);
}
    
fun c(x) {
   return x;
}
return a(1);
        "#,
            Value::Number(3 as f64),
        )
    }

    #[test]
    fn vm_2_chained_calls_and_add_1() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            

fun a(x) {
    return b(x+1);
}
fun b(x) {
   return x;
}
    

return 1 + a(2);
        "#,
            Value::Number(4 as f64),
        )
    }
    #[test]
    fn vm_2_chained_calls_and_add_2() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            

fun a(x) {
    return b(x+1);
}
fun b(x) {
   return x;
}
    

return 2 + a(1);
        "#,
            Value::Number(4 as f64),
        )
    }
    #[test]
    fn vm_nested_chained_calls() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"            

fun a(x) {

    // 
    fun b(x) {
       //
       fun c(x) {
            return x;
       }
       return c(x+1);
    }
    
    return b(x+1);
}

return 1 + a(1);
        "#,
            Value::Number(4 as f64),
        )
    }

    ///
    ///
    ///
    #[test]
    #[should_panic]
    fn vm_assert_stack_overflow() -> () {
        VM::new()
            .interpret(
                r#"
fun a(x) {
    return a(x+1);
}
print a(1);
        "#,
            )
            .expect("Should panic");
        ()
    }
    #[test]
    fn vm_factorial() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun fib(n) {

    if (n < 2) {
        return n;
    } else {
        var r = n * fib(n-1);
        return  r;
    }
}

var a = fib(3);
return a;
        "#,
            Value::Number(6 as f64),
        )
    }

    ///
    ///
    ///
    #[test]
    fn vm_fibonacci() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun fib(n) {
    print "FIB n= " + n;
    print n < 2;
    if (n < 2) {
        //print "RETURN (a)=" + n;
        return n;
    } else { 
        //var r = n + fib(n-1);
        //print "RETURN (b)=" + r;
        return  fib(n-1) + fib(n-2);
    }
}

return fib(8);
        "#,
            Value::Number(21 as f64),
        )
    }

    //
    //

    #[test]
    fn vm_nested_call() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
fun returnArg(arg) {
  return arg;
}

fun returnFunCallWithArg(func, arg) {
  return returnArg(func)(arg);
}

fun printArg(arg) {
  return  arg;
}

return returnFunCallWithArg(printArg, "hello world"); // expect: hello world
        "#,
            Value::String("hello world".to_string()),
        )
    }

    ///
    ///
    ///
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
