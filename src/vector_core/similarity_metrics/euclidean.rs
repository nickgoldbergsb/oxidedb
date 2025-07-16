use super::super::vector::Vector;
use super::metric::SimilarityMetric;

pub struct Euclidean;

impl SimilarityMetric for Euclidean {
    fn compute(&self, v1: &Vector, v2: &Vector) -> Option<f32> {
        if v1.dimension() != v2.dimension() {
            return None;
        }

        let sum_squared = v1
            .data
            .iter()
            .zip(v2.data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>();
        Some(sum_squared.sqrt())
    }
}
