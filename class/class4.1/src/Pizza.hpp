#pragma once

#include <vector>

#include "Ingredient.hpp"

class Pizza {
    std::vector<Ingredient> ingredients;
public:
    void addIngredient(Ingredient&& ingredient);

    std::vector<Ingredient> getIngredients() const;
    double getTotalCost() const;
};
