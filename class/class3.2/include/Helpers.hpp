#pragma once

#include <cstdint>
#include <cmath>
#include <array>
#include <iostream>

class Triangle {
    double a, b, c;
public:
    Triangle(double a, double b, double c): a(a), b(b), c(c) {}
    Triangle(const Triangle& other) = delete;
    Triangle(Triangle&& other) = delete;

    double calculateArea();
};

bool isColorValid(int color);
std::array<int, 3> generateMagicColor(int r, int g, int b);
