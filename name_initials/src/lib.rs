pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();
    for e in names {
        let mut vec: Vec<String> = Vec::new();
        let str = e.split_whitespace();
        for e in str {
            let mut mystr = e.to_string();
            mystr.replace_range(1..,".");
            vec.push(mystr);
        }
        new_vec.push(vec.join(" "));
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        println!("{:?}", initials(names));
    }
}
