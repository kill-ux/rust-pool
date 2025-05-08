pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn fibonacci2(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    } 
    fibonacci2(n-1) + fibonacci2(n-2)
}

pub fn fibonacci(n: u32) -> u32 {
    let mut i = 0;
    let mut fib1 = 0;
    let mut fib2 = 1;
    loop {
        if i == n {
            break;
        }
        let res = fib1 + fib2;
        fib1 = fib2;
        fib2 = res;
        i += 1;
    }
    fib1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "The element in the position {} in fibonacci series is {}",
            0,
            fibonacci2(0)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            4,
            fibonacci2(4)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            22,
            fibonacci2(22)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            20,
            fibonacci2(20)
        );
    }
}
