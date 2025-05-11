use hashing::*;

fn main() {
    let v = [1, 2, 3, 4];

    println!("mean {}", mean(&v));
    println!("median {}", median(&v));
    println!("mode {}", mode(&v));
}
