pub fn rotate(input: &str, key: i8) -> String {
    let mut output = String::new();
    let mut key = key % 26;
    if key < 0 {
        key = 26 + key;
    }

    for ch in input.chars() {
        let alpha = match ch {
            'A'..='Z' => {
                let base = ch as u8 - b'A';
                (((base as i8 + key) % 26) as u8 + b'A') as char
            }
            'a'..='z' => {
                let base = ch as u8 - b'a';
                (((base as i8 + key) % 26) as u8 + b'a') as char
            }
            _ => ch,
        };
        output.push(alpha);
    }

    output
}
