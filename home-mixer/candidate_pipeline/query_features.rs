//! 中文说明：本文件位于 home-mixer/candidate_pipeline/query_features.rs，用于说明该模块的核心实现。
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserFeatures {
    pub muted_keywords: Vec<String>,
    pub blocked_user_ids: Vec<i64>,
    pub muted_user_ids: Vec<i64>,
    pub followed_user_ids: Vec<i64>,
    pub subscribed_user_ids: Vec<i64>,
}