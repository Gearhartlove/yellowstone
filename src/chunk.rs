use crate::value::Value;

/// Yellowstone VM byte-code instructions.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT(Value),
    OP_NIL,
    OP_TRUE,
    OP_FALSE,
    OP_EQUAL,
    OP_GREATER,
    OP_LESS,
    OP_RETURN,
    OP_DEBUG,
    OP_NEGATE,
    OP_NOT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_PRINT,
}

/// Contains the bytecode instructions as well as constants created from parsing tokens.
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: String,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            code: Vec::default(),
            constants: Vec::default(),
            lines: "".to_string(),
        }
    }
}

impl Chunk {
    /// Adds an opcode to a chunk.
    pub fn write_chunk(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        encode(self, line);
    }

    /// Adds a constant to the chunk. Returns the index of that constant to locate later.
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        let index = self.constants.len() - 1;
        return index;
    }
}

/// Each line number is separated by a '\_', the numbers in between the '\_' are the number of
/// operations on that line.
/// Look at 'chunk_tests' module examples.
fn encode(chunk: &mut Chunk, curr_line: usize) {
    fn new_line(chunk: &mut Chunk, curr_line: usize) {
        chunk.lines.push_str("1_");
    }

    // Calculate the current line count.
    let line_count = chunk
        .lines
        .chars()
        .filter(|x| x == &'_')
        .collect::<Vec<char>>()
        .len();

    if line_count == 0 {
        new_line(chunk, curr_line);
    }
    else {
        let same_line: bool = line_count == curr_line;
        if !same_line {
            new_line(chunk, curr_line);
        }
        else if same_line {
            chunk.lines.pop();
            let mut num = "".to_string();
            println!("{}", num);
            loop {
                match chunk.lines.pop() {
                    None => {
                        break;
                    }
                    Some(c) => {
                        if c == '_' {
                            chunk.lines.push('_');
                            break;
                        }
                        num.push(c)
                    }
                }
            }
            let mut num = num.parse::<usize>().unwrap();
            num += 1;
            let num = num.to_string();
            chunk.lines.push_str(&num);
            chunk.lines.push('_');
        }
    }
}

/// Gets the line of a given an instruction 'offset'.
pub fn get_line(offset: &mut u32, lines: &String) -> String {
    if *offset == 0 {
        return "1".to_string();
    }

    let line_numb = |offset: &mut u32| -> String {
        let mut split = lines.split('_').collect::<Vec<&str>>();
        split.pop();
        let split = split
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let split_len = split.len();
        let mut sum: u32 = 0;
        let mut line_numb = 0;
        for num in split {
            line_numb += 1;
            sum += num;
            if *offset < sum {
                return line_numb.to_string();
            }
        }
        return split_len.to_string();
    };

    let before = line_numb(&mut (*offset - 1));
    let current = line_numb(offset);

    return match before == current {
        true => "same".to_string(),
        false => current,
    };
}

// Testing that the chunk.lines formatting is correct and ensuring each instruction is associated
// with the correct line.
mod chunk_tests {
    use crate::chunk::*;

    fn write_chunk(chunk: &mut Chunk, instruction: OpCode, line: usize) {
        if line <= 0 {
            panic!("Line cannot be smaller than 1")
        }

        chunk.write_chunk(instruction, line)
    }

    #[test]
    fn single_line_encode_test() {
        let chunk = &mut Chunk::default();
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(0.)), 1);
        write_chunk(chunk, OpCode::OP_RETURN, 1);
        assert_eq!("2_", &chunk.lines);
    }

    #[test]
    fn encode_test() {
        let chunk = &mut Chunk::default();
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(0.)), 1);
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(1.)), 1);
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(2.)), 2);
        write_chunk(chunk, OpCode::OP_RETURN, 2);
        assert_eq!("2_2_", &chunk.lines);
    }

    #[test]
    fn get_line_test() {
        let chunk = &mut Chunk::default();
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(0.)), 1); // 1
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(1.)), 1); // same
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(2.)), 2); // 2
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(2.)), 2); // same
        write_chunk(chunk, OpCode::OP_CONSTANT(Value::number_value(2.)), 2); // same
        write_chunk(chunk, OpCode::OP_RETURN, 3); // 3

        assert_eq!("1", get_line(&mut 0, &chunk.lines));
        assert_eq!("same", get_line(&mut 1, &chunk.lines));
        assert_eq!("2", get_line(&mut 2, &chunk.lines));
        assert_eq!("same", get_line(&mut 3, &chunk.lines));
        assert_eq!("same", get_line(&mut 4, &chunk.lines));
        assert_eq!("3", get_line(&mut 5, &chunk.lines));
    }
}
