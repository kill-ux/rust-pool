pub fn number_logic(num: u32) -> bool {
    let str = num.to_string();
    let power = str.len() as u32;
    let mut sum: u32 = 0;
    for ch in str.chars() {
        let n = ch as u32 - b'0' as u32;
        sum += n.pow(power);
    }
    num == sum
}
