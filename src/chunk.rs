use crate::value::Value;

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
}

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
    pub fn write_chunk(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        //encode(self, line); // todo: fix encoding for debug output
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);

        // return the index where the constant was appended so that we can
        // locate that constant later
        let index = self.constants.len() - 1;
        return index;
    }
}

/// Each line number is separated by a '\_', the numbers in between the '\_' are the number of
/// operations on that line.
/// Look at tests for example.
fn encode(chunk: &mut Chunk, curr_line: usize) {
    // determine if I am on a new line by comparing the length of the lines with the current line number
    let line_count = chunk
        .lines
        .chars()
        .filter(|x| x == &'_')
        .map(|x| x)
        .collect::<Vec<char>>()
        .len();
    let same_line: bool = line_count == curr_line;
    if !same_line {
        // pop leading '_'
        chunk.lines.pop();
        // get the number by popping then reversing
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
        // pushing the number to lines
        chunk.lines.push_str(&num);
        // pushing the '_' back to lines
        chunk.lines.push('_');
    } else {
        // push "1_"
        chunk.lines.push_str("1_");
    }
}

// todo: calculate the line number of the offset before for comparison
pub fn get_line(offset: &mut u32, lines: &String) -> String {
    if *offset == 0 {
        return "1".to_string();
    }

    let line_numb = |offset: &mut u32| -> String {
        let mut split = lines.split('_').collect::<Vec<&str>>();
        split.pop(); // remove ending ""
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

// #################################################################################################
// Testing

#[allow(unused_imports)]
mod tests {
    use crate::chunk::*;
    use crate::chunk::*;

    #[test]
    fn encode_test() {
        let chunk = Chunk::default()
            .write_chunk(OpCode::OP_CONSTANT(0.), 0)
            .write_chunk(OpCode::OP_CONSTANT(1.), 0)
            .write_chunk(OpCode::OP_CONSTANT(2.), 1)
            .write_chunk(OpCode::OP_RETURN, 1);
        assert_eq!("2_2_", &chunk.lines);
    }

    #[test]
    fn get_line_test() {
        let chunk = Chunk::default()
            .write_chunk(OpCode::OP_CONSTANT(0.), 0) // 1
            .write_chunk(OpCode::OP_CONSTANT(1.), 0) // same
            .write_chunk(OpCode::OP_CONSTANT(2.), 1) // 2
            .write_chunk(OpCode::OP_CONSTANT(2.), 1) // same
            .write_chunk(OpCode::OP_CONSTANT(2.), 1) // same
            .write_chunk(OpCode::OP_RETURN, 2); // 3

        assert_eq!("1", get_line(&mut 0, &chunk.lines));
        assert_eq!("same", get_line(&mut 1, &chunk.lines));
        assert_eq!("2", get_line(&mut 2, &chunk.lines));
        assert_eq!("same", get_line(&mut 3, &chunk.lines));
        assert_eq!("same", get_line(&mut 4, &chunk.lines));
        assert_eq!("3", get_line(&mut 5, &chunk.lines));
    }
}
