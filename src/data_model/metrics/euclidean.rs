use super::metric::SimilarityMetric;
use super::vector::Vector;

pub struct Euclidean;

impl SimilarityMetric for Euclidean {
    fn compute(v1: &Vector, v2: &Vector) -> Option<f32> {
        if self.dimension() != other.dimension() {
            None
        }

        let sum_squared = self.data.iter().zip(other.data.iter()).map(|(a, b)| (a-b).powi(2)).sum::<f32>();
        Some(sum_squared.sqrt())
    }
}