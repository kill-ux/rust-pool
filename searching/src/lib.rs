

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (a,value) in array.iter().enumerate() {
        if key == *value {
            return Some(a);
        }
    }
    None
}