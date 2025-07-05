pub fn first_fifty_even_square() -> Vec<i32> {
    (1..=50)
        .map(|x| x * 2)
        .map(|x: i32| x.pow(2))
        .take(50)
        .collect()
}
