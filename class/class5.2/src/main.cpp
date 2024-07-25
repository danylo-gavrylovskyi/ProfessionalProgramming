#include <print>
#include <thread>
#include <vector>
#include <iostream>

#include "MegaDataPool.hpp"

// Helper functions
MegaData& acquireFromPool() {
    return MegaDataPool::getInstance().acquire();
}

void releaseToPool(MegaData& data) {
    MegaDataPool::getInstance().release(data);
}

size_t getPoolSize() {
    return MegaDataPool::getInstance().size();
}

void threadFunction(int threadId) {
    try {
        MegaData& data = acquireFromPool();
        std::println("Thread {} acquired an object\n", threadId);

        std::this_thread::sleep_for(std::chrono::milliseconds(100));

        releaseToPool(data);
        std::println("Thread {} released an object\n", threadId);
    } catch (const std::exception& e) {
        std::cerr << "Thread " << threadId << ": " << e.what() << '\n';
    }
}

int main() {
    constexpr int numThreads = 20;

    std::vector<std::thread> threads;

    for (int i = 0; i < numThreads; ++i) {
        threads.emplace_back(threadFunction, i);
    }

    for (auto& thread : threads) {
        thread.join();
    }

    std::println("Final Pool Size: {}\n", getPoolSize());
    std::println("Final Used Size: {}\n", MegaDataPool::getInstance().usedSize());
    std::println("Final Available Size: {}\n", MegaDataPool::getInstance().availableSize());

    return 0;
}
