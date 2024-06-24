#include <iostream>
#include <fstream>
#include <unordered_map>
#include <string>
#include <print>

const std::string FILENAME = "user_stats.txt";
const std::string SECRET_WORD = "bread";

void loadStatistics(std::unordered_map<std::string, int>& stats) {
    std::ifstream infile(FILENAME);
    std::string name;
    int count;
    while (infile >> name >> count) {
        stats[name] = count;
    }
}

void saveStatistics(const std::unordered_map<std::string, int>& stats) {
    std::ofstream outfile(FILENAME);
    for (const auto& [name, count] : stats) {
        outfile << name << " " << count << "\n";
    }
}

void clearAllHistory() {
    std::ofstream outfile(FILENAME, std::ofstream::trunc);
}

int main(int argc, char* argv[]) {
    if (argc < 2 || argc > 3) {
        std::println("Error: Invalid number of arguments");
        return 1;
    }

    std::string name = argv[1];
    std::unordered_map<std::string, int> stats;

    loadStatistics(stats);

    if (argc == 3) {
        std::string command = argv[2];
        if (command == "delete") {
            stats.erase(name);
            saveStatistics(stats);
            std::println("Statistics reset for {}", name);
            return 0;
        } else {
            std::println("Error: Invalid command");
            return 1;
        }
    }

    if (name == SECRET_WORD) {
        clearAllHistory();
        std::println("All history cleared");
    } else {
        if (stats.find(name) == stats.end()) {
            stats[name] = 1;
            std::println("Welcome, {}!", name);
        } else {
            stats[name]++;
            std::println("Hello again(x{}), {}", stats[name], name);
        }
        saveStatistics(stats);
    }

    return 0;
}
