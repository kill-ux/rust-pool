use scytale_cipher::scytale_cipher;

fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
    println!("\"abcd\" size=2 -> {:?}", scytale_cipher(String::from("abcd"), 2));
}