use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.;
    let mut carbs = 0.;
    let mut proteins = 0.;
    let mut fats = 0.;
    foods.iter().for_each(|food| {
        cals += food.calories.1[..food.calories.1.len() - 4].parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    });

    object! {
        cals : format!("{cals:.2}").parse::<f64>().unwrap() ,
        carbs: format!("{carbs:.2}").parse::<f64>().unwrap(),
        proteins: format!("{proteins:.2}").parse::<f64>().unwrap() ,
        fats: format!("{fats:.2}").parse::<f64>().unwrap()
    }
}
