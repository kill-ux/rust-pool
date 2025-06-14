pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c1: f64 = c.into();
    (c, c1.exp(), c1.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut new_str = String::new();
    for e in a.split_whitespace() {
        let (_, num, _) = nbr_function(e.parse().unwrap());
        new_str.push_str(&num.to_string());
        new_str.push(' ');
    }
    new_str.pop();
    (a, new_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut new_vec: Vec<f64> = Vec::new();
    for e in &b {
        let (_, _, num) = nbr_function(*e);
        new_vec.push(num);
    }
    (b, new_vec)
}


