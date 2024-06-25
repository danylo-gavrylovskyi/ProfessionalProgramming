use crate::ingredient::Ingredient;
use crate::pizza::Pizza;

pub struct PizzaBuilder {
    pizza: Pizza,
}

impl PizzaBuilder {
    pub fn new() -> Self {
        Self {
            pizza: Pizza::new(),
        }
    }

    pub fn add_ingredient(mut self, ingredient: Ingredient) -> Self {
        self.pizza.add_ingredient(ingredient);
        self
    }

    pub fn build(self) -> Pizza {
        self.pizza
    }
}
