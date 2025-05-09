
pub fn arrange_phrase(phrase: &str) -> String {
    let mut new_str1: Vec<String> = Vec::new();
    for _ in phrase.split_whitespace() {
        new_str1.push("".to_string());
    }
    for word in phrase.split_whitespace() {
        let mut new_str = String::new();
        let mut index: usize = 0;
        for c in word.chars() {
            if c.is_numeric() {
                index = c.to_string().parse().unwrap();
            } else {
                new_str.push(c);
            }
        }
        new_str1[index-1] = new_str;
    }
    new_str1.join(" ")
}
