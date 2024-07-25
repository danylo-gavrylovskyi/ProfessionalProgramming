#pragma once

#include <vector>
#include <queue>
#include <ranges>
#include <stdexcept>
#include <cstddef>
#include <mutex>

#include "MegaData.hpp"

class MegaDataPool {
    size_t poolSize;
    size_t available;
    size_t used;
    std::vector<MegaData> pool;
    std::queue<size_t> availableIndices;
    std::mutex mtx;

    static MegaDataPool* instance;
    static std::mutex instanceMtx;

    explicit MegaDataPool(size_t poolSize);

public:
    MegaDataPool(const MegaDataPool&) = delete;
    MegaDataPool& operator=(const MegaDataPool&) = delete;

    static MegaDataPool& getInstance(size_t poolSize = 10);

    MegaData& acquire();

    void release(MegaData& data);

    size_t size() const;
    size_t availableSize() const;
    size_t usedSize() const;
};
