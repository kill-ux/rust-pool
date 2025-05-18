use diamond_creation::*;

fn main() {
    // println!("{:?}", get_diamond('A'));
    println!("{:#?}", get_diamond('Z'));
    for line in get_diamond('M') {
        println!("{}", line);
    }
}