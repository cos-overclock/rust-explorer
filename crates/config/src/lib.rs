//! rust-explorer 設定管理クレート
//!
//! アプリケーションの設定に関する機能を提供します。

#![allow(clippy::result_large_err)]

pub mod settings;
pub mod state_persistence;

#[cfg(test)]
mod tests;

pub use settings::Settings;
pub use state_persistence::{StatePersistenceConfig, StatePersistenceManager, state_helpers};
