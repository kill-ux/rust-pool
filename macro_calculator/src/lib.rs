pub use json::*;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
    for food in foods {
        cals += food.calories.1[..food.calories.1.len() - 4]
            .parse::<f64>()
            .unwrap()
            * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }
    object! {
        cals : format!("{:.2}",cals).parse::<f64>().unwrap(),
        carbs :  format!("{:.2}",carbs).parse::<f64>().unwrap(),
        proteins :  format!("{:.2}",proteins).parse::<f64>().unwrap(),
        fats :  format!("{:.2}",fats).parse::<f64>().unwrap(),
    }
}
