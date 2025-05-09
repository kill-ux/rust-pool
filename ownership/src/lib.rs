
pub fn first_subword(mut s: String) -> String {
    let mut i = false;
    let mut index = s.len();
    for (j,e) in s.chars().enumerate() {
        if i == false {
            i = true;
        } else {
            if e.is_ascii_uppercase() || e == '_' {
                index = j;
            }
        }
    }
    s.replace_range(index.. , "");
    s
}