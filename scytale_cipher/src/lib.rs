pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut new_str = String::new();
    let mut vec: Vec<String> = vec![];

    for j in 0..(message.len() as f32 / i as f32).ceil() as usize {
        let index = j * i as usize;
        if message.len() < index + i as usize {
            let mut s = String::from(&message[index..]);
            s.push_str(&" ".repeat((index + i as usize) - message.len()));
            vec.push(s);
        } else {
            vec.push(message[index..index + i as usize].to_string());
        }
    }

    for index_j in 0..i as usize {
        for index_i in 0..vec.len() {
            let ch = &vec[index_i][index_j..index_j + 1];
            new_str.push(ch.to_string().chars().nth(0).unwrap());
        }
    }
    new_str.trim().to_string()
}
