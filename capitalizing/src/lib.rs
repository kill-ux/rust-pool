pub fn capitalize_first(input: &str) -> String {
    input[0..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut new_str = String::with_capacity(input.chars().count());
    for word in input.split_whitespace() {
        let w = capitalize_first(word);
        new_str.push_str(&(w + " "));
    }
    new_str.pop();
    new_str
}

pub fn change_case(input: &str) -> String {
    let mut new_str = String::new();
    for (index, c) in input.chars().enumerate() {
        if c.is_ascii_uppercase() {
            new_str.push_str(&input[index..index + 1].to_lowercase());
        } else {
            new_str.push_str(&input[index..index + 1].to_uppercase());
        }
    }
    new_str
}
