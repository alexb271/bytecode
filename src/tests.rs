use crate::Compiler;
use crate::interpreter::{Thread, Value};

fn process_and_unwrap_value(compiler: &mut Compiler, input: &str) -> Value {
    match compiler.compile(input, "stdin") {
        Ok(bytecode) => {
            let mut thread = Thread::new(bytecode);
            thread.exec();
            thread.return_value().clone().unwrap()
        }
        Err(e) => panic!("{}", e.as_str()),
    }
}

#[test]
fn operations_int() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "5 == 5");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "5 != 5");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "5 < 5");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "5 <= 5");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "6 > 5");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "6 >= 7");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "3 + 7");
    assert_eq!(output, Value::Int(10));

    let output = process_and_unwrap_value(&mut compiler, "2 - 5");
    assert_eq!(output, Value::Int(-3));

    let output = process_and_unwrap_value(&mut compiler, "7 * 8");
    assert_eq!(output, Value::Int(56));

    let output = process_and_unwrap_value(&mut compiler, "7 / 3");
    assert_eq!(output, Value::Int(2));

    let output = process_and_unwrap_value(&mut compiler, "7 % 3");
    assert_eq!(output, Value::Int(1));

    let output = process_and_unwrap_value(&mut compiler, "-(5)");
    assert_eq!(output, Value::Int(-5));
}

#[test]
fn operations_float() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "5.0 == 5.0");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "5.0 != 5.0");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "5.0 < 5.0");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "5.0 <= 5.0");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "6.0 > 5.0");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "6.0 >= 7.0");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "3.2 + 7.1");
    assert_eq!(output, Value::Float(10.3));

    let output = process_and_unwrap_value(&mut compiler, "2.7 - 5.0");
    assert_eq!(output, Value::Float(-2.3));

    let output = process_and_unwrap_value(&mut compiler, "7.2 * 8.6");
    assert_eq!(output, Value::Float(61.92));

    let output = process_and_unwrap_value(&mut compiler, "9.0 / 2.0");
    assert_eq!(output, Value::Float(4.5));

    let output = process_and_unwrap_value(&mut compiler, "7.0 % 3.0");
    assert_eq!(output, Value::Float(1.0));

    let output = process_and_unwrap_value(&mut compiler, "-(5.0)");
    assert_eq!(output, Value::Float(-5.0));
}

#[test]
fn operations_bool() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "true == true");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "true != true");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "true and false");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "false or true");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "not true");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "not not false");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "not not not false");
    assert_eq!(output, Value::Bool(true));
}

#[test]
fn operations_string() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" == \"abc\"");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" != \"abc\"");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" < \"abc\"");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" <= \"abc\"");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\"def\" > \"abc\"");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\"def\" >= \"dg\"");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" + \"def\"");
    assert_eq!(output, Value::Str(Box::new(String::from("abcdef"))));

    let output = process_and_unwrap_value(&mut compiler, "\"abc\" * 3");
    assert_eq!(output, Value::Str(Box::new(String::from("abcabcabc"))));

    let output = process_and_unwrap_value(&mut compiler, "3 * \"abc\"");
    assert_eq!(output, Value::Str(Box::new(String::from("abcabcabc"))));

    let output = process_and_unwrap_value(&mut compiler, "0 * \"abc\"");
    assert_eq!(output, Value::Str(Box::new(String::from(""))));

    let output = process_and_unwrap_value(&mut compiler, "-3 * \"abc\"");
    assert_eq!(output, Value::Str(Box::new(String::from(""))));
}

#[test]
fn operations_char() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "\'C\' == \'C\'");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\'C\' != \'C\'");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "\'C\' < \'C\'");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "\'C\' <= \'C\'");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\'R\' > \'C\'");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "\'C\' >= \'R\'");
    assert_eq!(output, Value::Bool(false));
}

#[test]
fn number_parsing() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "11");
    assert_eq!(output, Value::Int(11));

    let output = process_and_unwrap_value(&mut compiler, "10_000_000");
    assert_eq!(output, Value::Int(10000000));

    let output = process_and_unwrap_value(&mut compiler, "10.0");
    assert_eq!(output, Value::Float(10.0));

    let output = process_and_unwrap_value(&mut compiler, "1_2_3.3_2_1");
    assert_eq!(output, Value::Float(123.321));
}

#[test]
fn expression() {
    let mut compiler = Compiler::new();

    let output = process_and_unwrap_value(&mut compiler, "(3/2) * 3");
    assert_eq!(output, Value::Int(3));

    let output = process_and_unwrap_value(&mut compiler, "5+(2-3)*4");
    assert_eq!(output, Value::Int(1));

    let output = process_and_unwrap_value(&mut compiler, "5+2-3*4");
    assert_eq!(output, Value::Int(-5));

    let output = process_and_unwrap_value(&mut compiler, "5 + -2");
    assert_eq!(output, Value::Int(3));

    let output = process_and_unwrap_value(&mut compiler, "456-41-675*8-15");
    assert_eq!(output, Value::Int(-5000));

    let output = process_and_unwrap_value(&mut compiler, "(456-41-675)*8-15");
    assert_eq!(output, Value::Int(-2095));

    let output = process_and_unwrap_value(&mut compiler, "3+4*50/25%5-1");
    assert_eq!(output, Value::Int(5));

    let output =
        process_and_unwrap_value(&mut compiler, "-10.0-(1.0+-4.0/16.0)*8.0-(7.0%2.0)*2.0/5.0");
    assert_eq!(output, Value::Float(-16.4));

    let output = process_and_unwrap_value(&mut compiler, "false or true and true");
    assert_eq!(output, Value::Bool(true));

    let output = process_and_unwrap_value(&mut compiler, "true and true and false");
    assert_eq!(output, Value::Bool(false));

    let output = process_and_unwrap_value(&mut compiler, "false or false or true");
    assert_eq!(output, Value::Bool(true));
}
