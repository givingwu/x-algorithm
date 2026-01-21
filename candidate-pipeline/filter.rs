//! 中文说明：本文件位于 candidate-pipeline/filter.rs，用于说明该模块的核心实现。
use std::any::{Any, type_name_of_val};
use tonic::async_trait;

use crate::util;

pub struct FilterResult<C> {
    pub kept: Vec<C>,
    pub removed: Vec<C>,
}

/// Filters run sequentially and partition candidates into kept and removed sets. 中文：过滤器按顺序执行，把候选拆分为保留与移除两组。
#[async_trait]
pub trait Filter<Q, C>: Any + Send + Sync
where
    Q: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Decide if this filter should run for the given query. 中文：决定该过滤器是否对当前查询生效。
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    /// Filter candidates by evaluating each against some criteria, returning kept candidates (which continue to the next stage) and removed candidates (which are excluded from further processing). 中文：依据规则筛选候选，返回保留与移除的结果；保留的继续进入下一阶段。
    async fn filter(&self, query: &Q, candidates: Vec<C>) -> Result<FilterResult<C>, String>;

    /// Returns a stable name for logging/metrics. 中文：返回用于日志与监控的稳定名称。
    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
