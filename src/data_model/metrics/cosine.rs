use super::metric::SimilarityMetric;
use super::vector::Vector;

pub struct Cosine;

impl SimilarityMetric for Cosine {
    fn compute(v1: &Vector, v2: &Vector) -> Option<f32> {
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