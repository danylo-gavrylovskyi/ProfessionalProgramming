#pragma once

#include <string>
#include <vector>
#include <random>

class BookRecommendation {
public:
    BookRecommendation();
    std::string getRecommendation();
    void removeBook(const std::string& book);

private:
    std::vector<std::string> books;
    std::default_random_engine generator;
};
