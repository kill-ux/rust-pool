#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub prices: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Self { prices: vec![] }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        self.prices.push(
            s.products
                .iter()
                .find(|(name, _)| name == &ele)
                .map(|(_, prise)| *prise)
                .unwrap_or(0.),
        );
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.prices.sort_by(|a,b| a.partial_cmp(b).unwrap());
        let num = self.prices.len() / 3 ;
        let mut pers = 0.;
        let sums = self.prices.iter().sum::<f32>();
        let mut copy_pers = vec![];
        for i in 0..num {
            pers += self.prices[i] ;
        }

        for ele in &self.prices {
            copy_pers.push((ele*100.)/sums);
        }

        let sums = sums - pers ;
        for (i,ele) in copy_pers.iter().enumerate() {
            self.prices[i] = format!("{:.2}",(ele * sums) / 100.).parse::<f32>().unwrap();
        }
        self.prices.clone()
    }
}
