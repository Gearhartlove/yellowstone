use std::convert::Infallible;
use crate::op_code::OpCode;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<f32>,
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
    pub fn write_chunk(mut self, op: OpCode, line: usize) -> Self {
        self.code.push(op);
        encode(&mut self, line);
        return self;
    }

    pub fn add_constant(&mut self, constant: f32) -> usize {
        self.constants.push(constant);

        // return the index where the constant was appended so that we can
        // locate that constant later
        let index = self.constants.len() - 1;
        return index;
    }
}

/// Each number represents a line
fn encode(chunk: &mut Chunk, curr_line: usize) {
    // determine if I am on a new line by comparing the length of the lines with the current line number
    let line_count = chunk.lines.chars().filter(|x| x == &'_').map(|x| x).collect::<Vec<char>>().len();
    let same_line: bool = line_count == curr_line;
    if !same_line {
        // pop leading '_'
        chunk.lines.pop();
        // get the number by popping then reversing
        let mut num = "".to_string();
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
        // pushing the number to lines
        chunk.lines.push_str(&num);
        // pushing the '_' back to lines
        chunk.lines.push('_');
    } else {
        // push "1_"
        chunk.lines.push_str("1_");
    }
}


pub fn get_line(instr_num: usize, lines: &String) -> String {
    if instr_num > 0 {
        let before_instr_num = instr_num - 1;
        let split = lines.split('_');
        let mut line_number = 1;
        let mut sum = 0;
        for num in split {
            let num = num.parse::<usize>().unwrap();
            sum += num;
            if instr_num <= sum {
                return line_number.to_string();
            }
            line_number += 1;
        }
    }
    return "1".to_string();
}

// #################################################################################################
// Testing

mod tests {
    use crate::Chunk;
    use crate::op_code::OpCode;

    #[test]
    fn encode_test() {
        let chunk = Chunk::default()
            .write_chunk(OpCode::OP_CONSTANT(0.), 0)
            .write_chunk(OpCode::OP_CONSTANT(1.), 0)
            .write_chunk(OpCode::OP_CONSTANT(2.), 1)
            .write_chunk(OpCode::OP_CONSTANT(2.), 1)
            .write_chunk(OpCode::OP_CONSTANT(2.), 1)
            .write_chunk(OpCode::OP_CONSTANT(2.), 1)
            .write_chunk(OpCode::OP_RETURN, 1);
        assert_eq!("2_5_", &chunk.lines);
    }
}