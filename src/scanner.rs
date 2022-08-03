// Design Decision: Scan token only when the compiler needs a token

use std::fmt::{Display, Formatter};
use crate::scanner::TokenKind::*;
use crate::util::*;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    TOKEN_LEFT_PAREN,
    TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE,
    TOKEN_RIGHT_BRACE,
    TOKEN_COMMA,
    TOKEN_DOT,
    TOKEN_MINUS,
    TOKEN_PLUS,
    TOKEN_SEMICOLON,
    TOKEN_SLASH,
    TOKEN_STAR,
    // One or two character tokens.
    TOKEN_BANG,
    TOKEN_BANG_EQUAL,
    TOKEN_EQUAL,
    TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER,
    TOKEN_GREATER_EQUAL,
    TOKEN_LESS,
    TOKEN_LESS_EQUAL,
    // Literals.
    TOKEN_IDENTIFIER,
    TOKEN_STRING,
    TOKEN_NUMBER,
    // Keywords.
    TOKEN_AND,
    TOKEN_CLASS,
    TOKEN_ELSE,
    TOKEN_FALSE,
    TOKEN_FOR,
    TOKEN_FUN,
    TOKEN_IF,
    TOKEN_NIL,
    TOKEN_OR,
    TOKEN_PRINT,
    TOKEN_RETURN,
    TOKEN_SUPER,
    TOKEN_THIS,
    TOKEN_TRUE,
    TOKEN_VAR,
    TOKEN_WHILE,

    TOKEN_ERROR,
    TOKEN_EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub start: *const u8,
    pub length: usize,
    pub line: u8,
}

impl Token {
    pub fn new(kind: TokenKind, start: *const u8, length: usize, line: u8) -> Self {
        Token {
            kind,
            start,
            length,
            line,
        }
    }

    pub fn make_token(kind: TokenKind) -> Token {
        unimplemented!()
    }
}

// impl Token {
//     pub fn kind_as_str(&self) -> String {
//         self.kind
//     }
// }

pub struct Scanner {
    pub start: *const u8,
    pub current: *const u8,
    line: u8,
}

impl Scanner {
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return Token::make_token(TOKEN_EOF);
        }

        let c = self.advance() as char;
        if is_alpha(c) {
            return self.tokenize_string();
        }
        if is_digit(c) {
            return self.tokenize_number();
        }

        match c {
            // single character
            '(' => return self.make_token(TOKEN_LEFT_PAREN),
            ')' => return self.make_token(TOKEN_RIGHT_PAREN),
            '{' => return self.make_token(TOKEN_LEFT_BRACE),
            '}' => return self.make_token(TOKEN_RIGHT_BRACE),
            ';' => return self.make_token(TOKEN_SEMICOLON),
            ',' => return self.make_token(TOKEN_COMMA),
            '.' => return self.make_token(TOKEN_DOT),
            '-' => return self.make_token(TOKEN_MINUS),
            '+' => return self.make_token(TOKEN_PLUS),
            '/' => return self.make_token(TOKEN_SLASH),
            '*' => return self.make_token(TOKEN_STAR),
            // optional two character
            '!' => {
                if self.expect('=') {
                    self.advance();
                    return self.make_token(TOKEN_BANG_EQUAL);
                } else {
                    return self.make_token(TOKEN_BANG);
                }
            }
            '=' => {
                if self.expect('=') {
                    self.advance();
                    return self.make_token(TOKEN_EQUAL_EQUAL);
                } else {
                    return self.make_token(TOKEN_EQUAL);
                }
            }
            '<' => {
                if self.expect('=') {
                    self.advance();
                    return self.make_token(TOKEN_LESS_EQUAL);
                } else {
                    return self.make_token(TOKEN_LESS);
                }
            }
            '>' => {
                if self.expect('=') {
                    self.advance();
                    return self.make_token(TOKEN_GREATER_EQUAL);
                } else {
                    return self.make_token(TOKEN_GREATER);
                }
            }
            '"' => {
                return self.tokenize_string();
            }
            _ => {}
        }
        //todo: evaluate syntax
        return self.error_token("Unexpected character.");
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let start = self.start;
        let length = bytes_between(self.current, self.start) as usize; //number of bytes between both memory locations
        let line = self.line;
        let token = Token::new(kind, start, length, line);
        return token;
    }

    fn error_token(&self, message: &str) -> Token {
        let kind = TOKEN_ERROR;
        let start = message.as_bytes().first().unwrap();
        let length = message.len();
        let line = self.line;
        let token = Token::new(kind, start, length, line);
        return token;
    }

    fn is_at_end(&self) -> bool {
        unsafe {
            return *self.current as char == '\0';
        }
    }

    // todo: debug
    fn advance(&self) -> *const u8 {
        unsafe {
            return self.current.add(1);
        }
    }

    fn peek(&self) -> char {
        unsafe {
            return *self.current as char;
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        unsafe {
            return Some(*self.current + 1 as char)
        }
    }

    fn expect(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        unsafe {
            if self.peek() != expected {
                return false;
            }
        }
        return true;
    }

    //todo: debug
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                    ;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    return;
                    ;
                }
                '/' => {
                    if self.expect('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    fn tokenize_string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote
        self.advance();
        return self.make_token(TOKEN_STRING);
    }

    fn tokenize_number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        return self.make_token(TOKEN_NUMBER);
    }

    fn tokenize_identifier(&self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        return self.make_token(identifier_type())
    }
}

pub fn identifier_type() -> TokenKind {
    return TOKEN_IDENTIFIER;
}

impl From<&String> for Scanner {
    fn from(source: &String) -> Self {
        let bytes = source.as_bytes();
        let start = bytes.first().unwrap();
        let current = bytes.first().unwrap();
        Scanner {
            start,
            current,
            line: 1,
        }
    }
}