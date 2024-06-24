#include <Helpers.hpp>
#include <UnitTests.hpp>

int main() {
    UnitTests testSuite;

    testSuite.addTest("Dummy_test1",
    [](){
        // Build:
        double value = -1;
    
        // Operate:
        auto result = dummyFunc(value);

        // Check:
        ASSERT_EQ(result, 0);
    });

    testSuite.addTest("Dummy_test2",
    [](){
        // Build:
        double value = 2.2;
    
        // Operate:
        auto result = dummyFunc(value);

        // Check:
        ASSERT_EQ(result, 2.2);
    });

    testSuite.addTest("Dummy_test3",
    [](){
        // Build:
        double value = 0.0;
    
        // Operate:
        auto result = dummyFunc(value);

        // Check:
        ASSERT_EQ(result, 0.0);
    });

    testSuite.addTest("StringRepeaterTest1",
    [](){
        std::vector<int> v = {1, 2, 3};
        std::string s = "test";
    
        auto result = stringRepeater(v, s);

        ASSERT_EQ(result, "testtesttest");
    });

    testSuite.addTest("StringRepeaterTest2",
    [](){
        std::vector<int> v = {};
        std::string s = "test";
    
        auto result = stringRepeater(v, s);

        ASSERT_EQ(result, "");
    });

    testSuite.addTest("StringRepeaterTest3",
    [](){
        std::vector<int> v = {1, 2, 3};
        std::string s = "";
    
        auto result = stringRepeater(v, s);

        ASSERT_EQ(result, "");
    });

    testSuite.addTest("StringRepeaterTest4",
    [](){
        std::vector<int> v = {2, 4, 1};
        std::string s = "test";
    
        auto result = stringRepeater(v, s);

        ASSERT_EQ(result, "testtesttesttest");
    });

    testSuite.addTest("StringRepeaterTest5",
    [](){
        std::vector<int> v = {0};
        std::string s = "test";
    
        auto result = stringRepeater(v, s);

        ASSERT_EQ(result, "")
    });

    testSuite.run();
}
