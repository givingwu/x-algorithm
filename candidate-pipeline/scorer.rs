//! 中文说明：本文件位于 candidate-pipeline/scorer.rs，用于说明该模块的核心实现。
use crate::util;
use std::any::type_name_of_val;
use tonic::async_trait;

/// Scorers update candidate fields (like a score field) and run sequentially. 中文：评分器按顺序执行，用来更新候选字段（例如分数）。
#[async_trait]
pub trait Scorer<Q, C>: Send + Sync
where
    Q: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Decide if this scorer should run for the given query. 中文：决定该评分器是否对当前查询生效。
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    /// Score candidates by performing async operations. 中文：通过异步操作为候选打分。
    /// Returns candidates with this scorer's fields populated. 中文：返回已填充评分字段的候选。
    /// IMPORTANT: The returned vector must have the same candidates in the same order as the input. 中文：注意：返回的候选数量与顺序必须与输入一致。
    /// Dropping candidates in a scorer is not allowed - use a filter stage instead. 中文：评分阶段不允许丢弃候选；如需移除请使用过滤阶段。
    async fn score(&self, query: &Q, candidates: &[C]) -> Result<Vec<C>, String>;

    /// Update a single candidate with the scored fields. 中文：用评分字段更新单个候选。
    /// Only the fields this scorer is responsible for should be copied. 中文：只更新本评分器负责的字段。
    fn update(&self, candidate: &mut C, scored: C);

    /// Update all candidates with the scored fields from `scored`. 中文：用 `scored` 中的字段更新所有候选。
    /// Default implementation iterates and calls `update` for each pair. 中文：默认实现会逐对调用 `update`。
    fn update_all(&self, candidates: &mut [C], scored: Vec<C>) {
        for (c, s) in candidates.iter_mut().zip(scored) {
            self.update(c, s);
        }
    }

    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
