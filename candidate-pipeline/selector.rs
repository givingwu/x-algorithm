//! 中文说明：本文件位于 candidate-pipeline/selector.rs，用于说明该模块的核心实现。
use crate::util;
use std::any::type_name_of_val;

pub trait Selector<Q, C>: Send + Sync
where
    Q: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Default selection: sort and truncate based on provided configs. 中文：默认选择逻辑：按配置排序并截断到指定数量。
    fn select(&self, _query: &Q, candidates: Vec<C>) -> Vec<C> {
        let mut sorted = self.sort(candidates);
        if let Some(limit) = self.size() {
            sorted.truncate(limit);
        }
        sorted
    }

    /// Decide if this selector should run for the given query. 中文：决定该选择器是否对当前查询生效。
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    /// Extract the score from a candidate to use for sorting. 中文：从候选中提取用于排序的分数。
    fn score(&self, candidate: &C) -> f64;

    /// Sort candidates by their scores in descending order. 中文：按分数从高到低排序候选。
    fn sort(&self, candidates: Vec<C>) -> Vec<C> {
        let mut sorted = candidates;
        sorted.sort_by(|a, b| {
            self.score(b)
                .partial_cmp(&self.score(a))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    /// Optionally provide a size to select. Defaults to no truncation if not overridden. 中文：可选提供选择数量；不设置时默认不截断。
    fn size(&self) -> Option<usize> {
        None
    }

    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
