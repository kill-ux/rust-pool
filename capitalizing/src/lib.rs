pub fn capitalize_first(input: &str) -> String {
    input[0..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut new_str = String::with_capacity(input.chars().count());
    let mut cap = true;
    for (index, c) in input.chars().enumerate() {
        if cap == true {
            new_str.push_str(&input[index..index + 1].to_uppercase());
            cap = false ;
            // continue;
        }
        if c.is_whitespace() {
            cap = true;
        }
        // let w = capitalize_first(word);
        new_str.push(c);
    }
    new_str.pop();
    new_str
}

pub fn change_case(input: &str) -> String {
    let mut new_str = String::new();
    for (index, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            new_str.push_str(&input[index..index + 1].to_lowercase());
        } else {
            new_str.push_str(&input[index..index + 1].to_uppercase());
        }
    }
    new_str
}
