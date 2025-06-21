use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map: HashMap<char, bool> = HashMap::new();
    for ele in s.chars() {
        map.insert(ele, true);
    }
    map.len() == 26
}
