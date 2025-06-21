pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, ele) in array.iter().enumerate() {
        if *ele == key {
            return Some(i);
        }
    }
    None
}
