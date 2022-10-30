use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::scanner::TokenKind::*;
use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::{allocate_object, Value};

const DEBUG_PRINT_CODE: bool = false;

pub fn compile(source: &String) -> Result<Chunk, ()> {
    let mut current_chunk = Chunk::default();
    let mut scanner = Scanner::from(source);
    let mut parser = Parser::new(&mut current_chunk);
    parser.advance(&mut scanner); // Q; 'primes the pump' > ? do I need
    parser.expression(&mut scanner);
    parser.consume(TOKEN_EOF, "Expect end of expression", &mut scanner);
    parser.end_compiler();
    return match parser.had_error {
        true => Err(()),
        false => Ok(current_chunk),
    };
}

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT, // =
    PREC_OR,         // or
    PREC_AND,        // and
    PREC_EQUALITY,   // == !=
    PREC_COMPARISON, // < > <= >=
    PREC_TERM,       // + -
    PREC_FACTOR,     // * /
    PREC_UNARY,      // ! -
    PREC_CALL,       // . ()
    PREC_PRIMARY,
}

impl Precedence {
    fn get_enum(prec: usize) -> Self {
        return match prec {
            0 => Precedence::PREC_NONE,
            1 => Precedence::PREC_ASSIGNMENT,
            2 => Precedence::PREC_OR,
            3 => Precedence::PREC_AND,
            4 => Precedence::PREC_EQUALITY,
            5 => Precedence::PREC_COMPARISON,
            6 => Precedence::PREC_TERM,
            7 => Precedence::PREC_FACTOR,
            8 => Precedence::PREC_UNARY,
            9 => Precedence::PREC_CALL,
            _ => Precedence::PREC_PRIMARY,
        };
    }
}

enum ErrorAt {
    Current,
    Before,
}

struct ParseRule<'function, 'source, 'chunk> {
    prefix: Option<&'function dyn Fn(&mut Parser<'source, 'chunk>, &mut Scanner<'source>)>,
    infix: Option<&'function dyn Fn(&mut Parser<'source, 'chunk>, &mut Scanner<'source>)>,
    precedence: Precedence,
}

// Functions to match each expression type
fn grouping<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    parser.expression(scanner);
    parser.consume(
        TokenKind::TOKEN_RIGHT_PAREN,
        "Expect ')' after expression.",
        scanner,
    );
}

fn number<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let value = parser
        .previous
        .as_ref()
        .unwrap()
        .slice
        .parse::<f32>()
        .unwrap();
    parser.emit_constant(Value::number_value(value));
}

fn string<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let slice = parser.previous.as_ref().unwrap().slice;
    let len = slice.len();
    let string = slice[1..len - 1].to_string();
    let string_obj = allocate_object(string);
    parser.emit_constant(string_obj);
}

fn binary<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous.as_ref().unwrap().kind.clone();
    let rule = get_rule(operator_type);
    let prec = rule.precedence as usize;
    parser.parse_precedence(Precedence::get_enum(prec), scanner);

    match operator_type {
        TokenKind::TOKEN_BANG_EQUAL => parser.emit_bytes(OpCode::OP_EQUAL, OpCode::OP_NOT),
        TokenKind::TOKEN_EQUAL_EQUAL => parser.emit_byte(OpCode::OP_EQUAL),
        TokenKind::TOKEN_GREATER => parser.emit_byte(OpCode::OP_GREATER),
        TokenKind::TOKEN_GREATER_EQUAL => parser.emit_bytes(OpCode::OP_LESS, OpCode::OP_NOT),
        TokenKind::TOKEN_LESS => parser.emit_byte(OpCode::OP_LESS),
        TokenKind::TOKEN_LESS_EQUAL => parser.emit_bytes(OpCode::OP_GREATER, OpCode::OP_NOT),
        TokenKind::TOKEN_PLUS => parser.emit_byte(OpCode::OP_ADD),
        TokenKind::TOKEN_MINUS => parser.emit_byte(OpCode::OP_SUBTRACT),
        TokenKind::TOKEN_STAR => parser.emit_byte(OpCode::OP_MULTIPLY),
        TokenKind::TOKEN_SLASH => parser.emit_byte(OpCode::OP_DIVIDE),
        _ => {}
    }
}

fn literal<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    match parser.previous.as_ref().unwrap().kind {
        TOKEN_TRUE => parser.emit_byte(OpCode::OP_TRUE),
        TOKEN_FALSE => parser.emit_byte(OpCode::OP_FALSE),
        TOKEN_NIL => parser.emit_byte(OpCode::OP_NIL),
        _ => {} // unreachable
    }
}

fn unary<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous.as_ref().unwrap().kind.clone();

    // Compile the operand
    parser.parse_precedence(Precedence::PREC_UNARY, scanner);

    // Emit the operator instruction
    match operator_type {
        TokenKind::TOKEN_MINUS => {
            parser.emit_byte(OpCode::OP_NEGATE);
        }
        TokenKind::TOKEN_BANG => {
            parser.emit_byte(OpCode::OP_NOT);
        }
        _ => {}
    }
}

struct Parser<'source, 'chunk> {
    current: Option<Token<'source>>,
    previous: Option<Token<'source>>,
    had_error: bool,
    panic_mode: bool,
    compiling_chunk: &'chunk mut Chunk,
}

impl<'source, 'chunk> Parser<'source, 'chunk> {
    fn new(compiling_chunk: &'chunk mut Chunk) -> Self {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            compiling_chunk,
        }
    }

    fn advance(&mut self, scanner: &mut Scanner<'source>) {
        self.previous = self.current.take();

        loop {
            let token = scanner.scan_token();

            if token.kind != TokenKind::TOKEN_NUMBER {
                scanner.advance();
            }

            self.current = Some(token);

            if self.current.as_ref().unwrap().kind != TokenKind::TOKEN_ERROR {
                break;
            } else {
                self.error_at(ErrorAt::Current);
            }
        }
    }

    fn error_at(&mut self, error_at: ErrorAt) {
        let token = match error_at {
            ErrorAt::Current => self.current.as_ref().unwrap(),
            ErrorAt::Before => self.previous.as_ref().unwrap(),
        };

        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.kind == TokenKind::TOKEN_EOF {
            eprintln!(" at end");
        } else {
            eprint!(" at '{}'", token.slice);
        }

        eprintln!(": {}", token.slice);
        self.had_error = true;
    }

    fn consume(&mut self, kind: TokenKind, message: &'source str, scanner: &mut Scanner<'source>) {
        match self.current.as_ref().unwrap().kind {
            kind => {
                self.advance(scanner);
                return;
            }
            _ => {
                self.error_at(ErrorAt::Current);
            }
        }
    }

    fn expression(&mut self, scanner: &mut Scanner<'source>) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT, scanner);
    }

    fn parse_precedence(&mut self, precedence: Precedence, scanner: &mut Scanner<'source>) {
        self.advance(scanner);
        let prev_kind = self.previous.as_ref().unwrap().kind.clone();

        let prefix_rule = get_rule(prev_kind).prefix;
        if let Some(rule) = prefix_rule {
            rule(self, scanner);

            while precedence <= get_rule(self.current.as_ref().unwrap().kind.clone()).precedence {
                self.advance(scanner);

                let prev = self.previous.as_ref().unwrap().kind.clone();
                let infix_rule = get_rule(prev).infix;
                if let Some(rule) = infix_rule {
                    rule(self, scanner);
                }
            }
        } else {
            eprint!("Expect expression.");
            return;
        }
    }

    fn end_compiler(&mut self) {
        if DEBUG_PRINT_CODE {
            if self.had_error {
                disassemble_chunk(&self.compiling_chunk, "code");
            }
        }
        self.emit_return()
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN);
    }

    fn emit_constant(&mut self, value: Value) {
        // check to make sure I don't have the max
        // constants in a chunk
        let constant_value = value.clone();
        let size = self.compiling_chunk.add_constant(constant_value);
        if size > 256 {
            eprint!("Too many constants in one chunk.");
        }
        self.emit_byte(OpCode::OP_CONSTANT(value))
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        let line = self.previous.as_ref().unwrap().line as usize;
        self.compiling_chunk.write_chunk(opcode, line);
    }

    fn emit_bytes(&mut self, opcode1: OpCode, opcode2: OpCode) {
        self.emit_byte(opcode1);
        self.emit_byte(opcode2);
    }
}

// NOTE: not calling the function here, instead
// returning the reference to the function to be called
// in some other scope
fn get_rule<'function, 'source, 'chunk>(kind: TokenKind) -> ParseRule<'function, 'source, 'chunk> {
    match kind {
        TOKEN_LEFT_PAREN => ParseRule {
            prefix: Some(&grouping),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RIGHT_PAREN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_LEFT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RIGHT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_COMMA => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_DOT => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_MINUS => ParseRule {
            prefix: Some(&unary),
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM,
        },
        TOKEN_PLUS => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM,
        },
        TOKEN_SEMICOLON => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_SLASH => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR,
        },
        TOKEN_STAR => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR,
        },
        TOKEN_BANG => ParseRule {
            prefix: Some(&unary),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_BANG_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_EQUALITY,
        },
        TOKEN_EQUAL => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_EQUAL_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_EQUALITY,
        },
        TOKEN_GREATER => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_GREATER_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_LESS => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_LESS_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_IDENTIFIER => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_STRING => ParseRule {
            prefix: Some(&string),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_NUMBER => ParseRule {
            prefix: Some(&number),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_AND => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_CLASS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_ELSE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FALSE => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FOR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FUN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_IF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_NIL => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_OR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_PRINT => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RETURN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_SUPER => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_THIS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_TRUE => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_VAR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_WHILE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_ERROR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_EOF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
    }
}
