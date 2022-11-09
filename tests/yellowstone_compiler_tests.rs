extern crate core;

use std::fmt::{Debug, Display};
use yellowstone::error::InterpretError;
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
        var yes = true;
        var nothing = nil;
    ";
    run_code(&mut vm, source);

    assert_eq!(num_val(&mut vm, "num"), Some(9.));
    assert_eq!(str_val(&mut vm, "lang"), Some(String::from("yellowstone")));
    assert_eq!(bool_val(&mut vm, "yes"), Some(true));
    assert_eq!(nil_val(&mut vm, "nothing"), Some(0.));

    assert_eq!(num_val(&mut vm, "nonumber"), None);
    assert_eq!(bool_val(&mut vm, "nobool"), None);
    assert_eq!(nil_val(&mut vm, "nonil"), None);
    assert_eq!(str_val(&mut vm, "nostr"), None);
}

#[test]
fn mutate_global_vars_test() {
    let mut vm = VM::default();
    let source = "
        var beverage = \"cafe au lait\"; 
        var breakfast = \"beignets with \" + beverage;";

    let _ = run_code(&mut vm, source);
    assert_eq!(str_val(&mut vm, "breakfast"), Some(String::from("beignets with cafe au lait")));
}

#[test]
fn local_var_declaration_test() {
    let mut vm = VM::default();
        let source = "
            {
                var lang = \"yellowstone\";
            }
        ";

        // NOTE: this test will never fail, change run_code to return a result and match
        // on that result. Look into the anyhow crate for this :)
        let _ = run_code(&mut vm, source);
}


/* 
// Develment Note: for whatever reason when I define a global variable, two of them are pushed 
// onto the constant stack. also I am not able to define multiple local variables in one scope.
// Need to: 1) get multiple local vars working 2) understand global vars and local vars being constants.
*/
#[test]
fn get_var_declaration_test() {
    let mut vm = VM::default();
        let source = "
            {
                var test = \"test\";
                var lang = \"yellowstone\";
                print lang;
            }
        ";

        // NOTE: this test will never fail, change run_code to return a result and match
        // on that result. Look into the anyhow crate for this :)
        let _ = run_code(&mut vm, source);
}

#[test]
fn global_local_interaction_test() {
    let mut vm = VM::default();
        let source = "
            var foo = \"foo\";
            {
                var lang = \"yellowstone\";
                foo = foo + lang;
            }
            foo
        ";

        let _ = run_code(&mut vm, source);

        assert_eq!(str_val(&mut vm, "foo"), Some(String::from("fooyellowstone")));
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
