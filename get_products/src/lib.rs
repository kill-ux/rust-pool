pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];
    for &ele in &arr {
        vec.push(
            arr.clone()
                .into_iter()
                .filter(|&a| a != ele)
                .clone()
                .reduce(|acc, e| acc * e)
                .unwrap_or(0),
        );
    }
    vec
}
