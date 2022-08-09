// Design Decision: Scan token only when the compiler needs a token

use std::fmt::{Display, Formatter};
use std::str::Chars;
use std::thread::current;
use crate::scanner::TokenKind::*;
use crate::util::*;

#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
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
    TOKEN_STRING(&'a str),
    TOKEN_NUMBER(&'a str),
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

impl<'a> Display for TokenKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: u8,
    pub literal: Option<&'a str>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, line: u8, literal: Option<&str>) -> Self {
        Token {
            kind,
            literal,
            line,
        }
    }

}

pub struct Scanner<'a> {
    pub source: Chars<'a>,
    pub line: u8,
}

impl<'a> Scanner<'a> {
    /// Looks at each character in the source code and creates tokens derived from those characters.
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        // current considered character
        let c = self.advance();

        // return true after scanning all tokens
        if self.is_at_end() {
            return self.make_token(TOKEN_EOF);
        }

        // multi character lexemes, must be converted to &str
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
        return match kind {
            TOKEN_STRING(literal) | TOKEN_NUMBER(literal) => {
                Token::new(kind, self.line, Some(literal))
            },
            _ => {
                // the character of the token can be inferred from it's kind
                Token::new(kind, self.line, None)
            },
        };
    }

    fn error_token(&self, message: &str) -> Token {
        return Token::new(TOKEN_ERROR, self.line, Some(message));
    }

    /// Checks the Char iterator for a next character. If no other charters exist, the scanner has reached the end.
    pub fn is_at_end(&self) -> bool {
        if let None = self.source.peekable().peek() {
            return true;
        }
        return false;
    }

    fn advance(&mut self) -> char {
        let next = self.source.next();
        match next {
            None => { panic!("Advancing when no character next!")},
            Some(c) => { return c}
        }
    }

    fn peek(&self) -> char {
        let c = self.source.peekable().peek().unwrap();
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        unsafe {
            return Some(*self.start.add(self.current as usize + 1) as char);
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
        let mut word = String::new();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                // increment line for debugging
                self.line += 1;
            }
            let c = self.advance();
            word.push(c);
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote
        let _ = self.advance();
        return self.make_token(TOKEN_STRING(word.as_str()));
    }

    fn tokenize_number(&mut self) -> Token {
        let mut integer_literal: Vec<char> = vec!();
        let consume_numbers = || {
            while is_digit(self.peek()) {
                let c = self.advance();
                integer_literal.push(c);
            }
        };

        consume_numbers();

        // Look for decimal of number
        if let Some(c) = self.peek_next() {
            if self.peek() == '.' && is_digit(c) {
                // Consume the "."
                let c = self.advance();
                integer_literal.push(c);

                consume_numbers();
            }
        }
        let str_number = integer_literal.iter().map().collect::<&str>();
        return self.make_token(TOKEN_NUMBER(str_number));
    }

    fn tokenize_identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenKind {
        let c = unsafe { *self.start as char };
        match c as char {
            'a' => return self.check_keyword(1, 2, "nd", TOKEN_AND),
            'c' => return self.check_keyword(1, 4, "lass", TOKEN_CLASS),
            'e' => return self.check_keyword(1, 3, "lse", TOKEN_ELSE),
            'f' => {
                let current_byte = self.current_byte();
                if bytes_between(current_byte, self.start) > 1 {
                    match self.peek() {
                        'a' => return self.check_keyword(2, 3, "lse", TOKEN_FALSE),
                        'o' => return self.check_keyword(2, 1, "r", TOKEN_FOR),
                        'u' => return self.check_keyword(2, 1, "n", TOKEN_FUN),
                        _ => {}
                    }
                }
            }
            'i' => return self.check_keyword(1, 1, "f", TOKEN_IF),
            'n' => return self.check_keyword(1, 2, "il", TOKEN_NIL),
            'o' => return self.check_keyword(1, 1, "or", TOKEN_OR),
            'p' => return self.check_keyword(1, 4, "rint", TOKEN_PRINT),
            'r' => return self.check_keyword(1, 5, "eturn", TOKEN_RETURN),
            's' => return self.check_keyword(1, 4, "uper", TOKEN_SUPER),
            't' => {
                let current_byte = self.current_byte();
                if bytes_between(current_byte, self.start) > 1 {
                    match self.peek() {
                        'h' => return self.check_keyword(2, 2, "is", TOKEN_THIS),
                        'r' => return self.check_keyword(2, 2, "ue", TOKEN_TRUE),
                        _ => {}
                    }
                }
            }
            'v' => return self.check_keyword(1, 2, "ar", TOKEN_VAR),
            'w' => return self.check_keyword(1, 4, "hile", TOKEN_WHILE),
            _ => {}
        }
        return TOKEN_IDENTIFIER;
    }

    fn check_keyword(&self, start: u8, length: u8, rest: &str, kind: TokenKind) -> TokenKind {
        let keyword_start = unsafe {
            new_inc_ptr(self.start, start as usize)
        };

        let current_byte = self.current_byte();
        if bytes_between(current_byte, self.start) == (start + length) as i64
            && memcmp_equal(keyword_start, rest, length as usize) {
            return kind;
        } else {
            return TOKEN_IDENTIFIER;
        }
    }

    fn current_byte(&self) -> *const u8 {
        let current_byte = unsafe {
            self.start.add(self.current as usize)
        };
        return current_byte;
    }
}

impl From<&String> for Scanner {
    fn from(source: &String) -> Self {
        let source = source.chars();
        Scanner {
            source,
        }
    }
}