use std::cmp::PartialOrd;

pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

use std::ops::Add;
impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        }
        let last = self.beg;
        self.beg = self.beg + self.step;
        Some(last)
    }
}
