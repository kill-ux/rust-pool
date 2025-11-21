
// curry function accept number and add it to the curry callback argumment
pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
    move |b: i32| a + b
}
