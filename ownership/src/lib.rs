
pub fn first_subword(mut s: String) -> String {
    let mut i = false;
    for (j,e) in s.chars().enumerate() {
        if i == false {
            i = true;
        } else {
            if e.is_ascii_uppercase() || e == '_' {
                s.replace_range(j.. , "");
                break;
            }
        }
    }
    s
}