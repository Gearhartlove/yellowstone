extern crate yellowstone;

use yellowstone::{assert_tokens_are};
use yellowstone::scanner::Scanner;
use yellowstone::scanner::TokenKind::*;

#[test]
fn tokenizer_basic_test() {
    assert_tokens_are!("1", TOKEN_NUMBER("1".to_string()), TOKEN_EOF);
    assert_tokens_are!("1 + 1", TOKEN_NUMBER("1".to_string()), TOKEN_PLUS, TOKEN_NUMBER("1".to_string()), TOKEN_EOF);
    assert_tokens_are!("1 \n + \n 1 ", TOKEN_NUMBER("1".to_string()), TOKEN_PLUS, TOKEN_NUMBER("1".to_string()), TOKEN_EOF);
}

#[test]
fn tokenizer_whitespace_test() {}

#[test]
fn tokenizer_number_test() {}

#[test]
fn tokenizer_string_test() {}

// note: refactoring involved
#[test]
fn tokenizer_identifier_test() {}

#[test]
fn tokenizer_binary_test() {}

#[test]
fn tokenizer_unary_test() {}