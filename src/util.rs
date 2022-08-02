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