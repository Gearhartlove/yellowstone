extern crate core;

use std::fmt::{Debug, Display};
use yellowstone::error::InterpretError::{*, self};
use yellowstone::value::{allocate_object, Value, ValueKind, ValueUnion};
use yellowstone::vm::VM;
use anyhow::{Result, Error};


#[test]
fn compiler_unary() {
    let mut vm = VM::default();
    let source = "1";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(2.)));
}

#[test]
fn compiler_binary() {
    let mut vm = VM::default();
    let source = "1 + 1";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(2.)));

    let source = "1 - 2";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(-1.)));

    let source = "3 / 2";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(1.5)));

    let source = "2 * 2";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(4.)));
}

#[test]
fn compiler_grouping() {
    let mut vm = VM::default();
    let source = "(1)";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(1.)));
}

#[test]
fn compiler_number() {
    let mut vm = VM::default();
    let source = "2022";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(2022.)));
}

#[test]
fn compiler_precedence() {
    let mut vm = VM::default();
    let source = "2 * (1 + 1)";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(4.)));

    let mut vm = VM::default();
    let source = "(2 * -1) + 4 / 4";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(-1.)));

    let mut vm = VM::default();
    let source = "2 * ((-1 + 4 / 4) - 2)";
    let result = run_code(&mut vm, source).unwrap();
    assert_eq!(result, Some(Value::number_value(-4.)));
}

#[test]
fn compiler_asserteq_bool_test() {
    let mut vm = VM::default();
    let source = "var foo = true; assert_eq(foo, true);";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}


#[test]
fn compiler_asserteq_num_test() {
    let mut vm = VM::default();
    let source = "var foo = 42; assert_eq(foo, 42);";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn compiler_asserteq_nil_test() {
    let mut vm = VM::default();
    let source = "var foo = nil; assert_eq(foo, nil);";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn compiler_asserteq_string_test() {
    let mut vm = VM::default();
    let source = "var foo = \"foo\"; assert_eq(foo, \"foo\");";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}


#[test]
fn global_var_test() {
    let mut vm = VM::default();
    let source = "var lang = \"yellowstone\";";
    run_code(&mut vm, source);
}

#[test]
fn global_var_declaration() {
    let mut vm = VM::default();
    let source = "
        var lang = \"yellowstone\";
        var num = 9;
        var truth = true;
        var null = nil;

        assert_eq(lang, \"yellowstone\");
        assert_eq(num, 9);
        assert_eq(truth, true);
        assert_eq(nil, nil);
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn mutate_global_vars_test() {
    let mut vm = VM::default();
    let source = "
        var beverage = \"cafe au lait\"; 
        var breakfast = \"beignets with \" + beverage;
        assert_eq(breakfast, \"beignets with cafe au lait\");";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn local_var_declaration_test() {
    let mut vm = VM::default();
    let source = "
        {
            var lang = \"yellowstone\";
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn get_var_declaration_test() {
    let mut vm = VM::default();
    let source = "
        {
            var lang = \"yellowstone\";
            assert_eq(lang, \"yellowstone\");
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

#[test]
fn global_local_interaction_test() {
    let mut vm = VM::default();
    let source = "
        var foo = \"yellow\";
        {
            var lang = \"stone\";
            foo = foo + lang;
        }
        assert_eq!(foo, \"yellowstone\");
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() { eprintln!("{:?}", result); assert!(false) }
}

// #[test]
// fn undefined_local_error_test() {
//     let mut vm = VM::default();
//         let source = "
//             {
//                 var lang = \"yellowstone\";
//             }
//             lang
//         ";

//         // NOTE: this test will never fail, change run_code to return a result and match
//         // on that result. Look into the anyhow crate for this :)
//         let result = run_code(&mut vm, source);
//         assert_eq!(result, InterpretError::INTERPRET_RUNTIME_UNRECOGNIZED_VARIABLE_ERROR);
// }

// ################################################################################
// Helper Functions
// ################################################################################

pub fn run_code<T: ToString + Display>(vm: &mut VM, code: T) -> Result<Option<Value>> {
    vm.interpret(&code.to_string())
}

pub fn num_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValNumber => Some(value.as_number().unwrap()),
            _ => { None }
        }
    } else {
        return None;
    }
}

pub fn nil_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValNil => Some(value.as_nil().unwrap()),
            _ => { None }
        }
    } else {
        return None;
    }
}

pub fn bool_val(vm: &mut VM, variable_name: &'static str) -> Option<bool> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValBool => Some(value.as_bool().unwrap()),
            _ => { None }
        }
    } else {
        return None;
    }
}

pub fn str_val(vm: &mut VM, variable_name: &'static str) -> Option<String> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValObj => Some(value.as_string().unwrap()),
            _ => { None }
        }
    } else {
        return None;
    }
}

pub fn get_constant_name(vm: &VM, i: usize) -> Option<String> {
    return match vm.chunk.constants.get(i) {
        Some(c) => {
            Some(c.as_string().unwrap())
        },
        None => { None }
    }
}