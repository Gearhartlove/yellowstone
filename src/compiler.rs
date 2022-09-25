use std::collections::HashMap;
use crate::chunk::Chunk;
use crate::op_code::{OpCode};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::scanner::TokenKind::*;


pub fn compile(source: &String, current_chunk: &mut Chunk) -> bool {
    let scanner = Scanner::from(source);
    let mut parser = Parser::new(scanner);
    parser.advance(&mut scanner); // 'primes the pump' > ? do I need
    expression();
    parser.consume(TOKEN_EOF, "Expect end of expression", &mut scanner);   
    end_compiler(current_chunk);
    return parser.had_error;
}

#[allow(non_camel_case_types)]
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

struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

// NOTE: the following lifetimes do not look right
struct Parser<'a> {
    // scanner: Scanner<'a>,
    current: &'a Token<'a>,
    previous: &'a Token<'a>,
    had_error: bool,
    panic_mode: bool,
    rules: HashMap<TokenKind, (&'a dyn Fn(&mut Parser), &'a dyn Fn(&mut Parser), Precedence)>,
}

impl Parser {
    fn new(scanner: &mut Scanner) -> Self {
        // create the

        let starting_token = scanner.scan_token();
        let mut parser = Parser {
            current: &starting_token,
            previous: &starting_token,
            had_error: false,
            panic_mode: false,
            rules: Default::default()
        };

        parser.rules = HashMap::new()
            .insert(TokenKind::TOKEN_MINUS, (&parser.unary, &parser.binary, Precedence::PREC_TERM));

    }

    fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current;
        loop {
            self.current = &scanner.scan_token();
            if self.current.kind != TokenKind::TOKEN_ERROR {
                break;
            }

            self.error_at_current(self.current.start);
        }
    }

    fn error_at_current(&mut self, message: &'static str) {
        self.error_at(&mut self, &self.current, message);
    }

    fn error_at_prev(&mut self, message: &'static str) {
        self.error_at(&mut self, &self.previous, message);
    }

    fn error_at(&mut self, token: &Token, message: &'static str) {
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

    fn consume(&mut self, kind: TokenKind, message: &'static str, scanner: &mut Scanner) {
        if self.current.kind == kind {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT);
    }

    fn number(&self) {
        let value = self.previous.slice.parse::<f32>().unwrap();
    }

    fn unary(&mut self) {
        let operator_type = self.previous.kind.clone();

        // Compile the operand
        parse_precedence(Precedence::PREC_UNARY);

        // Emit the operator instruction
        match operator_type {
            TokenKind::TOKEN_MINUS => { self.emitByte(OpCode::OP_NEGATE); }
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {

    }

    fn end_compiler(chunk: &mut Chunk) {
        emit_return(chunk)
    }

    fn binary(self) {
        let operator_type = self.previous.kind.clone();
        let rule = getRule(operator_type);
        //self.parse_precedence((Precedenc)(rule-> precedence + 1));

        match operator_type {
            TokenKind::TOKEN_PLUS => { self.emit_byte(OpCode::OP_ADD) },
            TokenKind::TOKEN_MINUS => { self.emit_byte(OpCode::OP_SUBTRACT) },
            TokenKind::TOKEN_STAR => { self.emit_byte(OpCode::OP_MULTIPLY) },
            TokenKind::TOKEN_SLASH => { self.emit_byte(OpCode::OP_DIVIDE) },
            _ => {}
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
    }

    fn emit_return(chunk: &mut Chunk) {
        emit_byte(OpCode::OP_RETURN, chunk);
    }

    fn emit_constant(value: f32, chunk: &mut Chunk) {
        // check to make sure I don't have the max
        // constants in a chunk
        let size = chunk.add_constant(value);
        if size > 256 {
            eprint!("Too many constants in one chunk.");
        }
        emit_byte(OpCode::OP_CONSTANT(value), chunk)
    }

    fn emit_byte(opcode: OpCode, chunk: &mut Chunk) {
        //write chunk
    }

    fn emit_bytes(opcode1: OpCode, opcode2: OpCode, chunk: &mut Chunk) {
        emit_byte(opcode1, chunk);
        emit_byte(opcode2, chunk);
    }

}
