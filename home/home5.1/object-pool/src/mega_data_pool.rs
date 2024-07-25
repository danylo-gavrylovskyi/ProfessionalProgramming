use crate::mega_data::MegaData;

pub struct MegaDataPool {
    pub pool: Vec<MegaData>,
    pub available_indices: Vec<usize>,
}

impl MegaDataPool {
    pub fn new(pool_size: usize) -> Self {
        let mut pool = Vec::with_capacity(pool_size);
        let mut available_indices = Vec::with_capacity(pool_size);

        for i in 0..pool_size {
            pool.push(MegaData::new());
            available_indices.push(i);
        }

        Self {
            pool,
            available_indices
        }
    }

    pub fn acquire(&mut self) -> Option<&mut MegaData> {
        if let Some(index) = self.available_indices.pop() {
            Some(&mut self.pool[index])
        }
        else {
            None
        }
    }

    pub fn release(&mut self, data: &mut MegaData) {
        let index = self.pool.iter_mut().position(|d| std::ptr::eq(d, data));
        if let Some(index) = index {
            self.available_indices.push(index);
        }
    }

    pub fn size(&self) -> usize {
        self.pool.len()
    }

    pub fn used_size(&self) -> usize {
        self.pool.len() - self.available_indices.len()
    }
}
