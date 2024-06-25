use super::*;

#[test]
fn test_triangle_area_1() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert_eq!(triangle.calculate_area().unwrap(), 6.0);
}

#[test]
fn test_triangle_area_2() {
    let triangle = Triangle::new(3.0, 0.0, 5.0);
    assert_eq!(triangle.calculate_area(), None);
}

#[test]
fn test_triangle_area_3() {
    let triangle = Triangle::new(3.0, -2.0, 5.0);
    assert_eq!(triangle.calculate_area().unwrap(), 0.0);
}

#[test]
fn test_triangle_height_a() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert_eq!(triangle.calculate_height("a").unwrap(), 4.0);
}

#[test]
fn test_triangle_height_b() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert_eq!(triangle.calculate_height("b").unwrap(), 3.0);
}

#[test]
fn test_triangle_height_c() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert_eq!(triangle.calculate_height("c").unwrap(), 2.4);
}

#[test]
fn test_triangle_invalid_side() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert_eq!(triangle.calculate_height("d"), None);
}
