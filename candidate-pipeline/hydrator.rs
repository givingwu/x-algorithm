//! 中文说明：本文件位于 candidate-pipeline/hydrator.rs，用于说明该模块的核心实现。
use crate::util;
use std::any::{Any, type_name_of_val};
use tonic::async_trait;

// Hydrators run in parallel and update candidate fields. 中文：补全器并行运行，用来更新候选内容的字段。
#[async_trait]
pub trait Hydrator<Q, C>: Any + Send + Sync
where
    Q: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Decide if this hydrator should run for the given query. 中文：决定该补全器是否对当前查询生效。
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    /// Hydrate candidates by performing async operations. 中文：通过异步操作补全候选字段。
    /// Returns candidates with this hydrator's fields populated. 中文：返回已填充相关字段的候选。
    /// IMPORTANT: The returned vector must have the same candidates in the same order as the input. 中文：注意：返回的候选数量与顺序必须与输入一致。
    /// Dropping candidates in a hydrator is not allowed - use a filter stage instead. 中文：补全阶段不允许丢弃候选，如需移除请使用过滤阶段。
    async fn hydrate(&self, query: &Q, candidates: &[C]) -> Result<Vec<C>, String>;

    /// Update a single candidate with the hydrated fields. 中文：用补全字段更新单个候选。
    /// Only the fields this hydrator is responsible for should be copied. 中文：只更新本补全器负责的字段。
    fn update(&self, candidate: &mut C, hydrated: C);

    /// Update all candidates with the hydrated fields from `hydrated`. 中文：用 `hydrated` 中的字段更新所有候选。
    /// Default implementation iterates and calls `update` for each pair. 中文：默认实现会逐对调用 `update`。
    fn update_all(&self, candidates: &mut [C], hydrated: Vec<C>) {
        for (c, h) in candidates.iter_mut().zip(hydrated) {
            self.update(c, h);
        }
    }

    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
