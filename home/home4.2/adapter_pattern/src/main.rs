use adapter_pattern::legacy_calculator::LegacyCalculator;
use adapter_pattern::mega_adapter::MegaAdapter;
use adapter_pattern::modern_calculators::BaseMegaCalculator;
use adapter_pattern::modern_calculators::ConstantCalculator;
use adapter_pattern::modern_calculators::MyCoolCalculator;

fn print_results(calculator: &dyn BaseMegaCalculator) {
    println!("Current price: {:.2}", calculator.get_price());
    println!("Current minimal value: {:.2}", calculator.get_minimal_value());
    println!("Current report: {}", calculator.get_report());
}

fn main() {
    let my_cool_calculator = MyCoolCalculator::new(6.0, 12.1, 3.2);
    print_results(&my_cool_calculator);

    let constant_calculator = ConstantCalculator;
    print_results(&constant_calculator);

    let legacy_calculator = LegacyCalculator::new(1.34, 5.4);
    let adapter = MegaAdapter::new(legacy_calculator);
    print_results(&adapter);
}
