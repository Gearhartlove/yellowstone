#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT(f32),
    OP_CONSTANT_LONG(f64),
    OP_RETURN,
    OP_DEBUG,
    OP_NEGATE,
    //binary
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    //
}
