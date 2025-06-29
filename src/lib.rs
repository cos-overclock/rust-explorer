//! rust-explorer - モダンなマルチプラットフォームファイラー
//!
//! このクレートは、Rust + floemを使用して開発されたタブ型ファイラーアプリケーションです。

#![allow(clippy::result_large_err)]

// 再利用可能なクレートを再エクスポート
pub use rust_explorer_config::Settings;
pub use rust_explorer_core::{Event, EventManager, FileSystemManager};
pub use rust_explorer_ui::{App, MainWindow};
pub use rust_explorer_utils::{AppError, AppResult};

/// アプリケーションの初期化
pub fn initialize() -> Result<App, AppError> {
    App::new()
}
