pub fn num_to_ordinal(x: u32) -> String {
    match x {
        11 | 12 | 13 => x.to_string() + "th",
        _ => {
            let a = x.to_string();
            let l = a.len();
            match &a[l - 1..] {
                "1" => a + "st",
                "2" => a + "nd",
                "3" => a + "rd",
                _ => a + "th",
            }
        }
    }
}
