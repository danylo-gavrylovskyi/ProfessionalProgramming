#include "Menu.hpp"
#include <fstream>
#include <sstream>
#include <stdexcept>

Menu::Menu(const std::string& filePath) {
    loadIngredients(filePath);
}

void Menu::loadIngredients(const std::string& filePath) {
    std::ifstream file(filePath);
    if (!file) {
        throw std::runtime_error("Could not open file");
    }

    std::string line;
    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::string name;
        double cost;
        if (iss >> name >> cost) {
            ingredients.emplace_back(name, cost);
        }
    }
}

std::vector<Ingredient> Menu::getIngredients() const {
    return ingredients;
}
