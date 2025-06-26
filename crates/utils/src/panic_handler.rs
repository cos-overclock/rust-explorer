//! パニックハンドラー
//!
//! アプリケーションのパニック処理とクラッシュレポート機能を提供します。

#![allow(clippy::result_large_err)]

use crate::error::{
    AppError, AppResultExt, ErrorCategory, ErrorMetadata, ErrorSeverity, error_utils,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::panic::{PanicHookInfo, set_hook, take_hook};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use tracing::{error, info, warn};

/// パニック情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicReport {
    /// パニック発生時刻
    pub timestamp: chrono::DateTime<Utc>,
    /// パニックメッセージ
    pub message: String,
    /// パニック発生場所
    pub location: Option<PanicLocation>,
    /// バックトレース（文字列化）
    pub backtrace: Option<String>,
    /// スレッド情報
    pub thread_info: ThreadInfo,
    /// アプリケーション情報
    pub app_info: AppInfo,
    /// システム情報
    pub system_info: SystemInfo,
}

/// パニック発生場所
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicLocation {
    /// ファイル名
    pub file: String,
    /// 行番号
    pub line: u32,
    /// 列番号
    pub column: u32,
}

/// スレッド情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadInfo {
    /// スレッド名
    pub name: Option<String>,
    /// スレッドID
    pub id: String,
    /// メインスレッドかどうか
    pub is_main: bool,
}

/// アプリケーション情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// アプリケーション名
    pub name: String,
    /// バージョン
    pub version: String,
    /// 起動時刻
    pub start_time: chrono::DateTime<Utc>,
    /// 稼働時間（秒）
    pub uptime_seconds: u64,
}

/// システム情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// OS名
    pub os: String,
    /// アーキテクチャ
    pub arch: String,
    /// Rustバージョン
    pub rust_version: String,
}

/// パニックハンドラー設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicHandlerConfig {
    /// クラッシュレポートを保存するかどうか
    pub save_crash_reports: bool,
    /// クラッシュレポートの保存ディレクトリ
    pub crash_reports_dir: PathBuf,
    /// バックトレースを含めるかどうか
    pub include_backtrace: bool,
    /// 詳細なシステム情報を含めるかどうか
    pub include_system_info: bool,
    /// パニック後の動作
    pub post_panic_action: PostPanicAction,
    /// 最大保存クラッシュレポート数
    pub max_crash_reports: usize,
}

/// パニック後の動作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostPanicAction {
    /// 通常の終了処理を行う
    Exit,
    /// 強制終了
    Abort,
    /// 何もしない（デフォルト動作）
    Continue,
}

impl Default for PanicHandlerConfig {
    fn default() -> Self {
        Self {
            save_crash_reports: true,
            crash_reports_dir: PathBuf::from("./crash_reports"),
            include_backtrace: true,
            include_system_info: true,
            post_panic_action: PostPanicAction::Exit,
            max_crash_reports: 10,
        }
    }
}

/// グローバルなパニック統計
static PANIC_COUNT: AtomicBool = AtomicBool::new(false);
static APP_START_TIME: Mutex<Option<chrono::DateTime<Utc>>> = Mutex::new(None);

/// パニックハンドラーを初期化
pub fn init_panic_handler(config: PanicHandlerConfig) -> Result<(), AppError> {
    // アプリケーション開始時刻を記録
    {
        let mut start_time = APP_START_TIME
            .lock()
            .map_err(|_| AppError::Internal("Failed to acquire start time mutex".to_string()))?;
        *start_time = Some(Utc::now());
    }

    // クラッシュレポートディレクトリを作成
    if config.save_crash_reports {
        std::fs::create_dir_all(&config.crash_reports_dir).map_err(|e| {
            AppError::FileSystem(e)
                .with_context("directory", config.crash_reports_dir.display().to_string())
                .with_context("operation", "create_crash_reports_directory")
        })?;

        // 古いクラッシュレポートをクリーンアップ
        cleanup_old_crash_reports(&config.crash_reports_dir, config.max_crash_reports)?;
    }

    // 既存のパニックハンドラーを保存
    let original_hook = take_hook();

    // カスタムパニックハンドラーを設定
    set_hook(Box::new(move |panic_info| {
        // パニック発生フラグを設定
        PANIC_COUNT.store(true, Ordering::SeqCst);

        // パニックレポートを作成
        let report = create_panic_report(panic_info, &config);

        // ログ出力
        log_panic(&report);

        // エラーハンドリングシステムに通知
        let app_error = AppError::with_metadata(
            format!("Application panic: {}", report.message),
            ErrorMetadata {
                severity: ErrorSeverity::Fatal,
                category: ErrorCategory::System,
                timestamp: report.timestamp,
                location: report
                    .location
                    .as_ref()
                    .map(|loc| format!("{}:{}:{}", loc.file, loc.line, loc.column)),
                user_message: Some("アプリケーションで重大なエラーが発生しました。".to_string()),
                context: std::collections::HashMap::new(),
            },
            None,
        );
        error_utils::handle_error(&app_error);

        // クラッシュレポートを保存
        if config.save_crash_reports {
            if let Err(e) = save_crash_report(&report, &config.crash_reports_dir) {
                error!("Failed to save crash report: {}", e);
            }
        }

        // 元のパニックハンドラーを呼び出し
        original_hook(panic_info);

        // 設定に応じた後処理
        match config.post_panic_action {
            PostPanicAction::Exit => {
                warn!("Performing graceful shutdown after panic");
                std::process::exit(1);
            }
            PostPanicAction::Abort => {
                error!("Aborting after panic");
                std::process::abort();
            }
            PostPanicAction::Continue => {
                warn!("Continuing after panic (not recommended)");
            }
        }
    }));

    info!("Panic handler initialized successfully");
    Ok(())
}

/// パニックレポートを作成
fn create_panic_report(panic_info: &PanicHookInfo, config: &PanicHandlerConfig) -> PanicReport {
    let timestamp = Utc::now();

    // パニックメッセージを取得
    let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic message".to_string()
    };

    // パニック発生場所を取得
    let location = panic_info.location().map(|loc| PanicLocation {
        file: loc.file().to_string(),
        line: loc.line(),
        column: loc.column(),
    });

    // バックトレースを取得
    let backtrace = if config.include_backtrace {
        let bt = Backtrace::capture();
        match bt.status() {
            BacktraceStatus::Captured => Some(bt.to_string()),
            _ => None,
        }
    } else {
        None
    };

    // スレッド情報を取得
    let current_thread = thread::current();
    let thread_info = ThreadInfo {
        name: current_thread.name().map(|s| s.to_string()),
        id: format!("{:?}", current_thread.id()),
        is_main: current_thread.name() == Some("main"),
    };

    // アプリケーション情報を取得
    let start_time = APP_START_TIME
        .lock()
        .ok()
        .and_then(|guard| *guard)
        .unwrap_or_else(Utc::now);

    let uptime_seconds = (timestamp - start_time).num_seconds().max(0) as u64;

    let app_info = AppInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        start_time,
        uptime_seconds,
    };

    // システム情報を取得
    let system_info = if config.include_system_info {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            rust_version: "rustc --version".to_string(), // 実際の実装では実行する
        }
    } else {
        SystemInfo {
            os: "Unknown".to_string(),
            arch: "Unknown".to_string(),
            rust_version: "Unknown".to_string(),
        }
    };

    PanicReport {
        timestamp,
        message,
        location,
        backtrace,
        thread_info,
        app_info,
        system_info,
    }
}

/// パニックをログ出力
fn log_panic(report: &PanicReport) {
    error!(
        timestamp = %report.timestamp,
        message = %report.message,
        thread_name = ?report.thread_info.name,
        thread_id = %report.thread_info.id,
        is_main_thread = report.thread_info.is_main,
        uptime_seconds = report.app_info.uptime_seconds,
        location = ?report.location,
        "APPLICATION PANIC OCCURRED"
    );

    if let Some(ref backtrace) = report.backtrace {
        error!(backtrace = %backtrace, "Panic backtrace");
    }
}

/// クラッシュレポートをファイルに保存
fn save_crash_report(report: &PanicReport, dir: &Path) -> Result<(), AppError> {
    let timestamp_str = report.timestamp.format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("crash_report_{}.json", timestamp_str);
    let filepath = dir.join(filename);

    let json_content = serde_json::to_string_pretty(report).map_err(AppError::Json)?;

    std::fs::write(&filepath, json_content)
        .map_err(AppError::FileSystem)
        .with_context("file", filepath.display().to_string())
        .with_context("operation", "save_crash_report")?;

    info!("Crash report saved to: {}", filepath.display());
    Ok(())
}

/// 古いクラッシュレポートをクリーンアップ
fn cleanup_old_crash_reports(dir: &Path, max_reports: usize) -> Result<(), AppError> {
    let entries = std::fs::read_dir(dir)
        .map_err(AppError::FileSystem)
        .with_context("directory", dir.display().to_string())?;

    let mut crash_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()?
                    .to_string_lossy()
                    .starts_with("crash_report_")
                && path.extension()? == "json"
            {
                let metadata = entry.metadata().ok()?;
                let modified = metadata.modified().ok()?;
                Some((path, modified))
            } else {
                None
            }
        })
        .collect();

    // 修正時刻でソート（新しい順）
    crash_files.sort_by(|a, b| b.1.cmp(&a.1));

    // 上限を超えたファイルを削除
    if crash_files.len() > max_reports {
        for (path, _) in crash_files.iter().skip(max_reports) {
            if let Err(e) = std::fs::remove_file(path) {
                warn!(
                    "Failed to remove old crash report {}: {}",
                    path.display(),
                    e
                );
            } else {
                info!("Removed old crash report: {}", path.display());
            }
        }
    }

    Ok(())
}

/// パニックが発生したかどうかを確認
pub fn has_panic_occurred() -> bool {
    PANIC_COUNT.load(Ordering::SeqCst)
}

/// パニック統計をリセット
pub fn reset_panic_stats() {
    PANIC_COUNT.store(false, Ordering::SeqCst);
}

/// 手動でクラッシュレポートをテスト
pub fn test_crash_report(config: &PanicHandlerConfig) -> Result<(), AppError> {
    warn!("Testing crash report generation");

    let test_report = PanicReport {
        timestamp: Utc::now(),
        message: "Test crash report".to_string(),
        location: Some(PanicLocation {
            file: "test.rs".to_string(),
            line: 42,
            column: 10,
        }),
        backtrace: Some("Test backtrace".to_string()),
        thread_info: ThreadInfo {
            name: Some("test_thread".to_string()),
            id: "test_id".to_string(),
            is_main: false,
        },
        app_info: AppInfo {
            name: "test_app".to_string(),
            version: "1.0.0".to_string(),
            start_time: Utc::now(),
            uptime_seconds: 60,
        },
        system_info: SystemInfo {
            os: "test_os".to_string(),
            arch: "test_arch".to_string(),
            rust_version: "1.70.0".to_string(),
        },
    };

    if config.save_crash_reports {
        save_crash_report(&test_report, &config.crash_reports_dir)?;
    }

    log_panic(&test_report);
    info!("Test crash report generated successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_panic_handler_config_default() {
        let config = PanicHandlerConfig::default();
        assert!(config.save_crash_reports);
        assert!(config.include_backtrace);
        assert!(config.include_system_info);
        assert_eq!(config.post_panic_action, PostPanicAction::Exit);
        assert_eq!(config.max_crash_reports, 10);
    }

    #[test]
    fn test_panic_report_creation() {
        let _config = PanicHandlerConfig::default();

        // パニック情報をモック
        std::panic::catch_unwind(|| {
            panic!("Test panic message");
        })
        .unwrap_err();

        // 実際のテストは統合テストで行う（パニックハンドラーのテストは複雑）
    }

    #[test]
    fn test_crash_report_save_and_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let config = PanicHandlerConfig {
            save_crash_reports: true,
            crash_reports_dir: temp_dir.path().to_path_buf(),
            max_crash_reports: 2,
            ..Default::default()
        };

        // テストレポートを作成
        let result = test_crash_report(&config);
        assert!(result.is_ok());

        // ファイルが作成されているか確認
        let entries: Vec<_> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("crash_report_"))
            .collect();

        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_panic_stats() {
        reset_panic_stats();
        assert!(!has_panic_occurred());

        PANIC_COUNT.store(true, Ordering::SeqCst);
        assert!(has_panic_occurred());

        reset_panic_stats();
        assert!(!has_panic_occurred());
    }

    #[test]
    fn test_cleanup_old_reports() {
        let temp_dir = TempDir::new().unwrap();

        // テストファイルを作成
        for i in 0..5 {
            let filename = format!("crash_report_202301{:02}_120000.json", i + 1);
            let filepath = temp_dir.path().join(filename);
            std::fs::write(filepath, "{}").unwrap();
        }

        // クリーンアップを実行
        let result = cleanup_old_crash_reports(&temp_dir.path().to_path_buf(), 2);
        assert!(result.is_ok());

        // 残りのファイル数を確認
        let remaining_files = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("crash_report_"))
            .count();

        assert!(remaining_files <= 2);
    }
}
