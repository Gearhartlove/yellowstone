// Design Decision: Scan token only when the compiler needs a token

use std::fmt::{Display, Formatter};
use std::str::Chars;

use crate::scanner::TokenKind::*;
use crate::util::*;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
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
    TOKEN_STRING(String),
    TOKEN_NUMBER(String),
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

    TOKEN_ERROR(String),
    TOKEN_EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u8,
}

impl Token {
    pub fn new(kind: TokenKind, line: u8) -> Self {
        Token {
            kind,
            line,
        }
    }

    pub fn literal(&self) -> &str {
        match &self.kind {
            TOKEN_LEFT_PAREN => { "(" }
            TOKEN_RIGHT_PAREN => { ")" }
            TOKEN_LEFT_BRACE => { "[" }
            TOKEN_RIGHT_BRACE => { "]" }
            TOKEN_COMMA => { "," }
            TOKEN_DOT => { "." }
            TOKEN_MINUS => { "-" }
            TOKEN_PLUS => { "+" }
            TOKEN_SEMICOLON => { ";" }
            TOKEN_SLASH => { "/" }
            TOKEN_STAR => { "*" }
            TOKEN_BANG => { "!" }
            TOKEN_BANG_EQUAL => { "!=" }
            TOKEN_EQUAL => { "=" }
            TOKEN_EQUAL_EQUAL => { "==" }
            TOKEN_GREATER => { ">" }
            TOKEN_GREATER_EQUAL => { ">=" }
            TOKEN_LESS => { "<" }
            TOKEN_LESS_EQUAL => { "<=" }
            TOKEN_IDENTIFIER => { todo!() }
            TOKEN_STRING(s) => { s.as_str() }
            TOKEN_NUMBER(n) => { n.as_str() }
            TOKEN_AND => { "and" }
            TOKEN_CLASS => { "class" }
            TOKEN_ELSE => { "else" }
            TOKEN_FALSE => { "false" }
            TOKEN_FOR => { "for" }
            TOKEN_FUN => { "fun" }
            TOKEN_IF => { "if" }
            TOKEN_NIL => { "nil" }
            TOKEN_OR => { "or" }
            TOKEN_PRINT => { "print" }
            TOKEN_RETURN => { "return" }
            TOKEN_SUPER => { "super" }
            TOKEN_THIS => { "this" }
            TOKEN_TRUE => { "true" }
            TOKEN_VAR => { "var" }
            TOKEN_WHILE => { "while" }
            TOKEN_ERROR(m) => { m.as_str() }
            TOKEN_EOF => { "eof" }
        }
    }
}

pub struct Scanner<'a> {
    pub source: Chars<'a>,
    pub line: u8,
}

impl<'a> Scanner<'_> {
    /// Looks at each character in the source code and creates tokens derived from those characters.
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        // return true after scanning all tokens
        if self.is_at_end() {
            return self.make_token(TOKEN_EOF);
        }

        // current considered character
        let c = self.advance();

        // multi character lexemes, must be converted to &str
        if is_alpha(&c) {
            return self.tokenize_string(c);
        }
        if is_digit(&c) {
            let token = self.tokenize_number(c);
            return token;
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
                return self.tokenize_string('c');
            }
            _ => {}
        }
        return self.error_token("Unexpected character.");
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        return Token::new(kind.clone(), self.line);
    }

    fn error_token(&self, message: &'a str) -> Token {
        return Token::new(TOKEN_ERROR(message.to_string()), self.line);
    }

    /// Checks the Char iterator for a next character. If no other charters exist, the scanner has reached the end.
    pub fn is_at_end(&self) -> bool {
        let mut source = self.source.clone().peekable();
        let peek = source.peek();
        return match peek {
            None => true,
            _ => false,
        };
    }

    fn advance(&mut self) -> char {
        let next = self.source.next();
        match next {
            None => { panic!("Error: Advancing when no character next!") }
            Some(c) => { return c; }
        }
    }

    fn peek(&self) -> Option<char> {
        let mut source = self.source.clone().peekable();
        let peek = source.peek().cloned();
        return peek;
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        // peek the next character
        let mut source = self.source.clone().peekable();
        source.next();
        let peek = source.peek().cloned();
        return peek;
    }

    fn expect(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(c) = self.peek() {
            return c == expected;
        } else {
            return false;
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                        break;
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                        return;
                    }
                    '/' => {
                        if self.expect('/') {
                            while self.peek() != Some('\n') && !self.is_at_end() {
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
            return;
        }
    }

    fn tokenize_string(&mut self, literal_start: char) -> Token {
        let mut literal = String::new();
        literal.push(literal_start);

        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                // increment line for debugging
                self.line += 1;
            }
            let c = self.advance();
            literal.push(c);
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote
        let _ = self.advance();

        return self.make_token(TOKEN_STRING(literal));
    }

    fn tokenize_number(&mut self, literal_start: char) -> Token {
        let mut integer_literal: Vec<char> = vec!();
        integer_literal.push(literal_start);

        let mut consume_numbers = |scanner: &mut Scanner, iter: &mut Vec<char>| {
            while !scanner.is_at_end() && is_digit(&scanner.peek().unwrap()) {
                let c = scanner.advance();
                iter.push(c);
            }
        };

        consume_numbers(self, &mut integer_literal);

        // Look for decimal of number
        if let Some(c) = self.peek_next() {
            if self.peek() == Some('.') && is_digit(&c) {
                // Consume the "."
                let c = self.advance();
                integer_literal.push(c);

                consume_numbers(self, &mut integer_literal);
            }
        }

        let mut string_number = String::new();
        for c in integer_literal {
            string_number.push(c);
        }

        return self.make_token(TOKEN_NUMBER(string_number));
    }

    fn tokenize_identifier(&mut self) -> Token {
        while is_alpha(&self.peek().unwrap()) || is_digit(&self.peek().unwrap()) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    // refactor
    fn identifier_type(&self) -> TokenKind {
        todo!()
        // let c = self
        // match c as char {
        //     'a' => return self.check_keyword(1, 2, "nd", TOKEN_AND),
        //     'c' => return self.check_keyword(1, 4, "lass", TOKEN_CLASS),
        //     'e' => return self.check_keyword(1, 3, "lse", TOKEN_ELSE),
        //     'f' => {
        //         let current_byte = self.current_byte();
        //         if bytes_between(current_byte, self.start) > 1 {
        //             match self.peek() {
        //                 'a' => return self.check_keyword(2, 3, "lse", TOKEN_FALSE),
        //                 'o' => return self.check_keyword(2, 1, "r", TOKEN_FOR),
        //                 'u' => return self.check_keyword(2, 1, "n", TOKEN_FUN),
        //                 _ => {}
        //             }
        //         }
        //     }
        //     'i' => return self.check_keyword(1, 1, "f", TOKEN_IF),
        //     'n' => return self.check_keyword(1, 2, "il", TOKEN_NIL),
        //     'o' => return self.check_keyword(1, 1, "or", TOKEN_OR),
        //     'p' => return self.check_keyword(1, 4, "rint", TOKEN_PRINT),
        //     'r' => return self.check_keyword(1, 5, "eturn", TOKEN_RETURN),
        //     's' => return self.check_keyword(1, 4, "uper", TOKEN_SUPER),
        //     't' => {
        //         let current_byte = self.current_byte();
        //         if bytes_between(current_byte, self.start) > 1 {
        //             match self.peek() {
        //                 'h' => return self.check_keyword(2, 2, "is", TOKEN_THIS),
        //                 'r' => return self.check_keyword(2, 2, "ue", TOKEN_TRUE),
        //                 _ => {}
        //             }
        //         }
        //     }
        //     'v' => return self.check_keyword(1, 2, "ar", TOKEN_VAR),
        //     'w' => return self.check_keyword(1, 4, "hile", TOKEN_WHILE),
        //     _ => {}
        // }
        // return TOKEN_IDENTIFIER;
    }

    fn check_keyword(&self, start: u8, length: u8, rest: &str, kind: TokenKind) -> TokenKind {
        todo!("Refactor with identifier type")
        // let keyword_start = unsafe {
        //     new_inc_ptr(self.start, start as usize)
        // };
        //
        // let current_byte = self.current_byte();
        // if bytes_between(current_byte, self.start) == (start + length) as i64
        //     && memcmp_equal(keyword_start, rest, length as usize) {
        //     return kind;
        // } else {
        //     return TOKEN_IDENTIFIER;
        // }
    }
}

impl<'a> From<&'a String> for Scanner<'a> {
    fn from(source: &'a String) -> Self {
        let source = source.chars();
        Scanner {
            source,
            line: 0,
        }
    }
}
