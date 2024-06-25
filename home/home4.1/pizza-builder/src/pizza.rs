use crate::ingredient::Ingredient;

pub struct Pizza {
    ingredients: Vec<Ingredient>,
}

impl Pizza {
    pub fn new() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }

    pub fn get_total_cost(&self) -> f64 {
        self.ingredients.iter().map(|i| i.get_cost()).sum()
    }
}
