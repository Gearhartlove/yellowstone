extern crate core;

use std::fmt::{Debug, Display};
use yellowstone::value::{allocate_object, Value, ValueKind, ValueUnion};
use yellowstone::vm::VM;

pub fn run_code<T: ToString + Display>(vm: &mut VM, code: T) -> Option<Value> {
    // println!("{code}");
    let result = vm.interpret(&code.to_string());

    match result {
        Err(e) => {
            println!("{:?}", e);
            return None;
        }
        Ok(_) => return result.unwrap(),
    }
}

#[test]
fn compiler_unary() {
    let mut vm = VM::default();
    let source = "1";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(2.)));
}

#[test]
fn compiler_binary() {
    let mut vm = VM::default();
    let source = "1 + 1";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(2.)));

    let source = "1 - 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(-1.)));

    let source = "3 / 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(1.5)));

    let source = "2 * 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(4.)));
}

#[test]
fn compiler_grouping() {
    let mut vm = VM::default();
    let source = "(1)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(1.)));
}

#[test]
fn compiler_number() {
    let mut vm = VM::default();
    let source = "2022";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(2022.)));
}

#[test]
fn compiler_precedence() {
    let mut vm = VM::default();
    let source = "2 * (1 + 1)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(4.)));

    let mut vm = VM::default();
    let source = "(2 * -1) + 4 / 4";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(-1.)));

    let mut vm = VM::default();
    let source = "2 * ((-1 + 4 / 4) - 2)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(Value::number_value(-4.)));
}

//#[test]
//fn print_statement() {
//    let mut vm = VM::default();
//    let source = "print \"Hello Yellowstone!\";";
//    let result = run_code(&mut vm, source);
//}

#[test]
fn global_var_declaration() {
    let mut vm = VM::default();
    let source = "
        var lang = \"yellowstone\";
        var num = 9;
        var yes = true;
        var nothing = nil;
    ";
    run_code(&mut vm, source);

    assert_eq!(str_val(&mut vm, "lang"), Some(String::from("yellowstone")));
    assert_eq!(num_val(&mut vm, "num"), Some(9.));
    assert_eq!(bool_val(&mut vm, "yes"), Some(true));
    assert_eq!(nil_val(&mut vm, "nothing"), Some(0.));
}

// Helper Functions
pub fn num_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValNumber => Some(value.as_number().unwrap()),
        }
    } else {
        return None;
    }
}

pub fn nil_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValNil => Some(value.as_number().unwrap()),
        }
    } else {
        return None;
    }
}

pub fn bool_val(vm: &mut VM, variable_name: &'static str) -> Option<bool> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValBool => Some(value.as_bool().unwrap()),
        }
    } else {
        return None;
    }
}

pub fn str_val(vm: &mut VM, variable_name: &'static str) -> Option<String> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValBool => Some(value.as_string().unwrap()),
        }
    } else {
        return None;
    }
}
