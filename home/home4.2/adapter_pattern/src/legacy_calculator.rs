pub struct LegacyCalculator {
    mega_delta: f64,
    mega_multiplier: f64,
}

impl LegacyCalculator {
    pub fn new(in_mega_delta: f64, in_mega_multiplier: f64) -> Self {
        LegacyCalculator {
            mega_delta: in_mega_delta,
            mega_multiplier: in_mega_multiplier,
        }
    }

    const STRANGE_VAL1: f64 = 6.0;
    const STRANGE_VAL2: f64 = 13.0;

    pub fn calculate_price_part1(&self) -> f64 {
        Self::STRANGE_VAL1 - self.mega_delta
    }

    pub fn calculate_price_part2(&self) -> f64 {
        Self::STRANGE_VAL2 * self.mega_multiplier + 1.0 - self.mega_delta
    }

    pub fn get_our_the_most_and_minimal_value(&self) -> f64 {
        self.mega_delta * self.mega_multiplier
    }

    pub fn get_some_document_representation(&self) -> String {
        format!(
            "The man {} who sold the {} world",
            Self::STRANGE_VAL1 - self.mega_delta,
            self.mega_multiplier * Self::STRANGE_VAL2
        )
    }
}
