pub fn is_digit(c: &str) -> bool {
    ("0"..="9").contains(&c)
}

pub fn is_alpha(c: &str) -> bool {
    ("a"..="z").contains(&c)
    || ("A"..="Z").contains(&c)
    || c == "_"
}

pub fn grow_capacity(capacity: usize) -> usize {
    return if capacity <= 1 {
        8
    } else {
        capacity * 2
    }
}