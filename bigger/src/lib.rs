use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut count: i32 = 0 ;
    for (_ , &num) in h.iter() {
        if num >= count {
            count = num;
        }
    }
    count
}