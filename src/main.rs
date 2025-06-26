#![allow(clippy::result_large_err)]

use rust_explorer_ui::App;
use rust_explorer_utils::{
    AppResult, LogConfig, LogLevel, LogOutput, LogRotation, PanicHandlerConfig, PerformanceTimer,
    PostPanicAction, init_logging, init_panic_handler,
};
use std::path::PathBuf;
use tracing::{error, info};

fn main() -> AppResult<()> {
    // ログシステムの初期化
    let log_config = LogConfig {
        level: LogLevel::Info,
        output: LogOutput::Rolling {
            directory: PathBuf::from("./logs"),
            file_prefix: "rust-explorer".to_string(),
            rotation: LogRotation::Daily,
        },
        include_location: true,
        include_thread_name: false,
    };

    if let Err(e) = init_logging(log_config) {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    // パニックハンドラーの初期化
    let panic_config = PanicHandlerConfig {
        save_crash_reports: true,
        crash_reports_dir: PathBuf::from("./crash_reports"),
        include_backtrace: true,
        include_system_info: true,
        post_panic_action: PostPanicAction::Exit,
        max_crash_reports: 10,
    };

    if let Err(e) = init_panic_handler(panic_config) {
        error!("Failed to initialize panic handler: {}", e);
        std::process::exit(1);
    }

    // アプリケーション開始ログ
    info!(
        version = env!("CARGO_PKG_VERSION"),
        config = "log_level=Info, crash_reports=enabled",
        "Application starting"
    );

    // アプリケーション実行
    let start_time = std::time::Instant::now();
    let result = run_application();
    let uptime = start_time.elapsed();

    // アプリケーション終了ログ
    info!(
        exit_status = match &result {
            Ok(_) => "normal_exit",
            Err(_) => "error_exit",
        },
        uptime_seconds = uptime.as_secs(),
        "Application shutdown"
    );

    result
}

fn run_application() -> AppResult<()> {
    let timer = PerformanceTimer::start("application_startup");

    let app = App::new()?;
    timer.stop();

    let timer = PerformanceTimer::start("application_runtime");
    let result = app.run();
    timer.stop();

    result
}
