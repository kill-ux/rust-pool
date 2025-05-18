use expected_variable::*;

fn main() {
    println!(
        "{} close to it",
        expected_variable("lets_try", "lets_try_it").unwrap()
    );

}