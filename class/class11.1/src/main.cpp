#include "BookRecommendation.hpp"
#include <iostream>
#include <print>
#include <string>

int main() {
    BookRecommendation bookRecommender;

    try {
        while (true) {
            std::string recommendation = bookRecommender.getRecommendation();
            std::println("How about: {}?", recommendation);

            std::string response;
            std::println("Do you want this book? (yes/no): ");
            std::cin >> response;

            if (response == "yes") {
                std::println("Enjoy your book!");
                break;
            } else if (response == "no") {
                bookRecommender.removeBook(recommendation);
            } else {
                std::println("Invalid response. Please answer 'yes' or 'no'.");
            }
        }
    } catch (const std::runtime_error& e) {
        std::println("{}", e.what());
        return -1;
    }

    return 0;
}
