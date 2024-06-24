#include "Helpers.hpp"

double dummyFunc(double x) {
    return x > 0 ? x : 0;
}

std::string stringRepeater(const std::vector<int>& numbers, const std::string& line) {
    if (numbers.empty() || line == "") return "";

    std::string result = "";
    auto biggestNum = std::max_element(numbers.begin(), numbers.end());

    for (int i = 0; i < *biggestNum; i++) {
        result += line;
    }

    return result;
}
