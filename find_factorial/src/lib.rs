pub fn factorial(num: u64) -> u64 {
    if num == 1 || num == 0 { return 1 }
    num * (factorial(num - 1))
}
