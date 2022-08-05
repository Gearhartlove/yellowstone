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

pub fn memcmp_equal(str1: *const u8, str2: &str, length: usize) -> bool {
    unsafe {
        let bytes1 = std::slice::from_raw_parts(str1, length);
        let bytes2 = str2.as_bytes();
        return bytes2 == bytes1;
    }
}

pub unsafe fn new_inc_ptr(ptr: *const u8, x: usize) -> *const u8 {
    let new_ptr: *const u8 = ptr;
    unsafe {
        new_ptr.add(x)
    };
    return new_ptr;
}

pub unsafe fn new_dec_ptr(ptr: *const u8, x: usize) -> *const u8 {
    let new_ptr: *const u8 = ptr;
    unsafe {
        new_ptr.sub(x)
    };
    return new_ptr;
}