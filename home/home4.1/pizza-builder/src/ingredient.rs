#[derive(Clone)]
pub struct Ingredient {
    name: String,
    cost: f64,
}

impl Ingredient {
    pub fn new(name: &str, cost: f64) -> Self {
        Self {
            name: name.to_string(),
            cost
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_cost(&self) -> f64 {
        self.cost
    }
}
