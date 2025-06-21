pub fn rotate(input: &str, mut key: i8) -> String {
    key = key % 26;
    if key < 0 {
        key = 26 + key;
    }
    let mut res = String::new();
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        let mut num = ch as u8;
        if ch.is_ascii_alphabetic() {
            num = (ch as u8 - b'A' + key as u8) % 26 + b'A';
            if ch.is_ascii_lowercase() {
                num = (ch as u8 - b'a' + key as u8) % 26 + b'a';
            }
        }
        res.push(num as char);
    }
    res
}
