#include "Helpers.hpp"

double dummyFunc(double x) {
    return x > 0 ? x : 0;
}

std::string stringRepeater(const std::vector<int>& numbers, const std::string& line) {
    auto repeatCount = numbers.empty() ? 0 : std::ranges::max(numbers);
    return repeatCount > 0 ? std::ranges::views::repeat(line, repeatCount) | std::views::join | std::ranges::to<std::string>() : "";
}
