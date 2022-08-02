// Design Decision: Scan token only when the compiler needs a token

use std::fmt::{Display, Formatter};
use crate::scanner::TokenKind::{TOKEN_EOF, TOKEN_ERROR};
use crate::util::bytes_between;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
    TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
    TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,
    // One or two character tokens.
    TOKEN_BANG, TOKEN_BANG_EQUAL,
    TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER, TOKEN_GREATER_EQUAL,
    TOKEN_LESS, TOKEN_LESS_EQUAL,
    // Literals.
    TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,
    // Keywords.
    TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
    TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
    TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
    TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

    TOKEN_ERROR, TOKEN_EOF
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
    pub fn new(kind: TokenKind, start: *const u8, length: usize, line: u8 ) -> Self {
        Token {
            kind,
            start,
            length,
            line
        }
    }

    pub fn make_token(kind: TokenKind) -> Token{
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
        self.start = self.current;
        //Q: whet to make EOF token? , rust does not have the '\0' token on strings
        if self.is_at_end() {
            return Token::make_token(TOKEN_EOF);
        } else {

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
    }

    fn is_at_end(&self) -> bool {
        return self.current as char == '\0';
    }
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