struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Triangle { a, b, c }
    }

    fn calculate_area(&self) -> Option<f64> {
        let s = (self.a + self.b + self.c) / 2.0;
        let result = (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt();
        if result.is_nan() {
            None
        }
        else {
            Some(result)
        }
    }

    fn calculate_height(&self, side: &str) -> Option<f64> {
        match side {
            "a" => Some(2.0 * self.calculate_area().unwrap() / self.a),
            "b" => Some(2.0 * self.calculate_area().unwrap() / self.b),
            "c" => Some(2.0 * self.calculate_area().unwrap() / self.c),
            _ => None
        }
    }
}

#[cfg(test)]
mod test;
