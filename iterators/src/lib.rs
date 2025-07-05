#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            None
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(self.v)
        } else {
            self.v = (self.v * 3) + 1;
            Some(self.v)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut collat = Collatz::new(n);
    let mut step = 0 ;
    while let Some(nbr) = collat.next() {
        step += 1;
    }
    step
}
