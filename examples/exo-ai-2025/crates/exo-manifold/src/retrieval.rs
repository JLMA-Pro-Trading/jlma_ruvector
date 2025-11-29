//! Simplified retrieval module using vector similarity

use crate::network::LearnedManifold;
use exo_core::{ManifoldConfig, Pattern, Result, SearchResult};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct GradientDescentRetriever {
    _network: Arc<RwLock<LearnedManifold>>,
    _config: ManifoldConfig,
}

impl GradientDescentRetriever {
    pub fn new(
        network: Arc<RwLock<LearnedManifold>>,
        config: ManifoldConfig,
    ) -> Self {
        Self {
            _network: network,
            _config: config,
        }
    }

    pub fn retrieve(
        &self,
        query: &[f32],
        k: usize,
        patterns: &Arc<RwLock<Vec<Pattern>>>,
    ) -> Result<Vec<SearchResult>> {
        let patterns = patterns.read();
        let mut results = Vec::new();

        // Simple cosine similarity search
        for pattern in patterns.iter() {
            let similarity = cosine_similarity(query, &pattern.embedding);
            let distance = euclidean_distance(query, &pattern.embedding);
            results.push(SearchResult {
                pattern: pattern.clone(),
                score: similarity,
                distance,
            });
        }

        // Sort by score descending and take top k
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(k);

        Ok(results)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![1.0, 0.0];
        let d = vec![0.0, 1.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 1e-6);
    }
}
