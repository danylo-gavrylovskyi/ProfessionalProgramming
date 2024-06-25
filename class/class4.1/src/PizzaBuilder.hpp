#pragma once

#include "Pizza.hpp"

class PizzaBuilder {
    Pizza pizza;
public:
    PizzaBuilder() = default;
    PizzaBuilder(const PizzaBuilder& other) = delete;
    PizzaBuilder(PizzaBuilder&& other) = delete;

    PizzaBuilder& addIngredient(Ingredient&& ingredient);
    Pizza build() const;
};