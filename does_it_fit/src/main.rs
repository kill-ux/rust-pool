use does_it_fit::*;

fn main() {
    assert!(!area_fit((3, 5), GeometricalShapes::Circle, 2, (2, 0)));
    assert!(area_fit((8, 6), GeometricalShapes::Triangle, 5, (5, 2)));
    assert!(area_fit((7, 3), GeometricalShapes::Rectangle, 2, (2, 4)));
    println!("ll");
}
