pub fn is_pangram(s: &str) -> bool {
    let s = s.to_uppercase();
    let vec: Vec<char> = ('A'..='Z').into_iter().collect();
    for ch in vec {
        if !s.contains(ch) {
            return false;
        }
    }
    true
}
