#include <cmath>

#include "../include/Helpers.hpp"
#include "../include/UnitTests.hpp"

int main() {
    UnitTests testSuite;

    testSuite.addTest("triangleAreaTest1",
    [](){
        Triangle triangle(3,4,5);
    
        double result = triangle.calculateArea();

        ASSERT_EQ(result, 6);
    });

    testSuite.addTest("triangleAreaTest2",
    [](){
        Triangle triangle(3,0,5);
    
        double result = triangle.calculateArea();

        ASSERT_EQ(std::isnan(result), true);
    });

    testSuite.addTest("triangleAreaTest3",
    [](){
        Triangle triangle(3,-2,5);
    
        double result = triangle.calculateArea();

        ASSERT_EQ(result, 0);
    });

    testSuite.addTest("magicColorGeneratorTest1", 
    [](){
        try {
            generateMagicColor(300, 123, 4);
        } catch (const std::invalid_argument&) {
            return true; 
        }
        return false; 
    });

    testSuite.addTest("magicColorGeneratorTest2",
    [](){
        std::array<int, 3> result = generateMagicColor(100, 50, 200);

        ASSERT_EQ(result[0], 49);
        ASSERT_EQ(result[1], 98);
        ASSERT_EQ(result[2], 200);
    });

    testSuite.addTest("magicColorGeneratorTest3",
    [](){
        std::array<int, 3> result = generateMagicColor(255, 128, 0);

        ASSERT_EQ(result[0], 126);
        ASSERT_EQ(result[1], 254);
        ASSERT_EQ(result[2], 0);
    });

    testSuite.run();
}
