pub fn num_to_ordinal(x: u32) -> String {
    if x == 11 || x == 12 || x == 13 {
        return x.to_string() + "th";
    }
    let num = x % 10;
    let a = match num {
        1 => x.to_string() + "st",
        2 => x.to_string() + "nd",
        3 => x.to_string() + "rd",
        _ => x.to_string() + "th",
    };
    a
}
