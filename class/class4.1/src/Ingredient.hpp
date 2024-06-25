#pragma once

#include <string>

class Ingredient {
    std::string name;
    double cost;
public:
    Ingredient(const std::string& name, double cost);
    // Ingredient(const Ingredient& other) = delete;
    // Ingredient(Ingredient&& other) = delete;

    double getCost() const;
    std::string getName() const;
};
