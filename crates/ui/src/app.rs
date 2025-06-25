//! アプリケーションのメインエントリポイント

use rust_explorer_config::Settings;
use rust_explorer_core::FileSystemManager;
use rust_explorer_utils::AppError;
use crate::window::MainWindow;

/// アプリケーションのメインクラス
pub struct App {
    settings: Settings,
    filesystem: FileSystemManager,
    main_window: Option<MainWindow>,
}

impl App {
    /// 新しいアプリケーションインスタンスを作成
    pub fn new() -> Result<Self, AppError> {
        let settings = Settings::load()?;
        let filesystem = FileSystemManager::new();
        
        Ok(App {
            settings,
            filesystem,
            main_window: None,
        })
    }
    
    /// アプリケーションを起動
    pub fn run(mut self) -> Result<(), AppError> {
        self.main_window = Some(MainWindow::new(&self.settings)?);
        // ここで実際のfloemアプリケーションを起動する
        // 現在は基本構造のみ
        Ok(())
    }
    
    /// 設定への参照を取得
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
    
    /// ファイルシステムマネージャーへの参照を取得
    pub fn filesystem(&self) -> &FileSystemManager {
        &self.filesystem
    }
}