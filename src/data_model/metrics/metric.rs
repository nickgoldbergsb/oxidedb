use super::super::vector::Vector;

pub trait SimilarityMetric {
    fn compute(&self, v1: &Vector, v2: &Vector) -> Option<f32>;
}