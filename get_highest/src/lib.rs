#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        }
        self.numbers.get(self.numbers.len() - 1).copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut new_vec = self.numbers.to_vec();
        new_vec.sort_by(|a, b| b.cmp(a));
        new_vec.iter().take(3).copied().collect()
    }
}
