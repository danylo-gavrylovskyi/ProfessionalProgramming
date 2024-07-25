#include <print>
#include <iostream>

#include "Rectangle.hpp"
#include "RectangleManager.hpp"
#include "UnitTests.hpp"

void unitTests() {
    RectangleManager manager;
    
    Rectangle rect1(3, 4);
    Rectangle rect2(5, 6);
    Rectangle rect3(2, 2);
    
    manager.addRectangle(rect1);
    manager.addRectangle(rect2);
    manager.addRectangle(rect3);
    
    ASSERT_EQ(manager.getBiggestArea(), rect2.calculateArea());

    ASSERT_EQ(manager.getSmallestArea(), rect3.calculateArea());

    ASSERT_EQ(manager.getTotalArea(), (rect1.calculateArea() + rect2.calculateArea() + rect3.calculateArea()));

    ASSERT_EQ(manager.getBiggestSide(), rect2.getHeight());

    std::println("All tests passed!\n");
}

int main() {
    RectangleManager manager;

    for (int i = 1; i <= 5; ++i) {
        double width, height;
        std::println("Enter rectangle {} (width height): ", i);
        if (std::cin >> width >> height) {
            manager.addRectangle(Rectangle(width, height));
        }
    }

    manager.checkPlacementPossibilities();

    std::println("The biggest area: {}\n", manager.getBiggestArea());
    std::println("The smallest area: {}\n", manager.getSmallestArea());
    std::println("The biggest side of any rectangle: {}", manager.getBiggestSide());
    std::println("Total area of rectangles: {}", manager.getTotalArea());

    unitTests();

    return 0;
}
