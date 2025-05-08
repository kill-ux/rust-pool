pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32) * 5 / 9
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c / (5/9)) + 32
}
