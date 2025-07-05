#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>, pub u32);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            _ => RomanDigit::M,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let v = value ;
        let mut romans = Vec::new();
        if value == 0 {
            return Self(vec![0.into()], value);
        }
        while value > 0 {
            let r: (u32, Option<u32>) = match value {
                1000.. => (1000, None),
                900..1000 => (100, Some(1000)),
                500.. => (500, None),
                400..500 => (100, Some(500)),
                100.. => (100, None),
                90..100 => (10, Some(100)),
                50.. => (50, None),
                40..50 => (10, Some(50)),
                10.. => (10, None),
                9 => (1, Some(10)),
                5.. => (5, None),
                4 => (1, Some(5)),
                _ => (1, None),
            };

            if let Some(a) = r.1 {
                romans.push(r.0.into());
                romans.push(a.into());
                value -= a - r.0;
            } else {
                romans.push(r.0.into());
                value -= r.0;
            }
        }

        Self(romans, v)
    }
}

//...

impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
        Some(RomanNumber::from(self.1 + 1))
    }
}
