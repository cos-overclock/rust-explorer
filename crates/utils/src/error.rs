//! エラー定義

use std::path::PathBuf;
use thiserror::Error;

/// アプリケーション固有のエラー
#[derive(Error, Debug)]
pub enum AppError {
    /// ファイルシステムエラー
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    /// 設定エラー
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// UIエラー
    #[error("UI error: {0}")]
    Ui(String),
    
    /// 無効なパスエラー
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    
    /// JSON設定エラー
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result型のエイリアス
pub type AppResult<T> = Result<T, AppError>;