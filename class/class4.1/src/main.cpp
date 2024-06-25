#include <iostream>
#include <print>

#include "Menu.hpp"
#include "PizzaBuilder.hpp"

int main() {
    Menu menu("data/ingredients.txt");
    PizzaBuilder builder;

    std::println("Available ingredients:\n");
    for (const auto& ingredient : menu.getIngredients()) {
        std::println("- {}: ${}\n", ingredient.getName(), ingredient.getCost());
    }

    std::string input;
    while (true) {
        std::println("Enter ingredient name (or 'done' to finish): ");
        std::cin >> input;
        if (input == "done") {
            break;
        }

        bool found = false;
        for (auto& ingredient : menu.getIngredients()) {
            if (ingredient.getName() == input) {
                builder.addIngredient(std::move(ingredient));
                found = true;
                break;
            }
        }

        if (!found) {
            std::println("Ingredient not found. Please try again.\n");
        }
    }

    Pizza pizza = builder.build();
    std::println("Your pizza contains:\n");
    for (const auto& ingredient : pizza.getIngredients()) {
        std::println("- {}: ${}\n", ingredient.getName(), ingredient.getCost());
    }
    std::println("Total cost: ${}\n", pizza.getTotalCost());

    return 0;
}
