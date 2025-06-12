pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    } 
    fibonacci(n-1) + fibonacci(n-2)
}

// pub fn fibonacci(n: u32) -> u32 {
//     let mut i = 0;
//     let mut fib1 = 0;
//     let mut fib2 = 1;
//     loop {
//         if i == n {
//             break;
//         }
//         let res = fib1 + fib2;
//         fib1 = fib2;
//         fib2 = res;
//         i += 1;
//     }
//     fib1
// }
