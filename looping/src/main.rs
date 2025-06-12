use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut number: u32 = 0;
    loop {
        println!("{}", riddle);
        let mut input = String::new();
        number += 1;
        io::stdin().read_line(&mut input).unwrap();
        if input == "The letter e\n" {
            println!("Number of trials: {}", number);
            break;
        }
    }
}
