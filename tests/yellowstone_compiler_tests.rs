extern crate core;

use anyhow::Result;
use std::fmt::Display;
use yellowstone::error::InterpretError::{self, *};
use yellowstone::value::{Value, ValueKind};
use yellowstone::vm::VM;

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
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_asserteq_fail_test() {
    let mut vm = VM::default();
    let source = "var foo = true; assert_eq(foo, false);";
    run_code_expect_error(&mut vm, source, RUNTIME_ASSERT_ERROR);
}

#[test]
fn compiler_asserteq_num_test() {
    let mut vm = VM::default();
    let source = "var foo = 42; assert_eq(foo, 42);";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_asserteq_nil_test() {
    let mut vm = VM::default();
    let source = "var foo = nil; assert_eq(foo, nil);";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_asserteq_string_test() {
    let mut vm = VM::default();
    let source = "var foo = \"foo\"; assert_eq(foo, \"foo\");";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_global_var_test() {
    let mut vm = VM::default();
    let source = "var lang = \"yellowstone\";";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_global_var_declaration() {
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
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_mutate_global_vars_test() {
    let mut vm = VM::default();
    let source = "
        var beverage = \"cafe au lait\"; 
        var breakfast = \"beignets with \" + beverage;
        assert_eq(breakfast, \"beignets with cafe au lait\");";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_local_var_declaration_test() {
    let mut vm = VM::default();
    let source = "
        {
            var lang = \"yellowstone\";
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_get_var_declaration_test() {
    let mut vm = VM::default();
    let source = "
        {
            var lang = \"yellowstone\";
            assert_eq(lang, \"yellowstone\");
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_get_local_and_global_vars_test() {
    let mut vm = VM::default();
    let source = "
        var start = \"yellow\";
        {
            var end = \"stone\";

            assert_eq(end, \"stone\");
            assert_eq(start, \"yellow\");
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_global_local_interaction_test() {
    let mut vm = VM::default();
    let source = "
        var foo = \"yellow\";
        {
            var bar = \"stone\";
            foo = foo + bar;
        }
        assert_eq(foo, \"yellowstone\");
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_global_local_nums_interaction_test() {
    let mut vm = VM::default();
    let source = "
        var foo = 9;
        {
            var bar = 1;
            foo = foo + bar;
        }
        assert_eq(foo, 10);
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_multiple_blocks_global_interaction_test() {
    let mut vm = VM::default();
    let source = "
        var foo = 0;
        {
            foo = foo + 1;
        }
        {
            var bar = 2;
            foo = foo - bar;
        }
        assert_eq(foo, -1);
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_multiple_blocks_test() {
    let mut vm = VM::default();
    let source = "
        {
            var foo = \"Hello World!\";
            assert_eq(foo, \"Hello World!\");
        }

        {
            var bar = 10;
            assert_eq(bar, 10);
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_variable_dropping_block_test() {
    let mut vm = VM::default();
    let source = "
        {
            var foo = \"Hello World!\";
        }

        {
            print foo;
        }
    ";
    run_code_expect_error(&mut vm, source, RUNTIME_UNRECOGNIZED_VARIABLE_ERROR);
}

#[test]
fn compiler_variable_dropping_global_test() {
    let mut vm = VM::default();
    let source = "
        {
            var foo = \"Hello World!\";
        }
        print foo;
    ";
    run_code_expect_error(&mut vm, source, RUNTIME_UNRECOGNIZED_VARIABLE_ERROR);
}

#[test]
fn compiler_variable_local_shadowing_test() {
    let mut vm = VM::default();
    let source = "
        {
            var foo = \"Hello World!\";
            var foo = \"Yellowstone\";
            assert_eq(foo, \"Yellowstone\");
        }
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_variable_local_global_shadowing_test() {
    let mut vm = VM::default();
    let source = "
        var foo = \"first\";
        {
            var foo = \"second\";
            assert_eq(foo, \"second\");
        }
        assert_eq(foo, \"first\");
    ";
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_if_true_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1; 
        if (true) { 
            num = 2 
        } 
        assert_eq(2, num);",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_if_false_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1; 
        if (false) { 
            num = 2 
        } 
        assert_eq(1, num);",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_else_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1; 
        if (false) {  
            num = 2; 
        } else {
            assert_eq(1, num);
            num = 3
        }
        assert_eq(3, num);",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_condition_and_true_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1;
        if (true and true) {
            num = 2;
        }
        assert_eq(2, num);
    ",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_condition_and_false_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1;
        if (false and true) {
            num = 2;
        }
        assert_eq(1, num);
    ",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_condition_or_true_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1;
        if (true or false) {
            num = 2;
        }
        assert_eq(2, num);
    ",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_condition_or_false_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1;
        if (false or false) {
            num = 2;
        }
        assert_eq(1, num);
    ",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

#[test]
fn compiler_while_test() {
    let mut vm = VM::default();
    let source = String::from(
        "
        var num = 1;
        while (num != 3) {
            num = num + 1;
        }
        assert_eq(3, num);
    ",
    );
    let result = run_code(&mut vm, source);
    if result.is_err() {
        eprintln!("{result:?}");
        panic!()
    }
}

// #[test]
// fn compiler_for_test() {
//     let mut vm = VM::default();
//     let source = String::from(
//         "
//         var loops = 1;
//         for (var i = 0; i < 2; i += 1) {
//             loops += 1;
//         }
//         assert_eq(2, loops);
//     ",
//     );
//     let result = run_code(&mut vm, source);
//     if result.is_err() {
//         eprintln!("{result:?}");
//         panic!()
//     }
// }

// ################################################################################
// Helper Functions
// ################################################################################

pub fn run_code<T: ToString + Display>(vm: &mut VM, source: T) -> Result<Option<Value>> {
    vm.interpret(&source.to_string())
}

pub fn run_code_expect_value<T: ToString + Display>(vm: &mut VM, source: T, expect: Option<Value>) {
    let result = run_code(vm, source);
    match result {
        Ok(v) => {
            assert_eq!(expect, v);
        }
        _ => {
            eprintln!("error returned when value expected");
            panic!()
        }
    }
}

pub fn run_code_expect_error<T: ToString + Display>(
    vm: &mut VM,
    source: T,
    expect: InterpretError,
) {
    let result = run_code(vm, source);
    match result {
        Err(e) => {
            let root = e.root_cause();
            assert_eq!(format!("{root}"), expect.to_string());
        }
        _ => {
            eprintln!("value returned when error expected");
            panic!()
        }
    }
}

pub fn num_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValNumber => Some(value.as_number().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn nil_val(vm: &mut VM, variable_name: &'static str) -> Option<f32> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValNil => Some(value.as_nil().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn bool_val(vm: &mut VM, variable_name: &'static str) -> Option<bool> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValBool => Some(value.as_bool().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn str_val(vm: &mut VM, variable_name: &'static str) -> Option<String> {
    if let Some(value) = vm.table.get(variable_name) {
        match value.kind {
            ValueKind::ValObj => Some(value.as_string().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_constant_name(vm: &VM, i: usize) -> Option<String> {
    return vm.chunk.constants.get(i).map(|c| c.as_string().unwrap());
}
