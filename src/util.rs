pub fn bytes_between(f: *const u8, l: *const u8) -> i64 {
    let binary = |c: *const u8| -> i64 {
        let raw = format!("{:?}", c);
        let without_prefix = raw.trim_start_matches("0x");
        let z = i64::from_str_radix(without_prefix, 16);
        println!("{:?}", z);
        return z.unwrap()
    };

    let result = binary(l) - binary(f);
    let result = result.abs();
    return result;
}

pub fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

pub fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z' ||
           c >= 'A' && c <= 'Z' ||
           c == '_';
}