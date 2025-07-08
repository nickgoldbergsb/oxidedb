#[derive(Debug)]
pub struct Vector {
    pub data: Vec<f32>
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    fn dimension(&self) -> usize {
        self.data.len()
    }
}