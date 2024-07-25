use super::*;

#[test]
fn test_product_inverse() {
    let x = 2.0;
    let y = 3.0;
    let z = 4.0;
    assert_eq!(calculate_inverse(x, y, z), 1.0 / 24.0);
}

#[test]
fn test_sum_inverse() {
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    assert_eq!(calculate_inverse(x, y, z), 1.0 / 6.0);
}

#[test]
fn test_fallback_expression() {
    let x = 0.0;
    let y = -1.0;
    let z = 1.0;
    assert_eq!(calculate_inverse(x, y, z), 0.0);
}

#[test]
fn test_all_zeros() {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;
    assert_eq!(calculate_inverse(x, y, z), -1.0);
}
