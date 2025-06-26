//! ログシステム
//!
//! 構造化ログとファイルローテーション機能を提供します。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{Level, info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;

/// ログレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    /// エラーレベル
    Error,
    /// 警告レベル
    Warn,
    /// 情報レベル
    Info,
    /// デバッグレベル
    Debug,
    /// トレースレベル
    Trace,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

impl LogLevel {
    /// トレースフィルタ文字列に変換
    pub fn to_env_filter_string(&self) -> String {
        match self {
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
        }
    }
}

/// ログ出力先
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogOutput {
    /// 標準出力
    #[default]
    Stdout,
    /// ファイル出力
    File {
        /// ファイルパス
        path: PathBuf,
    },
    /// ローテーションファイル出力
    Rolling {
        /// 出力ディレクトリ
        directory: PathBuf,
        /// ファイル名プレフィックス
        file_prefix: String,
        /// ローテーション設定
        rotation: LogRotation,
    },
}

/// ログローテーション設定
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogRotation {
    /// 毎時
    Hourly,
    /// 毎日
    Daily,
    /// 毎月
    Monthly,
}

impl From<LogRotation> for Rotation {
    fn from(rotation: LogRotation) -> Self {
        match rotation {
            LogRotation::Hourly => Rotation::HOURLY,
            LogRotation::Daily => Rotation::DAILY,
            LogRotation::Monthly => Rotation::NEVER, // 近似
        }
    }
}

/// ログ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// ログレベル
    pub level: LogLevel,
    /// 出力先
    pub output: LogOutput,
    /// ファイル位置を含めるか
    pub include_location: bool,
    /// スレッド名を含めるか
    pub include_thread_name: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            output: LogOutput::default(),
            include_location: true,
            include_thread_name: false,
        }
    }
}

/// ログシステムを初期化
pub fn init_logging(config: LogConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.to_env_filter_string()));

    match config.output {
        LogOutput::Stdout => {
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_names(config.include_thread_name)
                .init();
        }
        LogOutput::File { path } => {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;

            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_writer(file)
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_names(config.include_thread_name)
                .init();
        }
        LogOutput::Rolling {
            directory,
            file_prefix,
            rotation,
        } => {
            let file_appender = RollingFileAppender::builder()
                .rotation(rotation.into())
                .filename_prefix(file_prefix)
                .filename_suffix("log")
                .build(directory)?;

            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_writer(file_appender)
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_names(config.include_thread_name)
                .init();
        }
    }

    info!("Logging system initialized");
    Ok(())
}

/// ログコンテキスト
pub struct LogContext {
    /// 作成時刻
    pub created_at: DateTime<Utc>,
    /// セッションID
    pub session_id: String,
}

impl LogContext {
    /// 新しいログコンテキストを作成
    pub fn new() -> Self {
        Self {
            created_at: Utc::now(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

impl Default for LogContext {
    fn default() -> Self {
        Self::new()
    }
}

/// パフォーマンス計測用のタイマー
pub struct PerformanceTimer {
    /// 開始時刻
    start_time: std::time::Instant,
    /// 操作名
    operation_name: String,
}

impl PerformanceTimer {
    /// 新しいタイマーを開始
    pub fn start(operation_name: impl Into<String>) -> Self {
        let operation_name = operation_name.into();
        info!(operation = %operation_name, "Starting operation");

        Self {
            start_time: std::time::Instant::now(),
            operation_name,
        }
    }

    /// タイマーを停止して経過時間をログ出力
    pub fn stop(self) {
        let elapsed = self.start_time.elapsed();
        info!(
            operation = %self.operation_name,
            duration_ms = elapsed.as_millis(),
            "Operation completed"
        );
    }
}

/// ログファイルのクリーンアップ
pub fn cleanup_old_logs(log_dir: &Path, max_files: usize) -> Result<(), std::io::Error> {
    let mut log_files: Vec<_> = std::fs::read_dir(log_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "log" {
                Some((path, entry.metadata().ok()?.modified().ok()?))
            } else {
                None
            }
        })
        .collect();

    if log_files.len() <= max_files {
        return Ok(());
    }

    // ファイルを最終更新日時でソート（古い順）
    log_files.sort_by_key(|(_, modified)| *modified);

    // 古いファイルを削除
    let files_to_remove = log_files.len() - max_files;
    for (path, _) in log_files.into_iter().take(files_to_remove) {
        if let Err(e) = std::fs::remove_file(&path) {
            warn!(?path, error = %e, "Failed to remove old log file");
        } else {
            info!(?path, "Removed old log file");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
        assert_eq!(Level::from(LogLevel::Info), Level::INFO);
        assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(matches!(config.output, LogOutput::Stdout));
        assert!(config.include_location);
        assert!(!config.include_thread_name);
    }

    #[test]
    fn test_log_context() {
        let context = LogContext::new();
        assert!(!context.session_id.is_empty());
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        timer.stop();
    }

    #[test]
    fn test_cleanup_old_logs() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let log_dir = temp_dir.path();

        // テスト用ログファイルを作成
        for i in 0..5 {
            let log_file = log_dir.join(format!("test_{}.log", i));
            std::fs::write(log_file, "test log content")?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // 2ファイルまで保持するようクリーンアップ
        cleanup_old_logs(log_dir, 2)?;

        // 2ファイルだけ残っているか確認
        let remaining_files: Vec<_> = std::fs::read_dir(log_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.path().extension()? == "log" {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(remaining_files.len(), 2);
        Ok(())
    }
}
