pub fn scytale_cipher(chars: String, i: u32) -> String {
    if i == 0 {
        return String::new();
    }
    let chars: Vec<char> = chars.chars().collect();
    let le = chars.len();
    let i_usize = i as usize;
    let len = (le + i_usize - 1) / i_usize;
    let mut str = String::with_capacity(le);
    for index in 0..i {
        let mut inc = index as usize;
        for _ in 0..len {
            match chars.get(inc) {
                Some(ch) => str.push(*ch),
                None => str.push(' '),
            }
            inc += i_usize;
        }
    }
    str.trim().to_string()
}
