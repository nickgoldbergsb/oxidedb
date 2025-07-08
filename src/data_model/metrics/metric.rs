use super::vector::Vector;

pub trait SimilarityMetric {
    fn compute(v1: &Vector, v2: &Vector) -> Option<f32>;
}