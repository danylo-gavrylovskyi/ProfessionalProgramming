#include "Rectangle.hpp"

Rectangle::Rectangle(double width, double height) : width(width), height(height) {}

double Rectangle::calculateArea() const {
    return width * height;
}

double Rectangle::getWidth() const {
    return width;
}

double Rectangle::getHeight() const {
    return height;
}
