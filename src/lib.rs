extern crate core;

use std::borrow::BorrowMut;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::op_code::OpCode::*;

pub mod scanner;
pub mod test_macros;
pub mod chunk;
pub mod common;
pub mod debug;
pub mod op_code;
pub mod vm;
pub mod stack;
pub mod util;