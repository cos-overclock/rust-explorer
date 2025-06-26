//! rust-explorer ユーティリティクレート
//!
//! 共通で使用される機能を提供します。

pub mod error;
pub mod logging;
pub mod panic_handler;

pub use error::{AppError, AppResult, ErrorCategory, ErrorMetadata, ErrorSeverity, error_utils};
pub use logging::{
    LogConfig, LogContext, LogLevel, LogOutput, LogRotation, PerformanceTimer, init_logging,
};
pub use panic_handler::{
    PanicHandlerConfig, PostPanicAction, has_panic_occurred, init_panic_handler, reset_panic_stats,
};
