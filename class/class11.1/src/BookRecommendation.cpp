#include "BookRecommendation.hpp"

#include <chrono>
#include <algorithm>
#include <stdexcept>

BookRecommendation::BookRecommendation() {
    books = {
        "The Topographer’s Clown",
        "The Chamber of Beaver",
        "The Ironer of Kanban",
        "The Piglet of Tire",
        "The Border of the Unix",
        "The Half-Time Convince",
        "The Earthly Pillows",
        "The Censorship of the Ping",
        "The True Powers",
        "The Overturn of the Ling"
    };
    generator.seed(std::chrono::system_clock::now().time_since_epoch().count());
}

std::string BookRecommendation::getRecommendation() {
    if (books.empty()) {
        throw std::runtime_error("Adios amigo!");
    }
    std::uniform_int_distribution<int> distribution(0, books.size() - 1);
    int index = distribution(generator);
    return books[index];
}

void BookRecommendation::removeBook(const std::string& book) {
    books.erase(std::remove(books.begin(), books.end(), book), books.end());
}
