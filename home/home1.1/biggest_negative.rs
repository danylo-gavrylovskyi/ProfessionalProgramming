use std::io;

fn main() {
    println!("Input a list of numbers separated by commas: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let numbers: Vec<f64> = input
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    let largest_negative = numbers
        .iter()
        .filter(|&&n| n < 0.0)
        .max_by(|a, b| a.partial_cmp(b).unwrap());

    match largest_negative {
        Some(&value) => println!("The largest negative value is {}", value),
        None => println!("No negative numbers in the list"),
    }
}