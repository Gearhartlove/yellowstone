#![allow(warnings)]
// Design Decision: Scan token only when the compiler needs a token

use crate::scanner::TokenKind::*;
use crate::util::*;
use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    TOKEN_ASSERT_EQ,
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

#[derive(Debug)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub slice: &'source str,
    pub line: u8,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, slice: &'source str, line: u8) -> Self {
        Token { kind, slice, line }
    }
}

pub struct Scanner<'source> {
    pub source: &'source String,
    pub start: usize,
    pub current: usize,
    pub source_length: usize,
    pub line: u8,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            source_length: source.len(),
            line: 1,
        }
    }

    fn start(&self) -> &'source str {
        &self.source[self.start..self.start + 1]
    }

    fn start_next(&self) -> &'source str {
        &self.source[self.start + 1..self.start + 2]
    }

    fn current(&self) -> &'source str {
        &self.source[self.current..self.current + 1]
    }

    pub fn scan_token(&mut self) -> Token<'source> {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TOKEN_EOF);
        }

        let c = self.start();

        if is_alpha(c) {
            return self.tokenize_identifier();
        }
        if is_digit(c) {
            return self.tokenize_number();
        }

        match c {
            // single character
            "(" => return self.make_token(TOKEN_LEFT_PAREN),
            ")" => return self.make_token(TOKEN_RIGHT_PAREN),
            "{" => return self.make_token(TOKEN_LEFT_BRACE),
            "}" => return self.make_token(TOKEN_RIGHT_BRACE),
            ";" => return self.make_token(TOKEN_SEMICOLON),
            "," => return self.make_token(TOKEN_COMMA),
            "." => return self.make_token(TOKEN_DOT),
            "-" => return self.make_token(TOKEN_MINUS),
            "+" => return self.make_token(TOKEN_PLUS),
            "/" => return self.make_token(TOKEN_SLASH),
            "*" => return self.make_token(TOKEN_STAR),
            // optional two character
            "!" => {
                if self.expect("=") {
                    self.advance();
                    return self.make_token(TOKEN_BANG_EQUAL);
                } else {
                    return self.make_token(TOKEN_BANG);
                }
            }
            "=" => {
                if self.expect("=") {
                    self.advance();
                    return self.make_token(TOKEN_EQUAL_EQUAL);
                } else {
                    return self.make_token(TOKEN_EQUAL);
                }
            }
            "<" => {
                if self.expect("=") {
                    self.advance();
                    return self.make_token(TOKEN_LESS_EQUAL);
                } else {
                    return self.make_token(TOKEN_LESS);
                }
            }
            ">" => {
                if self.expect("=") {
                    self.advance();
                    return self.make_token(TOKEN_GREATER_EQUAL);
                } else {
                    return self.make_token(TOKEN_GREATER);
                }
            }
            "\"" => {
                self.advance(); // advance past the \"
                return self.tokenize_string();
            }
            _ => {}
        }
        self.error_token("Unexpected character.")
    }

    fn make_token(&self, kind: TokenKind) -> Token<'source> {
        if kind == TokenKind::TOKEN_EOF {
            return Token::new(kind, "EOF", self.line);
        }
        let line = self.line;
        let slice = &self.source[self.start..self.current];
        Token::new(kind, slice, line)
    }

    fn error_token(&self, message: &'source str) -> Token<'source> {
        let kind = TOKEN_ERROR;
        let line = self.line;
        Token::new(kind, message, line)
    }

    // debug: check for off by one
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    pub fn is_at_peek_next_end(&self) -> bool {
        self.current + 1 >= self.source_length
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn peek(&self) -> Option<&'source str> {
        if self.is_at_end() {
            return None;
        }
        Some(&self.source[self.current..self.current + 1])
    }

    pub fn peek_next(&self) -> Option<&'source str> {
        if self.is_at_peek_next_end() {
            return None;
        }
        Some(&self.source[self.current + 1..self.current + 2])
    }

    pub fn expect(&self, expected: &'source str) -> bool {
        if let Some(peek) = self.peek_next() {
            if peek != expected {
                return false;
            }
        }
        true
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                " " | "\r" | "\t" => {
                    self.advance();
                }
                "\n" => {
                    self.line += 1;
                    self.advance();
                }
                "/" => {
                    if self.expect("/") {
                        while let Some(peek) = self.peek() {
                            self.advance();
                            if peek == "\n" {
                                self.line += 1;
                                break;
                            };
                        }
                    } else {
                        return;
                    }
                    break;
                }
                _ => return,
            }
        }
    }

    pub fn tokenize_string(&mut self) -> Token<'source> {
        while let Some(c) = self.peek() {
            if c == "\"" {
                break;
            }
            if c == "\n" {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote
        self.advance();
        self.make_token(TOKEN_STRING)
    }

    pub fn tokenize_number(&mut self) -> Token<'source> {
        // keep consuming numbers
        while let Some(peek) = self.peek() {
            if !is_digit(peek) {
                break;
            }
            self.advance();
        }

        // Look for a fractional part
        if let Some(peek) = self.peek() {
            if let Some(peek_next) = self.peek_next() {
                if peek == "." && is_digit(peek_next) {
                    self.advance();

                    // keep consuming numbers
                    while let Some(peek) = self.peek() {
                        if !is_digit(peek) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }
        self.make_token(TOKEN_NUMBER)
    }

    pub fn tokenize_identifier(&mut self) -> Token<'source> {
        while let Some(peek) = self.peek() {
            if is_alpha(peek) || is_digit(peek) {
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(self.identifier_type())
    }

    pub fn identifier_type(&self) -> TokenKind {
        let c = self.start();

        match c {
            "a" => {
                if self.current - self.start > 1 {
                    match self.start_next() {
                        "n" => return self.check_keyword(2, 1, "d", TOKEN_AND),
                        "s" => return self.check_keyword(2, 7, "sert_eq", TOKEN_ASSERT_EQ),
                        _ => {}
                    }
                }
            }
            "c" => return self.check_keyword(1, 4, "lass", TOKEN_CLASS),
            "e" => return self.check_keyword(1, 3, "lse", TOKEN_ELSE),
            "f" => {
                if self.current - self.start > 1 {
                    match self.start_next() {
                        "a" => return self.check_keyword(2, 3, "lse", TOKEN_FALSE),
                        "o" => return self.check_keyword(2, 1, "r", TOKEN_FOR),
                        "u" => return self.check_keyword(2, 1, "n", TOKEN_FUN),
                        _ => {}
                    }
                }
            }
            "i" => return self.check_keyword(1, 1, "f", TOKEN_IF),
            "n" => return self.check_keyword(1, 2, "il", TOKEN_NIL),
            "o" => return self.check_keyword(1, 1, "r", TOKEN_OR),
            "p" => return self.check_keyword(1, 4, "rint", TOKEN_PRINT),
            "r" => return self.check_keyword(1, 5, "eturn", TOKEN_RETURN),
            "s" => return self.check_keyword(1, 4, "uper", TOKEN_SUPER),
            "t" => {
                if self.current - self.start > 1 {
                    match self.start_next() {
                        "h" => return self.check_keyword(2, 2, "is", TOKEN_THIS),
                        "r" => return self.check_keyword(2, 2, "ue", TOKEN_TRUE),
                        _ => {}
                    }
                }
            }
            "v" => return self.check_keyword(1, 2, "ar", TOKEN_VAR),
            "w" => return self.check_keyword(1, 4, "hile", TOKEN_WHILE),
            _ => {}
        }

        TOKEN_IDENTIFIER
    }

    pub fn check_keyword(
        &self,
        start: usize,
        end: usize,
        the_rest: &str,
        kind: TokenKind,
    ) -> TokenKind {
        if self.current - self.start == start + end
            && the_rest == &self.source[self.start + start..(self.start + start + end)]
        {
            return kind;
        }
        TOKEN_IDENTIFIER
    }
}

impl<'source> From<&'source String> for Scanner<'source> {
    fn from(source: &'source String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            source_length: source.len(),
        }
    }
}
