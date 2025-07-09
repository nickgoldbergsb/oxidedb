use super::metric::SimilarityMetric;
use super::super::vector::Vector;

pub struct Cosine;

impl SimilarityMetric for Cosine {
    fn compute(&self, v1: &Vector, v2: &Vector) -> Option<f32> {
        if v1.dimension() != v2.dimension() {
            return None;
        }

        let dot_product = v1.data.iter().zip(v2.data.iter()).map(|(a, b)| (a*b)).sum::<f32>();
        let self_magnitude = v1.data.iter().map(|v| v.powi(2)).sum::<f32>().sqrt();
        let other_magnitude = v2.data.iter().map(|v| v.powi(2)).sum::<f32>().sqrt();

        if self_magnitude == 0.0 || other_magnitude == 0.0 {
            return None;
        }

        Some(dot_product/(self_magnitude*other_magnitude))
    }
}