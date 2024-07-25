#include "MegaDataPool.hpp"

MegaDataPool::MegaDataPool(size_t poolSize)
    : poolSize(poolSize), available(poolSize), used(0) {
    for (size_t i = 0; i < poolSize; ++i) {
        pool.emplace_back();
        availableIndices.push(i);
    }    
}

MegaData& MegaDataPool::acquire() {
    if (available == 0){
        throw std::runtime_error("No available MegaData objects in the pool");
    }

    size_t index = availableIndices.front();
    availableIndices.pop();
    --available;
    ++used;

    return pool[index];
}

void MegaDataPool::release(MegaData& data) {
    auto it = std::ranges::find_if(pool, [&data](const MegaData& d) { return &d == &data; });
    if (it != pool.end()) {
        size_t index = std::distance(pool.begin(), it);
        availableIndices.push(index);
        --used;
        ++available;
    }
}

size_t MegaDataPool::size() const {
    return this->poolSize;
}
size_t MegaDataPool::usedSize() const {
    return this->used;
}
size_t MegaDataPool::availableSize() const {
    return this->available;
}
