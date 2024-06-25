pub trait BaseMegaCalculator {
    fn get_price(&self) -> f64;
    fn get_minimal_value(&self) -> f64;
    fn get_report(&self) -> String;
}

pub struct ConstantCalculator;

impl BaseMegaCalculator for ConstantCalculator {
    fn get_price(&self) -> f64 {
        0.0
    }

    fn get_minimal_value(&self) -> f64 {
        0.0
    }

    fn get_report(&self) -> String {
        "Sound of Silence".to_string()
    }
}

pub struct MyCoolCalculator {
    coef1: f64,
    coef2: f64,
    coef3: f64,
}

impl MyCoolCalculator {
    pub fn new(in_coef1: f64, in_coef2: f64, in_coef3: f64) -> Self {
        Self {
            coef1: in_coef1,
            coef2: in_coef2,
            coef3: in_coef3
        }
    }

    const VAL1: f64 = 12.0;
    const VAL2: f64 = 3.0;
    const VAL3: f64 = 0.05;
}

impl BaseMegaCalculator for MyCoolCalculator {
    fn get_price(&self) -> f64 {
        self.coef1 * Self::VAL1 + self.coef2 * Self::VAL2 - self.coef3 * Self::VAL3 
    }

    fn get_minimal_value(&self) -> f64 {
        (self.coef1 * Self::VAL1).min(self.coef2 * Self::VAL2)
    }

    fn get_report(&self) -> String {
        format!(
            "Some1 {}, another2 {:.3}, and3 {:.2}",
            self.coef1,
            self.coef2,
            self.coef3 * Self::VAL2
        )
    }
}
