use crate::ingredient::Ingredient;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Menu {
    ingredients: Vec<Ingredient>,
}

impl Menu {
    pub fn new(file_path: &str) -> Self {
        let mut menu = Self {
            ingredients: Vec::new(),
        };
        menu.load_ingredients(file_path);
        menu
    }

    fn load_ingredients(&mut self, file_path: &str) {
        let file = File::open(file_path).expect("Could not open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Could not read line");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let name = parts[0];
                let cost = parts[1].parse::<f64>().expect("Could not parse cost");
                self.ingredients.push(Ingredient::new(name, cost));
            }
        }
    }

    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }
}
