#include "RectangleManager.hpp"

#include <print>
#include <algorithm>
#include <limits>

void RectangleManager::addRectangle(const Rectangle& rectangle) {
    rectangles.push_back(rectangle);
}

double RectangleManager::getBiggestArea() const {
    double biggestArea = 0;
    for (const auto& rect : rectangles) {
        double area = rect.calculateArea();
        if (area > biggestArea) {
            biggestArea = area;
        }
    }
    return biggestArea;
}

double RectangleManager::getSmallestArea() const {
    double smallestArea = std::numeric_limits<double>::max();
    for (const auto& rect : rectangles) {
        double area = rect.calculateArea();
        if (area < smallestArea) {
            smallestArea = area;
        }
    }
    return smallestArea;
}

double RectangleManager::getTotalArea() const {
    double totalArea = 0;
    for (const auto& rect : rectangles) {
        totalArea += rect.calculateArea();
    }
    return totalArea;
}

double RectangleManager::getBiggestSide() const {
    double biggestSide = 0;
    for (const auto& rect : rectangles) {
        double maxSide = std::max(rect.getWidth(), rect.getHeight());
        if (maxSide > biggestSide) {
            biggestSide = maxSide;
        }
    }
    return biggestSide;
}

bool RectangleManager::canPlaceInside(const Rectangle& inner, const Rectangle& outer) const {
    return inner.getWidth() <= outer.getWidth() && inner.getHeight() <= outer.getHeight();
}

void RectangleManager::checkPlacementPossibilities() const {
    for (size_t i = 0; i < rectangles.size(); ++i) {
        for (size_t j = 0; j < rectangles.size(); ++j) {
            if (i != j && canPlaceInside(rectangles[i], rectangles[j])) {
                std::println("Rectangle {} can be placed inside rectangle {}\n", i + 1, j + 1);
            }
        }
    }
}
