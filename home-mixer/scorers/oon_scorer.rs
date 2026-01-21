//! 中文说明：本文件位于 home-mixer/scorers/oon_scorer.rs，用于说明该模块的核心实现。
use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use crate::params as p;
use tonic::async_trait;
use xai_candidate_pipeline::scorer::Scorer;

// Prioritize in-network candidates over out-of-network candidates. 中文：优先展示关注关系内的候选，高于关注关系外的候选。
pub struct OONScorer;

#[async_trait]
impl Scorer<ScoredPostsQuery, PostCandidate> for OONScorer {
    async fn score(
        &self,
        _query: &ScoredPostsQuery,
        candidates: &[PostCandidate],
    ) -> Result<Vec<PostCandidate>, String> {
        let scored = candidates
            .iter()
            .map(|c| {
                let updated_score = c.score.map(|base_score| match c.in_network {
                    Some(false) => base_score * p::OON_WEIGHT_FACTOR,
                    _ => base_score,
                });

                PostCandidate {
                    score: updated_score,
                    ..Default::default()
                }
            })
            .collect();

        Ok(scored)
    }

    fn update(&self, candidate: &mut PostCandidate, scored: PostCandidate) {
        candidate.score = scored.score;
    }
}
