#pragma once

#include <vector>
#include <array>

class MegaData {
public:
    std::array<float, 1024> smallArray;
    std::vector<double> bigArray;
    
    MegaData() : bigArray(1024 * 1024, 42.0) {
        reset();
    }

    void reset() {
        smallArray.fill(42.0f);
        std::fill(bigArray.begin(), bigArray.end(), 42.0);
    }
};
