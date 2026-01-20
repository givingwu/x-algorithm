//! 中文说明：本文件位于 candidate-pipeline/source.rs，用于说明该模块的核心实现。
use std::any::{Any, type_name_of_val};
use tonic::async_trait;

use crate::util;

#[async_trait]
pub trait Source<Q, C>: Any + Send + Sync
where
    Q: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Decide if this source should run for the given query
    fn enable(&self, _query: &Q) -> bool {
        true
    }

    async fn get_candidates(&self, query: &Q) -> Result<Vec<C>, String>;

    fn name(&self) -> &'static str {
        util::short_type_name(type_name_of_val(self))
    }
}
