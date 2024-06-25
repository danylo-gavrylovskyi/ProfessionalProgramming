#include "PizzaBuilder.hpp"

PizzaBuilder& PizzaBuilder::addIngredient(Ingredient&& ingredient) {
    this->pizza.addIngredient(std::move(ingredient));
    return *this;
}

Pizza PizzaBuilder::build() const {
    return this->pizza;
}
