use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::op_code::{OpCode};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::scanner::TokenKind::*;

const DEBUG_PRINT_CODE: bool = false;

pub fn compile(source: &String) -> Result<Chunk, ()>{
    let mut current_chunk = Chunk::default();
    let mut scanner = Scanner::from(source);
    let mut parser = Parser::new(&mut current_chunk);
    parser.advance(&mut scanner); // Q; 'primes the pump' > ? do I need
    parser.expression(&mut scanner);
    parser.consume(TOKEN_EOF, "Expect end of expression", &mut scanner);
    parser.end_compiler();
    return match parser.had_error {
        true => {
            Ok(current_chunk)
        },
        false => {
            Err(())
        },
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT,  // =
    PREC_OR,          // or
    PREC_AND,         // and
    PREC_EQUALITY,    // == !=
    PREC_COMPARISON,  // < > <= >=
    PREC_TERM,        // + -
    PREC_FACTOR,      // * /
    PREC_UNARY,       // ! -
    PREC_CALL,        // . ()
    PREC_PRIMARY
}

struct ParseRule<'function, 'source, 'chunk> {
    prefix: Option<&'function dyn Fn(&mut Parser<'source, 'chunk>, &mut Scanner<'source>)>,
    infix: Option<&'function dyn Fn(&mut Parser<'source, 'chunk>, &mut Scanner<'source>)>,
    precedence: Precedence,
}


// Functions to match each expression type
fn grouping<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    parser.expression(scanner);
    parser.consume(TokenKind::TOKEN_RIGHT_PAREN, "Expect ')' after expression.", scanner);
}

fn number<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let value = parser.previous().slice.parse::<f32>().unwrap();
}

fn binary<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous().kind.clone();
    let rule = get_rule(operator_type);
    //parser.parse_precedence((Precedenc)(rule-> precedence + 1));

    match operator_type {
        TokenKind::TOKEN_PLUS => { parser.emit_byte(OpCode::OP_ADD) },
        TokenKind::TOKEN_MINUS => { parser.emit_byte(OpCode::OP_SUBTRACT) },
        TokenKind::TOKEN_STAR => { parser.emit_byte(OpCode::OP_MULTIPLY) },
        TokenKind::TOKEN_SLASH => { parser.emit_byte(OpCode::OP_DIVIDE) },
        _ => {}
    }
}

fn unary<'source, 'chunk>(parser: &mut Parser<'source, 'chunk>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous().kind.clone();

    // Compile the operand
    parser.parse_precedence(Precedence::PREC_UNARY, scanner);

    // Emit the operator instruction
    match operator_type {
        TokenKind::TOKEN_MINUS => { parser.emit_byte(OpCode::OP_NEGATE); }
        _ => {}
    }
}

struct Parser<'source, 'chunk> {
    current: Option<&'source Token<'source>>,
    previous: Option<&'source Token<'source>>,
    had_error: bool,
    panic_mode: bool,
    compiling_chunk: &'chunk mut Chunk,
}

impl<'source, 'chunk> Parser<'source, 'chunk> {
    fn new(compiling_chunk: &'chunk mut Chunk)  -> Self {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            compiling_chunk,
        }
    }

    fn previous(&self) -> &'source Token<'source> {
        self.previous.unwrap()
    }

    fn current(&self) -> &'source Token<'source> {
        self.current.unwrap()
    }

    // removed 'source form &mut Scanner
    fn advance(&mut self, scanner: &mut Scanner<'source>) {
        self.previous = self.current.take();

        loop {
            // TODO: add the advance back
            let token: Token<'source> = scanner.scan_token();
            if token.kind != TokenKind::TOKEN_ERROR {
                break;
            } else {
                self.error_at_current(token.slice);
            }
        }
    }

    fn error_at_current(&mut self, message: &'source str) {
        let token = self.current();
        self.error_at(&token, message);
    }

    fn error_at_prev(&mut self, message: &'source str) {
        let token = self.previous();
        self.error_at(&token, message);
    }

    fn error_at(&mut self, token: &Token, message: &'source str) {
        if self.panic_mode { return; }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.kind == TokenKind::TOKEN_EOF {
            eprintln!(" at end");
        } else {
            eprint!(" at '{}'", token.slice);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn consume(&mut self, kind: TokenKind, message: &'source str, scanner: &mut Scanner<'source>) {
        if self.current().kind == kind {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    fn expression(&mut self, scanner: &mut Scanner<'source>) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT, scanner);
    }

    fn parse_precedence(&mut self, precedence: Precedence, scanner: &mut Scanner<'source>) {
        let prev = self.previous().kind.clone();

        self.advance(scanner);
        let prefix_rule = get_rule(prev).prefix;
        if let Some(rule) = prefix_rule {
            rule(self, scanner);

            let current = self.current().kind.clone();
            while precedence <= get_rule(current).precedence {
                self.advance(scanner);

                let prev = self.previous().kind.clone();
                let infix_rule = get_rule(prev).infix;
                if let Some(rule) = infix_rule {
                    rule(self, scanner);
                }
            }
        } else {
            eprint!("Expect expression.");
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

    fn emit_constant(&mut self, value: f32) {
        // check to make sure I don't have the max
        // constants in a chunk
        let size = self.compiling_chunk.add_constant(value);
        if size > 256 {
            eprint!("Too many constants in one chunk.");
        }
        self.emit_byte(OpCode::OP_CONSTANT(value))
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        let line = self.previous().line as usize;
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
        TOKEN_LEFT_PAREN => { ParseRule {
            prefix: Some(&grouping),
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_RIGHT_PAREN => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_LEFT_BRACE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_RIGHT_BRACE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_COMMA => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_DOT => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_MINUS => { ParseRule {
            prefix: Some(&unary),
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM
        }}
        TOKEN_PLUS => { ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM
        }}
        TOKEN_SEMICOLON => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_SLASH => { ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR
        }}
        TOKEN_STAR => { ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR
        }}
        TOKEN_BANG => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_BANG_EQUAL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_EQUAL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_EQUAL_EQUAL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_GREATER => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_GREATER_EQUAL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_LESS => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_LESS_EQUAL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_IDENTIFIER => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_STRING => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_NUMBER => { ParseRule {
            prefix: Some(&number),
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_AND => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_CLASS => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_ELSE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_FALSE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_FOR => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_FUN => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_IF => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_NIL => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_OR => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_PRINT => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_RETURN => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_SUPER => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_THIS => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_TRUE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_VAR => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_WHILE => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_ERROR => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
        TOKEN_EOF => { ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE
        }}
    }
}
