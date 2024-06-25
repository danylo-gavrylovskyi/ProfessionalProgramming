#include <UnitTests.hpp>

#include "PizzaBuilder.hpp"
#include "Pizza.hpp"
#include "Ingredient.hpp"
#include "Menu.hpp"

int main() {
    UnitTests testSuite;

    testSuite.addTest("should create ingredient",
    [](){
        Ingredient cheese("Cheese", 1.50);
    
        ASSERT_EQ(cheese.getName(), "Cheese");
        ASSERT_EQ(cheese.getCost(), 1.50);
    });

    testSuite.addTest("should add ingredient to pizza",
    [](){
        Pizza pizza;
        Ingredient cheese("Cheese", 1.50);
        pizza.addIngredient(std::move(cheese));
    
        ASSERT_EQ(pizza.getIngredients().size(), 1);
        ASSERT_EQ(pizza.getTotalCost(), 1.50);
    });

    testSuite.addTest("should create menu from file",
    [](){
        Menu menu("data/ingredients.txt");
    
        ASSERT_GT(menu.getIngredients().size(), 0);
    });

    testSuite.addTest("should build pizza with provided ingredients",
    [](){
        Ingredient cheese("Cheese", 1.50);
        Ingredient tomato("Tomato", 2.00);
        PizzaBuilder builder;
        builder.addIngredient(std::move(cheese));
        builder.addIngredient(std::move(tomato));
        Pizza pizza = std::move(builder.build());
    
        ASSERT_EQ(pizza.getIngredients().size(), 2);
        ASSERT_EQ(pizza.getTotalCost(), 3.50);
    });

    testSuite.run();
}
