pub fn get_diamond(c: char) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let mut spaces = (c as u8 - b'A') as usize;
    let mut index: usize = 0;
    for ch in 'A'..=c {
        // spaces
        let mut str = " ".repeat(spaces);
        // charachter
        str.push(ch);

        if index != 0 {
            // spaces inside
            str.push_str(&" ".repeat(index * 2 - 1));
            // ch
            str.push(ch);
        }

        // spaces
        str.push_str(&" ".repeat(spaces));

        vec.push(str);
        index += 1;
        spaces = spaces.saturating_sub(1);
    }

    let mut merror = vec.clone();
    merror.pop();
    merror.reverse();
    vec.extend(merror);
    vec
}
