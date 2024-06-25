#include "../include/Helpers.hpp"

double Triangle::calculateArea() {
    double s = (a + b + c) / 2;
    double result = sqrt(s*(s-a)*(s-b)*(s-c));
    return result;
}

bool isColorValid(int color) {
    if (color > 255 || color < 0) return false;
    return true;
}

std::array<int, 3> generateMagicColor(int r, int g, int b) {
    if (!isColorValid(r) || !isColorValid(g) || !isColorValid(b)) {
        throw std::invalid_argument("Values should be in range 0-255.\n");
    }

    std::array<int, 3> magicColor;
    magicColor[0] = (r / 2 - 1) < 0 ? 0 : r / 2 - 1;
    magicColor[1] = (g * 2 - 2) > 255 ? 255 : g * 2 - 2;
    magicColor[2] = b;
    return magicColor;
}
