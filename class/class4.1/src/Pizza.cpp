#include "Pizza.hpp"

void Pizza::addIngredient(Ingredient&& ingredient) {
    this->ingredients.push_back(std::move(ingredient));
}

std::vector<Ingredient> Pizza::getIngredients() const {
    return this->ingredients;
}

double Pizza::getTotalCost() const {
    double total = 0.0;

    for (const Ingredient& ingredient : this->ingredients) {
        total += ingredient.getCost();
    }
    
    return total;
}
