use pizza_builder::ingredient::Ingredient;
use pizza_builder::menu::Menu;
use pizza_builder::pizza_builder::PizzaBuilder;
use pizza_builder::pizza::Pizza;

fn main() {
    let menu = Menu::new("src/data/ingredients.txt");

    println!("Do you want to choose a classical pizza or compose a custom one?");
    println!("1. Classical Pizza");
    println!("2. Custom Pizza");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<u32>().expect("Invalid input");

    let pizza = match choice {
        1 => choose_classical_pizza(),
        2 => compose_custom_pizza(&menu),
        _ => {
            println!("Invalid choice, exiting.");
            return;
        }
    };

    print_pizza_summary(&pizza);
}

fn choose_classical_pizza() -> Pizza {
    let mut pizza_builder = PizzaBuilder::new();
    pizza_builder = pizza_builder.add_ingredient(Ingredient::new("Cheese", 1.50));
    pizza_builder = pizza_builder.add_ingredient(Ingredient::new("Tomato", 0.75));
    pizza_builder.build()
}

fn compose_custom_pizza(menu: &Menu) -> Pizza {
    let mut pizza_builder = PizzaBuilder::new();

    println!("Available ingredients:");
    for ingredient in menu.get_ingredients() {
        println!("- {}: ${:.2}", ingredient.get_name(), ingredient.get_cost());
    }

    loop {
        println!("Enter ingredient name (or 'done' to finish):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "done" {
            break;
        }

        if let Some(ingredient) = menu.get_ingredients().iter().find(|&i| i.get_name() == input) {
            pizza_builder = pizza_builder.add_ingredient(ingredient.clone());
        } else {
            println!("Ingredient not found. Please try again.");
        }
    }

    pizza_builder.build()
}

fn print_pizza_summary(pizza: &Pizza) {
    println!("Your pizza contains:");
    for ingredient in pizza.get_ingredients() {
        println!("- {}: ${:.2}", ingredient.get_name(), ingredient.get_cost());
    }
    println!("Total cost: ${:.2}", pizza.get_total_cost());
}

#[cfg(test)]
mod test;
