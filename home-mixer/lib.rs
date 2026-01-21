//! 中文说明：本文件位于 home-mixer/lib.rs，用于说明该模块的核心实现。
mod candidate_hydrators;
mod candidate_pipeline;
pub mod clients; // Excluded from open source release for security reasons
mod filters;
pub mod params; // Excluded from open source release for security reasons
mod query_hydrators;
pub mod scorers;
mod selectors;
mod server;
mod side_effects;
mod sources;
pub mod util; // Excluded from open source release for security reasons

pub use server::HomeMixerServer;
