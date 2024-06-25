#include "Ingredient.hpp"

Ingredient::Ingredient(const std::string& name, double cost): name(name), cost(cost) {}

std::string Ingredient::getName() const {
    return this->name;
}

double Ingredient::getCost() const {
    return this->cost;
}
