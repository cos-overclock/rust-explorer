//! エラー定義とハンドリング
//!
//! アプリケーション全体で使用する統一エラーハンドリングシステム

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tracing::{error, info, warn};

/// エラーの重要度レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// 致命的エラー（アプリ終了を引き起こす）
    Fatal,
    /// 重大エラー（機能停止を引き起こす）
    Critical,
    /// 一般エラー（操作失敗）
    Error,
    /// 警告（操作は続行可能）
    Warning,
    /// 情報
    Info,
}

/// エラーカテゴリ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// ファイルシステム関連
    FileSystem,
    /// 設定関連
    Configuration,
    /// UI関連
    UserInterface,
    /// ネットワーク関連
    Network,
    /// メモリ関連
    Memory,
    /// システム関連
    System,
    /// ユーザー入力関連
    UserInput,
    /// 内部ロジックエラー
    Internal,
}

/// エラーメタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetadata {
    /// エラー発生時刻
    pub timestamp: DateTime<Utc>,
    /// エラーの重要度
    pub severity: ErrorSeverity,
    /// エラーカテゴリ
    pub category: ErrorCategory,
    /// エラー発生箇所（関数名、ファイル名など）
    pub location: Option<String>,
    /// ユーザー向けメッセージ
    pub user_message: Option<String>,
    /// コンテキスト情報
    pub context: std::collections::HashMap<String, String>,
}

impl Default for ErrorMetadata {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            location: None,
            user_message: None,
            context: std::collections::HashMap::new(),
        }
    }
}

/// アプリケーション固有のエラー
#[derive(Error, Debug)]
pub enum AppError {
    /// ファイルシステムエラー
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// カスタムファイルシステムエラー
    #[error("File system error: {0}")]
    FileSystemCustom(String),

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

    /// メモリ不足エラー
    #[error("Out of memory")]
    OutOfMemory,

    /// リソースアクセスエラー
    #[error("Resource access denied: {0}")]
    AccessDenied(String),

    /// タイムアウトエラー
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// ネットワークエラー
    #[error("Network error: {0}")]
    Network(String),

    /// ユーザー入力エラー
    #[error("Invalid user input: {0}")]
    InvalidInput(String),

    /// 内部エラー
    #[error("Internal error: {0}")]
    Internal(String),

    /// 複合エラー（メタデータ付き）
    #[error("{message}")]
    WithMetadata {
        message: String,
        metadata: ErrorMetadata,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

/// Result型のエイリアス
#[allow(clippy::result_large_err)]
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// エラーの重要度を取得
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::OutOfMemory => ErrorSeverity::Fatal,
            AppError::FileSystem(_) => ErrorSeverity::Critical,
            AppError::FileSystemCustom(_) => ErrorSeverity::Critical,
            AppError::Config(_) | AppError::Json(_) => ErrorSeverity::Critical,
            AppError::AccessDenied(_) => ErrorSeverity::Error,
            AppError::Network(_) | AppError::Timeout(_) => ErrorSeverity::Error,
            AppError::Ui(_) | AppError::InvalidInput(_) => ErrorSeverity::Warning,
            AppError::InvalidPath(_) => ErrorSeverity::Warning,
            AppError::Internal(_) => ErrorSeverity::Error,
            AppError::WithMetadata { metadata, .. } => metadata.severity,
        }
    }

    /// エラーカテゴリを取得
    pub fn category(&self) -> ErrorCategory {
        match self {
            AppError::FileSystem(_) | AppError::FileSystemCustom(_) | AppError::InvalidPath(_) => {
                ErrorCategory::FileSystem
            }
            AppError::Config(_) | AppError::Json(_) => ErrorCategory::Configuration,
            AppError::Ui(_) => ErrorCategory::UserInterface,
            AppError::Network(_) => ErrorCategory::Network,
            AppError::OutOfMemory => ErrorCategory::Memory,
            AppError::AccessDenied(_) | AppError::Timeout(_) => ErrorCategory::System,
            AppError::InvalidInput(_) => ErrorCategory::UserInput,
            AppError::Internal(_) => ErrorCategory::Internal,
            AppError::WithMetadata { metadata, .. } => metadata.category.clone(),
        }
    }

    /// ユーザー向けメッセージを取得
    pub fn user_message(&self) -> String {
        match self {
            AppError::FileSystem(_) => "ファイル操作でエラーが発生しました。".to_string(),
            AppError::FileSystemCustom(msg) => msg.clone(),
            AppError::Config(_) => "設定ファイルに問題があります。".to_string(),
            AppError::Ui(_) => "画面表示でエラーが発生しました。".to_string(),
            AppError::InvalidPath(_) => "指定されたパスが無効です。".to_string(),
            AppError::Json(_) => "データの読み書きでエラーが発生しました。".to_string(),
            AppError::OutOfMemory => {
                "メモリ不足です。アプリケーションを再起動してください。".to_string()
            }
            AppError::AccessDenied(_) => {
                "アクセスが拒否されました。権限を確認してください。".to_string()
            }
            AppError::Timeout(_) => "操作がタイムアウトしました。再試行してください。".to_string(),
            AppError::Network(_) => "ネットワーク接続でエラーが発生しました。".to_string(),
            AppError::InvalidInput(_) => "入力内容に問題があります。確認してください。".to_string(),
            AppError::Internal(_) => {
                "内部エラーが発生しました。サポートに連絡してください。".to_string()
            }
            AppError::WithMetadata { metadata, .. } => metadata
                .user_message
                .clone()
                .unwrap_or_else(|| "エラーが発生しました。".to_string()),
        }
    }

    /// メタデータ付きエラーを作成
    pub fn with_metadata(
        message: impl Into<String>,
        mut metadata: ErrorMetadata,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        metadata.timestamp = Utc::now();
        AppError::WithMetadata {
            message: message.into(),
            metadata,
            source,
        }
    }

    /// コンテキスト付きエラーを作成
    pub fn with_context(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        match self {
            AppError::WithMetadata {
                message,
                mut metadata,
                source,
            } => {
                metadata.context.insert(key.into(), value.into());
                AppError::WithMetadata {
                    message,
                    metadata,
                    source,
                }
            }
            other => {
                let mut metadata = ErrorMetadata {
                    severity: other.severity(),
                    category: other.category(),
                    ..Default::default()
                };
                metadata.context.insert(key.into(), value.into());
                AppError::WithMetadata {
                    message: other.to_string(),
                    metadata,
                    source: Some(Box::new(other)),
                }
            }
        }
    }

    /// エラーをログ出力
    pub fn log(&self) {
        let severity = self.severity();
        let category = self.category();
        let user_msg = self.user_message();

        match severity {
            ErrorSeverity::Fatal => {
                error!(
                    category = ?category,
                    user_message = %user_msg,
                    error = %self,
                    "Fatal error occurred"
                );
            }
            ErrorSeverity::Critical => {
                error!(
                    category = ?category,
                    user_message = %user_msg,
                    error = %self,
                    "Critical error occurred"
                );
            }
            ErrorSeverity::Error => {
                error!(
                    category = ?category,
                    user_message = %user_msg,
                    error = %self,
                    "Error occurred"
                );
            }
            ErrorSeverity::Warning => {
                warn!(
                    category = ?category,
                    user_message = %user_msg,
                    error = %self,
                    "Warning occurred"
                );
            }
            ErrorSeverity::Info => {
                info!(
                    category = ?category,
                    user_message = %user_msg,
                    error = %self,
                    "Info message"
                );
            }
        }
    }
}

/// エラー統計情報
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorStatistics {
    /// 総エラー数
    pub total_errors: u64,
    /// 重要度別エラー数
    pub by_severity: std::collections::HashMap<ErrorSeverity, u64>,
    /// カテゴリ別エラー数
    pub by_category: std::collections::HashMap<ErrorCategory, u64>,
    /// 最新エラーのタイムスタンプ
    pub last_error_time: Option<DateTime<Utc>>,
}

impl ErrorStatistics {
    /// エラーを統計に追加
    pub fn record_error(&mut self, error: &AppError) {
        self.total_errors += 1;

        let severity = error.severity();
        *self.by_severity.entry(severity).or_insert(0) += 1;

        let category = error.category();
        *self.by_category.entry(category).or_insert(0) += 1;

        self.last_error_time = Some(Utc::now());
    }

    /// 統計をリセット
    pub fn reset(&mut self) {
        self.total_errors = 0;
        self.by_severity.clear();
        self.by_category.clear();
        self.last_error_time = None;
    }

    /// 重要度別のエラー数を取得
    pub fn get_error_count_by_severity(&self, severity: ErrorSeverity) -> u64 {
        self.by_severity.get(&severity).copied().unwrap_or(0)
    }

    /// カテゴリ別のエラー数を取得
    pub fn get_error_count_by_category(&self, category: ErrorCategory) -> u64 {
        self.by_category.get(&category).copied().unwrap_or(0)
    }
}

/// エラーハンドリングユーティリティ
pub mod error_utils {
    use super::*;
    use std::sync::LazyLock;
    use std::sync::Mutex;

    /// グローバルエラー統計
    static ERROR_STATS: LazyLock<Mutex<ErrorStatistics>> =
        LazyLock::new(|| Mutex::new(ErrorStatistics::default()));

    /// エラーを記録してログ出力
    pub fn handle_error(error: &AppError) {
        // ログ出力
        error.log();

        // 統計記録
        if let Ok(mut stats) = ERROR_STATS.lock() {
            stats.record_error(error);
        }
    }

    /// 現在のエラー統計を取得
    pub fn get_error_statistics() -> Option<ErrorStatistics> {
        ERROR_STATS.lock().ok().map(|stats| stats.clone())
    }

    /// エラー統計をリセット
    pub fn reset_error_statistics() {
        if let Ok(mut stats) = ERROR_STATS.lock() {
            stats.reset();
        }
    }

    /// エラーレベルのチェック
    pub fn is_critical_error_rate_high() -> bool {
        if let Some(stats) = get_error_statistics() {
            let critical_count = stats.get_error_count_by_severity(ErrorSeverity::Critical)
                + stats.get_error_count_by_severity(ErrorSeverity::Fatal);
            let total = stats.total_errors;

            if total > 0 {
                let critical_rate = critical_count as f64 / total as f64;
                critical_rate > 0.1 // 10%以上が重大エラー
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// エラーハンドリング用マクロ
#[macro_export]
macro_rules! app_error {
    ($severity:expr, $category:expr, $msg:expr) => {
        {
            let metadata = $crate::error::ErrorMetadata {
                severity: $severity,
                category: $category,
                location: Some(format!("{}:{}", file!(), line!())),
                ..Default::default()
            };
            $crate::error::AppError::with_metadata($msg, metadata, None)
        }
    };
    ($severity:expr, $category:expr, $msg:expr, $($context_key:expr => $context_value:expr),+) => {
        {
            let mut metadata = $crate::error::ErrorMetadata {
                severity: $severity,
                category: $category,
                location: Some(format!("{}:{}", file!(), line!())),
                ..Default::default()
            };
            $(
                metadata.context.insert($context_key.into(), $context_value.into());
            )+
            $crate::error::AppError::with_metadata($msg, metadata, None)
        }
    };
}

/// Resultユーティリティ
#[allow(clippy::result_large_err)]
pub trait AppResultExt<T> {
    /// エラーにコンテキストを追加
    fn with_context(self, key: impl Into<String>, value: impl Into<String>) -> AppResult<T>;

    /// エラーをログ出力して結果を返す
    fn log_error(self) -> AppResult<T>;

    /// エラーをハンドリングしてデフォルト値を返す
    fn handle_error_with_default(self, default: T) -> T;
}

impl<T> AppResultExt<T> for AppResult<T> {
    fn with_context(self, key: impl Into<String>, value: impl Into<String>) -> AppResult<T> {
        self.map_err(|e| e.with_context(key, value))
    }

    fn log_error(self) -> AppResult<T> {
        if let Err(ref e) = self {
            error_utils::handle_error(e);
        }
        self
    }

    fn handle_error_with_default(self, default: T) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                error_utils::handle_error(&e);
                default
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity() {
        let error = AppError::OutOfMemory;
        assert_eq!(error.severity(), ErrorSeverity::Fatal);

        let error = AppError::Config("test".to_string());
        assert_eq!(error.severity(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_category() {
        let error = AppError::FileSystem(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert_eq!(error.category(), ErrorCategory::FileSystem);

        let error = AppError::Network("test".to_string());
        assert_eq!(error.category(), ErrorCategory::Network);
    }

    #[test]
    fn test_user_message() {
        let error = AppError::InvalidInput("test".to_string());
        let msg = error.user_message();
        assert!(msg.contains("入力内容に問題"));
    }

    #[test]
    fn test_error_with_metadata() {
        let mut metadata = ErrorMetadata::default();
        metadata.severity = ErrorSeverity::Critical;
        metadata.category = ErrorCategory::Configuration;
        metadata.user_message = Some("テストメッセージ".to_string());

        let error = AppError::with_metadata("Test error", metadata, None);
        assert_eq!(error.severity(), ErrorSeverity::Critical);
        assert_eq!(error.category(), ErrorCategory::Configuration);
    }

    #[test]
    fn test_error_with_context() {
        let error = AppError::Internal("test".to_string())
            .with_context("function", "test_function")
            .with_context("module", "test_module");

        if let AppError::WithMetadata { metadata, .. } = error {
            assert_eq!(
                metadata.context.get("function"),
                Some(&"test_function".to_string())
            );
            assert_eq!(
                metadata.context.get("module"),
                Some(&"test_module".to_string())
            );
        } else {
            panic!("Expected WithMetadata error");
        }
    }

    #[test]
    fn test_error_statistics() {
        let mut stats = ErrorStatistics::default();

        let error1 = AppError::Config("test".to_string());
        let error2 = AppError::InvalidInput("test".to_string());

        stats.record_error(&error1);
        stats.record_error(&error2);

        assert_eq!(stats.total_errors, 2);
        assert_eq!(
            stats.get_error_count_by_severity(ErrorSeverity::Critical),
            1
        );
        assert_eq!(stats.get_error_count_by_severity(ErrorSeverity::Warning), 1);
    }

    #[test]
    fn test_app_error_macro() {
        let error = app_error!(
            ErrorSeverity::Error,
            ErrorCategory::UserInput,
            "Test error message",
            "user_id" => "123",
            "action" => "file_open"
        );

        if let AppError::WithMetadata { metadata, .. } = error {
            assert_eq!(metadata.severity, ErrorSeverity::Error);
            assert_eq!(metadata.category, ErrorCategory::UserInput);
            assert_eq!(metadata.context.get("user_id"), Some(&"123".to_string()));
            assert_eq!(
                metadata.context.get("action"),
                Some(&"file_open".to_string())
            );
        } else {
            panic!("Expected WithMetadata error");
        }
    }

    #[test]
    fn test_result_extensions() {
        let result: AppResult<i32> = Err(AppError::Internal("test".to_string()));
        let result_with_context = result.with_context("test_key", "test_value");

        assert!(result_with_context.is_err());

        let default_value =
            Err(AppError::Internal("test".to_string())).handle_error_with_default(42);
        assert_eq!(default_value, 42);
    }
}
