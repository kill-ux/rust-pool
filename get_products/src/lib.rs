pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];
    for &ele in &arr {
        if let Some(e) = arr
            .clone()
            .into_iter()
            .filter(|&a| a != ele)
            .clone()
            .reduce(|acc, e| acc * e)
        {
            vec.push(e);
        }
    }
    vec
}
