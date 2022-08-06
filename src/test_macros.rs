#[macro_export]
macro_rules! assert_tokens_are {
    ($s:expr, $( $x:expr ),*) => {
        {
            let source: &str = $s;
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            //scanner
            let mut scanner = Scanner::from(&$s.to_string());
            let mut i = 0;
            while !scanner.is_at_end() {
                let token = scanner.scan_token();
                // compare token to vec values
                if let Some(t_kind) = temp_vec.get(i) {
                    assert_eq!(&token.kind, t_kind);
                } else {
                    panic!("assertion failed, out of bounds index; not enough tokens to compare against.")
                }
                i += 1;
            }
        }
    };
}