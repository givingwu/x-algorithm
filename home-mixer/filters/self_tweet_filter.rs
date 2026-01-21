//! 中文说明：本文件位于 home-mixer/filters/self_tweet_filter.rs，用于说明该模块的核心实现。
use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

/// Filter that removes tweets where the author is the viewer. 中文：过滤掉作者就是当前用户的帖子。
pub struct SelfTweetFilter;

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for SelfTweetFilter {
    async fn filter(
        &self,
        query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let viewer_id = query.user_id as u64;
        let (kept, removed): (Vec<_>, Vec<_>) = candidates
            .into_iter()
            .partition(|c| c.author_id != viewer_id);

        Ok(FilterResult { kept, removed })
    }
}
