#pragma once

#include <vector>

#include "Rectangle.hpp"

class RectangleManager {
public:
    void addRectangle(const Rectangle& rectangle);
    double getBiggestArea() const;
    double getSmallestArea() const;
    double getTotalArea() const;
    double getBiggestSide() const;
    bool canPlaceInside(const Rectangle& inner, const Rectangle& outer) const;
    void checkPlacementPossibilities() const;

private:
    std::vector<Rectangle> rectangles;
};
