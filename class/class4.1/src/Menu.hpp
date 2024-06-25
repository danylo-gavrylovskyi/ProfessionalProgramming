#pragma once

#include <vector>

#include "Ingredient.hpp"

class Menu {
    std::vector<Ingredient> ingredients;
    void loadIngredients(const std::string& pathToFile);
public:
    Menu(const std::string& pathToFile);
    Menu(const Menu& other);
    Menu(Menu&& other);
    std::vector<Ingredient> getIngredients() const;
};