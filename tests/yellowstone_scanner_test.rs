extern crate yellowstone;

use yellowstone::assert_tokens_are;
use yellowstone::scanner::Scanner;
use yellowstone::scanner::Token;
use yellowstone::scanner::TokenKind;
use yellowstone::scanner::TokenKind::*;

#[test]
fn tokenizer_basic_test() {
    assert_tokens_are!("1", TOKEN_NUMBER, TOKEN_EOF);
    assert_tokens_are!("1.0", TOKEN_NUMBER, TOKEN_EOF);
    assert_tokens_are!("+", TOKEN_PLUS, TOKEN_EOF);
}

#[test]
fn tokenizer_paren_test() {
    assert_tokens_are!(
        "(1)",
        TOKEN_LEFT_PAREN,
        TOKEN_NUMBER,
        TOKEN_RIGHT_PAREN,
        TOKEN_EOF
    );
}

#[test]
fn tokenizer_additive_test() {
    //assert_tokens_are!("1 + 1", TOKEN_NUMBER, TOKEN_PLUS, TOKEN_NUMBER, TOKEN_EOF);
    assert_tokens_are!("1+1", TOKEN_NUMBER, TOKEN_PLUS, TOKEN_NUMBER, TOKEN_EOF);
}

#[test]
fn tokenizer_subtractive_test() {
    assert_tokens_are!("1 - 1", TOKEN_NUMBER, TOKEN_MINUS, TOKEN_NUMBER, TOKEN_EOF);
}

#[test]
fn tokenizer_multiply_test() {
    assert_tokens_are!("1 * 1", TOKEN_NUMBER, TOKEN_STAR, TOKEN_NUMBER, TOKEN_EOF);
}

#[test]
fn tokenizer_divide_test() {
    assert_tokens_are!("1 / 1", TOKEN_NUMBER, TOKEN_SLASH, TOKEN_NUMBER, TOKEN_EOF);
}

#[test]
fn tokenizer_expression_test() {
    assert_tokens_are!(
        "print 1 + 1",
        TOKEN_PRINT,
        TOKEN_NUMBER,
        TOKEN_PLUS,
        TOKEN_NUMBER,
        TOKEN_EOF
    );
}

#[test]
fn tokenizer_global_var_managerie() {
    let source = "
        var lang = \"yellowstone\";
        var num = 9;
        var yes = true;
        var nothing = nil;
    ";

    assert_tokens_are!(
        source, 
        TOKEN_VAR, TOKEN_IDENTIFIER, TOKEN_EQUAL, TOKEN_STRING, TOKEN_SEMICOLON,
        TOKEN_VAR, TOKEN_IDENTIFIER, TOKEN_EQUAL, TOKEN_NUMBER, TOKEN_SEMICOLON,
        TOKEN_VAR, TOKEN_IDENTIFIER, TOKEN_EQUAL, TOKEN_TRUE, TOKEN_SEMICOLON,
        TOKEN_VAR, TOKEN_IDENTIFIER, TOKEN_EQUAL, TOKEN_NIL, TOKEN_SEMICOLON,
        TOKEN_EOF
    )
}

#[test]
fn tokenizer_variable_test() {
    assert_tokens_are!(
        "var beverage = \"cafe au lait\"; 
        var breakfast = \"beignets with \" + beverage;
        print breakfast;",
        TOKEN_VAR,
        TOKEN_IDENTIFIER,
        TOKEN_EQUAL,
        TOKEN_STRING,
        TOKEN_SEMICOLON,
        //
        TOKEN_VAR,
        TOKEN_IDENTIFIER,
        TOKEN_EQUAL,
        TOKEN_STRING,
        TOKEN_PLUS,
        TOKEN_IDENTIFIER,
        TOKEN_SEMICOLON,
        ////
        TOKEN_PRINT,
        TOKEN_IDENTIFIER,
        TOKEN_SEMICOLON,
        TOKEN_EOF
    )
}

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

    let source = String::from("\r\t");
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
fn tokenize_string_test() {
    let source = String::from("\"yellowstone\"");
    let mut sc = Scanner::new(&source);
    let token: Token = sc.scan_token();
    assert_eq!("\"yellowstone\"", token.slice);
    assert_eq!(TokenKind::TOKEN_STRING, token.kind);

    let source = String::from("\"yellow\" \"stone\"");
    let mut sc = Scanner::new(&source);
    let yellow: Token = sc.scan_token();
    assert_eq!("\"yellow\"", yellow.slice);
    assert_eq!(TokenKind::TOKEN_STRING, yellow.kind);
    let stone: Token = sc.scan_token();
    assert_eq!("\"stone\"", stone.slice);
    assert_eq!(TokenKind::TOKEN_STRING, stone.kind);
}

#[test]
fn tokenize_string_unterminated_test() {
    let source = String::from("\"yellow"); // unterminated string
    let mut sc = Scanner::new(&source);
    let error: Token = sc.scan_token();

    assert_eq!("Unterminated string.", error.slice);
    assert_eq!(TokenKind::TOKEN_ERROR, error.kind);
}

#[test]
fn multi_line_string_test() {
    let source = String::from("\"yellow\nstone\""); // unterminated string
    let mut sc = Scanner::new(&source);
    let token = sc.scan_token();
    assert_eq!("\"yellow\nstone\"", token.slice);
    assert_eq!(2, token.line);
    assert_eq!(TokenKind::TOKEN_STRING, token.kind);
}

#[test]
fn tokenize_number_test() {
    let source = String::from("100");
    let mut sc = Scanner::new(&source);
    let num = sc.scan_token();
    assert_eq!("100", num.slice);
    assert_eq!(TokenKind::TOKEN_NUMBER, num.kind);
}

#[test]
fn tokenize_number_decimal_test() {
    let source = String::from("100.200");
    let mut sc = Scanner::new(&source);
    let num = sc.scan_token();
    assert_eq!("100.200", num.slice);
    assert_eq!(TokenKind::TOKEN_NUMBER, num.kind);
}

#[test]
fn tokenize_identifier_test() {
    let source = String::from("and");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("and", token.slice);
    assert_eq!(TokenKind::TOKEN_AND, token.kind);

    let source = String::from("class");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("class", token.slice);
    assert_eq!(TokenKind::TOKEN_CLASS, token.kind);

    let source = String::from("else");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("else", token.slice);
    assert_eq!(TokenKind::TOKEN_ELSE, token.kind);

    let source = String::from("false");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("false", token.slice);
    assert_eq!(TokenKind::TOKEN_FALSE, token.kind);

    let source = String::from("for");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("for", token.slice);
    assert_eq!(TokenKind::TOKEN_FOR, token.kind);

    let source = String::from("fun");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("fun", token.slice);
    assert_eq!(TokenKind::TOKEN_FUN, token.kind);

    let source = String::from("if");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("if", token.slice);
    assert_eq!(TokenKind::TOKEN_IF, token.kind);

    let source = String::from("nil");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("nil", token.slice);
    assert_eq!(TokenKind::TOKEN_NIL, token.kind);

    let source = String::from("or");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("or", token.slice);
    assert_eq!(TokenKind::TOKEN_OR, token.kind);

    let source = String::from("print");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("print", token.slice);
    assert_eq!(TokenKind::TOKEN_PRINT, token.kind);

    let source = String::from("return");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("return", token.slice);
    assert_eq!(TokenKind::TOKEN_RETURN, token.kind);

    let source = String::from("super");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("super", token.slice);
    assert_eq!(TokenKind::TOKEN_SUPER, token.kind);

    let source = String::from("this");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("this", token.slice);
    assert_eq!(TokenKind::TOKEN_THIS, token.kind);

    let source = String::from("true");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("true", token.slice);
    assert_eq!(TokenKind::TOKEN_TRUE, token.kind);

    let source = String::from("var");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("var", token.slice);
    assert_eq!(TokenKind::TOKEN_VAR, token.kind);

    let source = String::from("while");
    let mut sc = Scanner::from(&source);
    let token = sc.scan_token();
    assert_eq!("while", token.slice);
    assert_eq!(TokenKind::TOKEN_WHILE, token.kind);
}

#[test]
fn tokenizer_block_test() {
    let source = String:: from("{}");
    assert_tokens_are!(source, TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE, TOKEN_EOF);


    let source = String:: from("{ var a = \"apple\"; }");
    assert_tokens_are!(source, TOKEN_LEFT_BRACE, TOKEN_VAR, TOKEN_IDENTIFIER,
        TOKEN_EQUAL, TOKEN_STRING, TOKEN_SEMICOLON,TOKEN_RIGHT_BRACE, TOKEN_EOF);


    let source = String:: from("print \"hello\"; { \"world\" }");
    assert_tokens_are!(source, TOKEN_PRINT, TOKEN_STRING, TOKEN_SEMICOLON,
        TOKEN_LEFT_BRACE, TOKEN_STRING,TOKEN_RIGHT_BRACE, TOKEN_EOF);
}

#[test]
fn tokenizer_assert_test() {
    let source = String::from("assert_eq(true, true)");
    assert_tokens_are!(source, TOKEN_ASSERT_EQ, TOKEN_LEFT_PAREN, TOKEN_TRUE, TOKEN_COMMA, TOKEN_TRUE, TOKEN_RIGHT_PAREN, TOKEN_EOF);
}
