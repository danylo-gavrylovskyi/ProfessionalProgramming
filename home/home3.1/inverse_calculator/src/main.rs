fn calculate_inverse(x: f64,y: f64,z: f64) -> f64 {
    let product = x * y * z;
    if product != 0.0 {
        return 1.0 / product;
    }

    let sum = x + y + z;
    if sum != 0.0 {
        return 1.0 / sum;
    }

    x + (y + 1.0) * (z - 1.0)
}

fn main() {
    let result = calculate_inverse(2.0, 3.0, 4.0);
    println!("Result: {}", result);
}

#[cfg(test)]
mod test;
