extern crate core;

use std::borrow::BorrowMut;
use std::{env, mem};
use std::mem::{size_of, size_of_val};
use std::process::exit;
use std::fs;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::InterpretResult::{INTERPRET_COMPILE_ERROR, INTERPRET_RUNTIME_ERROR};
use crate::op_code::OpCode::*;
use crate::stack::Stack;
use crate::vm::{InterpretResult, VM};

pub mod scanner;
pub mod test_macros;
pub mod chunk;
pub mod common;
pub mod memory;
pub mod debug;
pub mod value;
pub mod op_code;
pub mod vm;
pub mod stack;
pub mod util;

// pub fn create_token_vec(source: String) -> Vec<TokenKind> {
//     let scanner = Scanner::from(&source);
// }

// fn get_tokens_as_vec() {}
