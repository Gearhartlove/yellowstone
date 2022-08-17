#[macro_export]
macro_rules! assert_tokens_are {
    ($s:expr, $( $x:expr ),*) => {
        {
            let source: &str = $s;
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            println!("{}", $s);
            println!("{:?}", temp_vec);

            //scanner
            let source = $s.to_string();
            let mut scanner = Scanner::from(&source);
            let mut i = 0;
            loop {
                if scanner.is_at_end() {
                    break;
                }

                let token = scanner.scan_token();
                if let Some(correct_token_kind) = temp_vec.get(i) {
                    assert_eq!(*correct_token_kind, token.kind);
                } else {
                    panic!("Out of bounds error. There are no more tokens to compare against.")
                }
                i += 1;
            }
            // while !scanner.is_at_end() {
            //     let token = scanner.scan_token();
            //     //compare token to vec values
            //     if let Some(t_kind) = temp_vec.get(i) {
            //         assert_eq!(token.kind, *t_kind);
            //     } else {
            //         panic!("assertion failed, out of bounds index; not enough tokens to compare against.")
            //     }
            //     i += 1;
            // }
        }
    };
}