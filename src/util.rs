pub fn is_digit(c: &str) -> bool {
    ("0"..="9").contains(&c)
}

pub fn is_alpha(c: &str) -> bool {
    ("a"..="z").contains(&c)
    || ("A"..="Z").contains(&c)
    || c == "_"
}
