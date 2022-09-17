use crate::scanner::TokenKind::*;

pub fn compile(source: &String) {
    let mut scanner = Scanner::new(source);
    let mut line: i32 = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{4} ", token.line);
            line = token.line;
        }
        println!("{} {}", token.kind, teken.length); 
        
        if token.type == TokenKind::TOKEN_EOF {
            break
        }
    }
    // scanner initialization
}
