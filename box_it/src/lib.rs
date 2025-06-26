pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut vec: Vec<u32> = vec![];
    for ele in s.split_whitespace() {
        match ele.strip_suffix("k") {
            Some(num) => vec.push((num.parse::<f32>().unwrap() * 1000.) as u32),
            _ => vec.push(ele.parse::<f32>().unwrap() as u32),
        }
    }
    Box::new(vec)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
