//! 中文说明：本文件位于 candidate-pipeline/query_hydrator.rs，用于说明该模块的核心实现。
use std::any::{Any, type_name_of_val};
use tonic::async_trait;

use crate::util;

#[async_trait]
pub trait QueryHydrator<Q>: Any + Send + Sync
where
    Q: Clone + Send + Sync + 'static,
{
    /// Decide if this query hydrator should run for the given query. 中文：决定该查询补全器是否对当前查询生效。
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    /// Hydrate the query by performing async operations. 中文：通过异步操作补全查询。
    /// Returns a new query with this hydrator's fields populated. 中文：返回已填充字段的新查询。
    async fn hydrate(&self, query: &Q) -> Result<Q, String>;

    /// Update the query with the hydrated fields. 中文：用补全字段更新查询。
    /// Only the fields this hydrator is responsible for should be copied. 中文：只复制本补全器负责的字段。
    fn update(&self, query: &mut Q, hydrated: Q);

    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
