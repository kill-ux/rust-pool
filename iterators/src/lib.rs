use std::iter::Enumerate;

#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let cc: Collatz = *self;
        if self.v == 0 || self.v == 1 {
            None
        } else {
            if self.v % 2 == 0 {
                self.v /= 2;
            } else {
                self.v = self.v * 3 + 1;
            }
            Some(cc)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n}
    }
}

pub fn collatz(n: u64) -> usize {
    let collat = Collatz::new(n);
    collat.count()
}
