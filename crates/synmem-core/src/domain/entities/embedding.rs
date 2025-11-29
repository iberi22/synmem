//! Embedding entity and related types.

use serde::{Deserialize, Serialize};

/// A vector embedding representing semantic content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    /// The embedding vector values.
    pub values: Vec<f32>,
    /// The dimensionality of the embedding.
    pub dimension: usize,
}

impl Embedding {
    /// Creates a new embedding from a vector of values.
    pub fn new(values: Vec<f32>) -> Self {
        let dimension = values.len();
        Self { values, dimension }
    }

    /// Computes the cosine similarity between this embedding and another.
    ///
    /// Returns a value between -1.0 and 1.0, where 1.0 means identical direction,
    /// 0.0 means orthogonal, and -1.0 means opposite direction.
    pub fn cosine_similarity(&self, other: &Embedding) -> f32 {
        if self.dimension != other.dimension {
            return 0.0;
        }

        let dot_product: f32 = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.values.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.values.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}

/// A search result containing a matched item and its similarity score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult<T> {
    /// The matched item.
    pub item: T,
    /// The similarity score (0.0 to 1.0 for cosine similarity).
    pub score: f32,
}

impl<T> SearchResult<T> {
    /// Creates a new search result.
    pub fn new(item: T, score: f32) -> Self {
        Self { item, score }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let values = vec![1.0, 2.0, 3.0];
        let embedding = Embedding::new(values.clone());

        assert_eq!(embedding.values, values);
        assert_eq!(embedding.dimension, 3);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![1.0, 0.0, 0.0]);

        let similarity = embedding1.cosine_similarity(&embedding2);
        assert!((similarity - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![0.0, 1.0, 0.0]);

        let similarity = embedding1.cosine_similarity(&embedding2);
        assert!(similarity.abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![-1.0, 0.0, 0.0]);

        let similarity = embedding1.cosine_similarity(&embedding2);
        assert!((similarity + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_different_dimensions() {
        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![1.0, 0.0]);

        let similarity = embedding1.cosine_similarity(&embedding2);
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let embedding1 = Embedding::new(vec![0.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![1.0, 0.0, 0.0]);

        let similarity = embedding1.cosine_similarity(&embedding2);
        assert_eq!(similarity, 0.0);
    }
}
