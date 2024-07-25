pub struct MegaData {
    pub small_array: [f32; 1024],
    pub big_array: [f64; 1024 * 1024],
}

impl MegaData {
    pub fn new() -> Self {
        Self {
            small_array: [42.0; 1024],
            big_array: [42.0; 1024 * 1024],
        }
    }

    pub fn reset(&mut self) {
        self.small_array = [42.0; 1024];
        self.big_array = [42.0; 1024 * 1024];
    }
}
