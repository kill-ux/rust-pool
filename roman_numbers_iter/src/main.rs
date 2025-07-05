pub use crate::RomanDigit::*;
use roman_numbers_iter::{RomanDigit, RomanNumber};

fn main() {
    let mut number = RomanNumber::from(15);

    println!("{:?}", number);
    println!("{:?}", number.next());
}
