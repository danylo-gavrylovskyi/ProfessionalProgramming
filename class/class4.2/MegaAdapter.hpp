#pragma once
#include "LegacyCalculator.hpp"
#include "ModernCalculators.hpp"

class MegaAdapter : public BaseMegaCalculator {
public:
    MegaAdapter(std::unique_ptr<LegacyCalculator> legacyCalculator)
        : legacyCalculator(std::move(legacyCalculator)) {}

    double getPrice() const override {
        return legacyCalculator->calculatePricePart1() + legacyCalculator->calculatePricePart2();
    }

    double getMinimalValue() const override {
        return std::min(legacyCalculator->calculatePricePart1(), legacyCalculator->calculatePricePart2());
    }

    std::string getReport() const override {
        auto documentRepresentation = legacyCalculator->getSomeDocumentRepresentation();
        double strangeVal1 = legacyCalculator->calculatePricePart1() + legacyCalculator->getOurTheMostAndMinimalValue();
        double strangeVal2 = legacyCalculator->calculatePricePart2() * legacyCalculator->getOurTheMostAndMinimalValue();

        return std::format("The man {} who sold the {} world", strangeVal1, strangeVal2);
    }

private:
    std::unique_ptr<LegacyCalculator> legacyCalculator;
};
