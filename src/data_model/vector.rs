pub struct Vector {
    pub data: Vec<f32>
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Vector { data }
    }

    fn dimension(&self) -> usize {
        self.data.len()
    }

    pub fn euclidean_distance(&self, other: &Vector) -> Option<f32> {
        if self.dimension() != other.dimension() {
            None
        }

        let sum_squared = self.data.iter().zip(other.data.iter()).map(|(a, b)| (a-b).powi(2)).sum::<f32>();
        Some(sum_squared.sqrt())
    }

    pub fn cosine_similarity(&self, other: &Vector) -> Option<f32> {
        if self.dimension() != other.dimension() {
            None
        }

        let dot_product = self.data.iter().zip(other.data.iter()).map(|(a, b)| (a*b)).sum::<f32>();
        let self_magnitude = self.data.iter().map(|(v)| v.powi(2)).sum<f32>().sqrt();
        let other_magnitude = other.data.iter().map(|(v)| v.powi(2)).sum<f32>().sqrt();

        if self_magnitude == 0.0 || other_magnitude == 0.0 {
            None
        }

        Some(dot_product/(self_magnitude*other_magnitude))
    }
}