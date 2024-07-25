#include <UnitTests.hpp>

#include "MegaData.hpp"
#include "MegaDataPool.hpp"

int main() {
    UnitTests testSuite;

    testSuite.addTest("should create MegaData with default values",
    [](){
        MegaData data;
        for (const auto& val : data.smallArray) {
            ASSERT_EQ(val, 42.0f);
        }
        for (const auto& val : data.bigArray) {
            ASSERT_EQ(val, 42.0);
        }
    });

    testSuite.addTest("should reset MegaData to default values",
    [](){
        MegaData data;
        data.smallArray[0] = 0.0f;
        data.bigArray[0] = 0.0;
        data.reset();
        ASSERT_EQ(data.smallArray[0], 42.0f);
        ASSERT_EQ(data.bigArray[0], 42.0);
    });

    testSuite.addTest("should initialize MegaDataPool with correct size",
    [](){
        MegaDataPool pool(10);
        ASSERT_EQ(pool.size(), 10);
        ASSERT_EQ(pool.usedSize(), 0);
        ASSERT_EQ(pool.availableSize(), 10);
    });

    testSuite.addTest("should acquire and release MegaData objects",
    [](){
        MegaDataPool pool(10);
        auto& data = pool.acquire();
        ASSERT_EQ(pool.usedSize(), 1);
        ASSERT_EQ(pool.availableSize(), 9);

        pool.release(data);
        ASSERT_EQ(pool.usedSize(), 0);
        ASSERT_EQ(pool.availableSize(), 10);
    });

    testSuite.addTest("should throw when pool is exhausted",
    [](){
        MegaDataPool pool(1);
        pool.acquire();
        try {
            pool.acquire();
            return false;
        } catch (const std::runtime_error& e) {
            return true;
        }
    });

    testSuite.run();
}
