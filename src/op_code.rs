#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT(f32),
    OP_RETURN,
    OP_DEBUG
}