pub fn delete_and_backspace(s: &mut String) {
    let mut new_str = Vec::new();
    for e in s.chars().rev() {
        if e == '+' {
            new_str.pop();
            continue
        } else {
            new_str.push(e);
        }
    }
    
    let mut new_str2 = Vec::new();
    for e in new_str.into_iter().rev() {
        if e != '-' {
            new_str2.push(e);
        } else {
            new_str2.pop();
        }
    }
    *s = new_str2.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for e in v {
        if e.contains("+") {
            let slice: Vec<&str> = e.split("+").collect();
            let num1: i32 = slice[0].parse().unwrap();
            let num2: i32 = slice[1].parse().unwrap();
            *e = (num1 + num2).to_string();
        } else if e.contains("-") {
            let slice: Vec<&str> = e.split("-").collect();
            let num1: i32 = slice[0].parse().unwrap();
            let num2: i32 = slice[1].parse().unwrap();
            *e = (num1 - num2).to_string();
        }
    }
}
