use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map: HashMap<char, bool> = HashMap::new();
    for ele in s.chars() {
        if ele.is_ascii_alphabetic() {
            map.insert(ele.to_lowercase().nth(0).unwrap(), true);
        }
    }
    map.len() == 26
}
