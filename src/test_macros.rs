#[macro_export]
macro_rules! assert_tokens_are {
    ($s:expr, $( $x:expr ),*) => {
        {
            let mut source = $s.to_string();

            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            //scanner
            let mut scanner = Scanner::from(&source);
            let mut i = 0;

            while !scanner.is_at_end() {
                let parsed_token = scanner.scan_token();
                // compare token to vec values
                if let Some(t_kind) = temp_vec.get(i) {
                    assert_eq!(&parsed_token.kind, t_kind);
                    if *t_kind != TokenKind::TOKEN_NIL && *t_kind != TokenKind::TOKEN_FALSE && *t_kind != TokenKind::TOKEN_TRUE && *t_kind != TokenKind::TOKEN_NUMBER && *t_kind != TokenKind::TOKEN_STRING && *t_kind != TokenKind::TOKEN_IDENTIFIER {
                        println!("{}", parsed_token.kind);
                        scanner.advance();
                    }
                } else {
                    panic!("assertion failed, out of bounds index; not enough tokens to compare against.")
                }
                i += 1;
                //scanner.advance();
            }
        }
    };
}
