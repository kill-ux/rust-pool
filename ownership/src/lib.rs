pub fn first_subword(mut s: String) -> String {
    for (j, e) in s[1..].chars().enumerate() {
        if e.is_ascii_uppercase() || e == '_' {
            s.replace_range(j+1.., "");
            break;
        }
    }
    s
}
