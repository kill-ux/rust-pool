pub fn delete_and_backspace(s: &mut String) {
    let mut vec: Vec<usize> = Vec::new();
    let mut vec_count: Vec<usize> = Vec::new();
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c2i = 0;
    for (i, e) in s.char_indices() {
        // handel +
        if e == '+' {
            if c1 == 0 {
                vec.push(i);
            }
            c1 += 1;
        } else if c1 != 0 {
            vec_count.push(c1 * 2);
            c1 = 0;
        }

        // handel -
        if e == '-' {
            if c2 == 0 {
                vec.push(i);
                c2i = vec.len() - 1;
            }
            c2 += 1;
        } else if c2 != 0 {
            vec_count.push(c2 * 2);
            if vec[c2i] >= c2 {
                vec[c2i] = vec[c2i] - c2;
            } else {
                vec[c2i] = 0;
            }
            c2 = 0;
        }
    }
    if c2 != 0 {
        vec_count.push(c2 * 2);
        let l = vec.len() - 1;
        vec[l] = vec[l] - c2;
    }
    if c1 != 0 {
        vec_count.push(c1 * 2);
    }

    let mut sum = 0;
    for (i, e) in vec.into_iter().enumerate() {
        s.replace_range(e - sum..e + vec_count[i] - sum, "");
        sum += vec_count[i];
    }
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
