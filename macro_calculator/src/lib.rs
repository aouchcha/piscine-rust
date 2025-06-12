use json::object;

pub struct Food {
    pub name : String,
    pub calories : (String, String),
    pub proteins : f64,
    pub fats : f64,
    pub carbs : f64,
    pub nbr_of_portions : f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals : f64 = 0.0;
    let mut carbs : f64 = 0.0;
    let mut proteins : f64 = 0.0;
    let mut fats : f64 = 0.0;
    for food in foods {
        let c : f64 = food.calories.1[..food.calories.1.len() - 4].parse().unwrap_or(0.0);
        cals += food.nbr_of_portions * c;
        carbs += food.nbr_of_portions * food.carbs;
        proteins += food.nbr_of_portions * food.proteins;
        fats += food.nbr_of_portions * food.fats;
    }
    object! {
        "cals" =>  (cals * 100.0).round() / 100.0,
        "carbs" =>  (carbs * 100.0).round() / 100.0,
        "proteins" =>  (proteins * 100.0).round() / 100.0,
        "fats" =>  (fats * 100.0).round() / 100.0,
    }
}