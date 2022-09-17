extern crate yellowstone;

use yellowstone::assert_tokens_are;
use yellowstone::scanner::Scanner;
use yellowstone::scanner::TokenKind::*;

//#[test]
//fn tokenizer_basic_test() {
//    assert_tokens_are!("1", TOKEN_NUMBER, TOKEN_EOF);
//    assert_tokens_are!("1.0", TOKEN_NUMBER, TOKEN_EOF);
//    assert_tokens_are!("+", TOKEN_PLUS, TOKEN_EOF);
//}
//
//#[test]
//
//fn tokenizer_additive_test() {
//    // skipping the + symbol
//    assert_tokens_are!("1 + 1", TOKEN_NUMBER, TOKEN_PLUS, TOKEN_NUMBER, TOKEN_EOF);
//}

#[test]
fn peek_test() {
    let source = String::from("Hi!");
    let mut sc = Scanner::new(&source);
    assert_eq!("H", sc.peek().unwrap());
    sc.advance();
    assert_eq!("i", sc.peek().unwrap());
    sc.advance();
    assert_eq!("!", sc.peek().unwrap());
    sc.advance();
    assert_eq!(None, sc.peek());
}

#[test]
fn peek_next_test() {
    let source = String::from("Hi!");
    let mut sc = Scanner::new(&source);
    assert_eq!("i", sc.peek_next().unwrap());
    sc.advance();
    assert_eq!("!", sc.peek_next().unwrap());
    sc.advance();
    assert_eq!(None, sc.peek_next());
}

#[test]
fn skip_whitespace_test() {    
    let source = String::from("    Hi!");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!("H", sc.peek().unwrap());
}

#[test]
fn skip_whitespace_space_source_test() {
    let source = String::from("    ");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!(None, sc.peek());
}

#[test]
fn skip_whitespace_new_line_test() {
    let source = String::from("\n");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!(2, sc.line);

    let source = String::from("\n\n\n");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!(4, sc.line);

    let source = String::from("\ny\n\ns");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!(2, sc.line);
    assert_eq!("y", sc.peek().unwrap());
    sc.advance();
    sc.skip_whitespace();
    assert_eq!(4, sc.line);
    assert_eq!("s", sc.peek().unwrap());
}

#[test]
fn skip_whitespace_comment_test() {
    let source = String::from("//comment\ny");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!("y", sc.peek().unwrap());

    let source = String::from("y//comment\ns");
    let mut sc = Scanner::new(&source);
    sc.advance(); // advance pass "y"
    sc.skip_whitespace();
    assert_eq!("s", sc.peek().unwrap());
}

#[test]
fn skip_whitespace_no_whitespace_test() {
    let source = String::from("ys");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!("y", sc.peek().unwrap());
}

#[test]
fn skip_whitespace_complex_test() {
    let source = String::from(" y  s ");
    let mut sc = Scanner::new(&source);
    sc.skip_whitespace();
    assert_eq!("y", sc.peek().unwrap());
    sc.advance();
    sc.skip_whitespace();
    assert_eq!("s", sc.peek().unwrap());
    sc.advance();
    sc.skip_whitespace();
    assert_eq!(None, sc.peek());
}


#[test]
fn tokenize_string_test() {}
#[test]
fn tokenize_number_test() {}
#[test]
fn tokenize_identifier_test() {}
#[test]
fn check_keyword_test() {}
