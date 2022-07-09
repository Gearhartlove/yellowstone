use std::convert::Infallible;
use crate::value::Values;

#[derive(Debug)]
pub enum OpCode {
    OP_RETURN,
    OP_DEBUG
}

// impl TryFrom<u8> for OpCode {
//     type Error = &'static str;
//
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0..=1 => {
//                 Ok( unsafe { std::mem::transmute(value)} )
//             }
//             _ => {
//                 Err("Invalid Instruction")
//             }
//         }
//     }
// }

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub values: Values,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            code: Vec::default(),
            values: Values::default(),
        }
    }
}

