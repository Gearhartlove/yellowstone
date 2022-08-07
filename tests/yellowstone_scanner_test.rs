extern crate yellowstone;

use yellowstone::{assert_tokens_are};
use yellowstone::scanner::Scanner;
use yellowstone::scanner::TokenKind::*;

#[test]
fn tokenizer_basic_test() {
    assert_tokens_are!("1", TOKEN_NUMBER, TOKEN_EOF);
    assert_tokens_are!("1.0", TOKEN_NUMBER, TOKEN_EOF);
    assert_tokens_are!("+", TOKEN_PLUS, TOKEN_EOF);
}

#[test]
fn tokenizer_additive_test() {
    // skipping the + symbol
    assert_tokens_are!("1 + 1", TOKEN_NUMBER, TOKEN_PLUS, TOKEN_NUMBER, TOKEN_EOF );
}