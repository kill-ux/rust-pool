use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut count = 0;
    for num in list {
        count += num;
    }
    (count as f64) / (list.len() as f64)
}

pub fn median(arr: &[i32]) -> i32 {
    let mut list = arr.to_vec();
    list.sort();
    let le = list.len();
    if le % 2 == 0 {
        return (list[le / 2] + list[le / 2 + 1]) / 2;
    }
    list[le / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut new_hash = HashMap::new();
    for num in list {
        match new_hash.get(num) {
            Some(len) => new_hash.insert(*num, len + 1),
            None => new_hash.insert(*num, 1),
        };
    }
    let mut max = *new_hash.get(&list[0]).unwrap_or(&0);
    let mut last = list[0];
    for (num, le) in new_hash {
        if le > max {
            max = le;
            last = num;
        }
    }
    last
}
