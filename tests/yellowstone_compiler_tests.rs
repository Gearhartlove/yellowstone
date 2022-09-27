use std::fmt::Display;
use yellowstone::vm::VM;

pub fn run_code<T: ToString + Display>(mut vm: &mut VM, code: T) -> Option<f32> {
    // println!("{code}");
    let result = vm.interpret(&code.to_string());

    match result {
        Err(e) => { println!("{:?}", e); return None },
        Ok(_) => { return result.unwrap() },
    }
}

#[test]
fn compiler_unary() {
    let mut vm = VM::default();
    let source = "-1";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(-1.));
}

#[test]
fn compiler_binary() {
    let mut vm = VM::default();
    let source = "1 + 1";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(2.));

    let source = "1 - 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(-1.));

    let source = "3 / 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(1.5));

    let source = "2 * 2";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(4.));
}

#[test]
fn compiler_grouping() {
    let mut vm = VM::default();
    let source = "(1)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(1.));
}

#[test]
fn compiler_number() {
    let mut vm = VM::default();
    let source = "2022";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(2022.));
}

#[test]
fn compiler_precedence() {
    let mut vm = VM::default();
    let source = "2 * (1 + 1)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(4.));


    let mut vm = VM::default();
    let source = "(2 * -1) + 4 / 4";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(-1.));

    let mut vm = VM::default();
    let source = "2 * ((-1 + 4 / 4) - 2)";
    let result = run_code(&mut vm, source);
    assert_eq!(result, Some(-4.));
}
