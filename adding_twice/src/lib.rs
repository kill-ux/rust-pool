pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
    move |b: i32| a + b
}

pub fn twice<T: Fn(i32) -> i32>(F: T) -> impl Fn(i32) -> i32 {
    move |b| F(F(b))
}
