//! 中文说明：本文件位于 home-mixer/filters/retweet_deduplication_filter.rs，用于说明该模块的核心实现。
use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

/// Deduplicates retweets, keeping only the first occurrence of a tweet (whether as an original or as a retweet). 中文：对转推去重，同一内容只保留首次出现（原帖或转推均算）。
pub struct RetweetDeduplicationFilter;

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for RetweetDeduplicationFilter {
    async fn filter(
        &self,
        _query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let mut seen_tweet_ids: HashSet<u64> = HashSet::new();
        let mut kept = Vec::new();
        let mut removed = Vec::new();

        for candidate in candidates {
            match candidate.retweeted_tweet_id {
                Some(retweeted_id) => {
                    // Remove if we've already seen this tweet (as original or retweet). 中文：如果已经见过该内容（原帖或转推），则移除。
                    if seen_tweet_ids.insert(retweeted_id) {
                        kept.push(candidate);
                    } else {
                        removed.push(candidate);
                    }
                }
                None => {
                    // Mark this original tweet ID as seen so retweets of it get filtered. 中文：将原帖 ID 标记为已见，后续转推会被过滤。
                    seen_tweet_ids.insert(candidate.tweet_id as u64);
                    kept.push(candidate);
                }
            }
        }

        Ok(FilterResult { kept, removed })
    }
}
