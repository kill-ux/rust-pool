pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();
    for e in names {
        let mut vec: Vec<String> = Vec::new();
        for e in e.split(" ") {
            vec.push(e[0..1].to_string() + ".");
        }
        new_vec.push(vec.join(" "));
    }
    new_vec
}
