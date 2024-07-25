#pragma once

#include <vector>
#include <queue>
#include <ranges>
#include <stdexcept>
#include <cstddef>

#include "MegaData.hpp"

class MegaDataPool {
    size_t poolSize;
    size_t available;
    size_t used;
    std::vector<MegaData> pool;
    std::queue<size_t> availableIndices;
public:
    explicit MegaDataPool(size_t poolSize);

    MegaData& acquire();

    void release(MegaData& data);

    size_t size() const;
    size_t availableSize() const;
    size_t usedSize() const;
};
