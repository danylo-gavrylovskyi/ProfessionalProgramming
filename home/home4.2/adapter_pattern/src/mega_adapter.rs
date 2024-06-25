use crate::legacy_calculator::LegacyCalculator;
use crate::modern_calculators::BaseMegaCalculator;

pub struct MegaAdapter {
    legacy_calculator: LegacyCalculator,
}

impl MegaAdapter {
    pub fn new(legacy_calculator: LegacyCalculator) -> Self {
        MegaAdapter { legacy_calculator }
    }
}

impl BaseMegaCalculator for MegaAdapter {
    fn get_price(&self) -> f64 {
        self.legacy_calculator.calculate_price_part1() + self.legacy_calculator.calculate_price_part2()
    }

    fn get_minimal_value(&self) -> f64 {
        self.legacy_calculator.calculate_price_part1().min(self.legacy_calculator.calculate_price_part2())
    }

    fn get_report(&self) -> String {
        let strange_val1 = self.legacy_calculator.calculate_price_part1() + self.legacy_calculator.get_our_the_most_and_minimal_value();
        let strange_val2 = self.legacy_calculator.calculate_price_part2() * self.legacy_calculator.get_our_the_most_and_minimal_value();

        format!("The man who sold {} units for a total value of {:.2}", strange_val1, strange_val2)
    }
}
