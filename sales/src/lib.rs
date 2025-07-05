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
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Self { receipt: vec![] }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        self.receipt.push(
            s.products
                .iter()
                .find(|(name, _)| name == &ele)
                .map(|(_, prise)| *prise)
                .unwrap_or(0.),
        );
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt.sort_by(|a,b| a.partial_cmp(b).unwrap());
        let num = self.receipt.len() / 3 ;
        let mut pers = 0.;
        let sums = self.receipt.iter().sum::<f32>();
        let mut copy_pers = vec![];
        for i in 0..num {
            pers += self.receipt[i] ;
        }

        for ele in &self.receipt {
            copy_pers.push((ele*100.)/sums);
        }

        let sums: f32 = sums - pers ;
        for (i,ele) in copy_pers.iter().enumerate() {
            self.receipt[i] = format!("{:.2}",(ele * sums) / 100.).parse::<f32>().unwrap();
        }
        self.receipt.clone()
    }
}

