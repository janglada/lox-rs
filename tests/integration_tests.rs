mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        assert_compile_error, assert_ok, assert_ok_equals, assert_ok_return_value,
        assert_runtime_error,
    };
    use miette::Result;
    use rox::value::Value;
    use rox::vm::VM;
    #[test]
    fn vm_multiply() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "1*2;", Value::Number(2f64));
        assert_ok_return_value(&mut VM::new(), "1*2*3;", Value::Number(6f64))
    }
    #[test]
    fn vm_add() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "1 + 2;", Value::Number(3f64));
        assert_ok_return_value(&mut VM::new(), "1 + 2 + 3 + 4;", Value::Number(10f64));
        assert_ok_equals(
            &mut VM::new(),
            "var c = 1 + 2 + 3 + 4;\nreturn c;",
            Value::Number(10f64),
        )
    }

    #[test]
    fn vm_unary() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "-1;", Value::Number(-1f64))
    }
    #[test]
    fn vm_number() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "1;", Value::Number(1f64))
    }
    #[test]
    fn vm_grouping() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), " -(1);", Value::Number(-1f64))
    }
    #[test]
    fn vm_minus() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), " 2+5*10;", Value::Number(52f64))
    }

    #[test]
    fn vm_bool_t() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), " true;", Value::Boolean(true))
    }

    #[test]
    fn vm_bool_f() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), " false;", Value::Boolean(false))
    }
    #[test]
    fn vm_bool_not() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), " !false;", Value::Boolean(true))
    }
    #[test]
    fn vm_nil() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "nil;", Value::Nil)
    }

    #[test]
    fn vm_not_nil() -> Result<(), &'static str> {
        assert_runtime_error(&mut VM::new(), "!nil;")
    }

    #[test]
    fn vm_not_number() -> Result<(), &'static str> {
        assert_runtime_error(&mut VM::new(), "!3.14;")
    }

    #[test]
    fn vm_negate_bool() -> Result<(), &'static str> {
        assert_runtime_error(&mut VM::new(), "-false;")
    }
    #[test]
    fn vm_negate_nil() -> Result<(), &'static str> {
        assert_runtime_error(&mut VM::new(), "-nil;")
    }

    #[test]
    fn vm_greater() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "2 > 1;", Value::Boolean(true));
        assert_ok_return_value(&mut VM::new(), "2 >= 1;", Value::Boolean(true))
    }

    #[test]
    fn vm_less() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "2 < 1;", Value::Boolean(false));
        assert_ok_return_value(&mut VM::new(), "2 <= 1;", Value::Boolean(false))
    }
    #[test]
    fn vm_equal() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "2 == 2;", Value::Boolean(true))
    }

    #[test]
    fn vm_equal_fail() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), "2 == 2;", Value::Boolean(true))
    }

    #[test]
    fn vm_str_eval() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), r#""A";"#, Value::String("A".to_string()))
    }

    #[test]
    fn vm_str_compare() -> Result<()> {
        assert_ok_return_value(&mut VM::new(), r#""A" == "A";"#, Value::Boolean(true));
        assert_ok_return_value(&mut VM::new(), r#""A" == "B";"#, Value::Boolean(false))
    }

    #[test]
    fn vm_add_str() -> Result<()> {
        assert_ok_return_value(
            &mut VM::new(),
            r#""A" + "b";"#,
            Value::String("Ab".to_string()),
        )
    }

    #[test]
    fn vm_add_distinct_types() -> Result<()> {
        assert_ok_return_value(
            &mut VM::new(),
            r#""A" + 3.1;"#,
            Value::String("A3.1".to_string()),
        )
    }

    #[test]
    fn vm_add_distinct_types_2() -> Result<()> {
        assert_ok_return_value(
            &mut VM::new(),
            r#" 3.1 + "A";"#,
            Value::String("3.1A".to_string()),
        )
    }

    #[test]
    fn vm_print_expr() -> Result<()> {
        assert_ok(&mut VM::new(), "print 1 + 2;")
    }
    #[test]
    fn vm_global_get() -> Result<()> {
        assert_ok_equals(
            &mut VM::new(),
            r#"
        var beverage = "cafe au lait";
        var breakfast = "beignets with " + beverage ;
        return breakfast;
        "#,
            Value::String("beignets with cafe au lait".to_string()),
        )
    }

    #[test]
    fn vm_global_set() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
var beverage  = "cafe au lait";
var breakfast = "beignets";
breakfast = breakfast + " with " +   beverage ;
print breakfast;
        "#,
        )?;

        Ok(())
    }

    #[test]
    fn vm_local_set_duplicate() -> Result<(), &'static str> {
        assert_compile_error(
            &mut VM::new(),
            r#"
{
    var a ="first";
    var a = "second"
}
        "#,
        )
    }

    #[test]
    fn vm_local_set1() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
{
    var a = "outer";
    {
        var a =  "inner";
        print "INNER A:";
        print a;
    }
    print "OUTER A:";
    print a;
}
        "#,
        )
    }
    #[test]
    fn vm_local_set_2() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
{
    var a = "outer";
    {
        var b =  "inner";
        var c =  "hi " + b;
        print c;
    }
}
        "#,
        )
    }

    #[test]
    fn vm_if_stmt() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
print "1";
if (false) {
    print "2";
}
if (true) {
    print "3";
}
print "4";
        "#,
        )
    }

    #[test]
    fn vm_if_else_stmt() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
print "1";
if (false) {
    print "2";
} else {
    print "3";
}
print "4";
        "#,
        )
    }

    #[test]
    fn vm_logical_and() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

var a =  true;
var b =  false;
a and b;


        "#,
        )
    }
    #[test]
    fn vm_logical_or() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

var a =  true;
var b =  false;
print a or b;


        "#,
        )
    }

    #[test]
    fn vm_while() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"
var a = 0;
while(a < 3) {
    print a;
    a =  a + 1;
}
        "#,
        )
    }

    #[test]
    fn vm_for() -> Result<()> {
        assert_ok(
            &mut VM::new(),
            r#"

for (var i = 0; i < 10; i = i + 2) {

print i;

}
        "#,
        )
    }
}
