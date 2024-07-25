#pragma once

class Rectangle {
public:
    Rectangle(double width, double height);
    
    double calculateArea() const;
    double getWidth() const;
    double getHeight() const;

private:
    double width;
    double height;
};
