//! アプリケーションのメインエントリポイント

use crate::window::MainWindow;
use rust_explorer_config::Settings;
use rust_explorer_core::{Event, EventManager, FileSystemManager};
use rust_explorer_utils::AppError;

/// アプリケーションのメインクラス
pub struct App {
    settings: Settings,
    filesystem: FileSystemManager,
    event_manager: EventManager,
}

impl App {
    /// 新しいアプリケーションインスタンスを作成
    pub fn new() -> Result<Self, AppError> {
        let settings = Settings::load()?;
        let filesystem = FileSystemManager::new();
        let event_manager = EventManager::new();

        Ok(App {
            settings,
            filesystem,
            event_manager,
        })
    }

    /// アプリケーションを初期化
    pub fn initialize(&mut self) -> Result<(), AppError> {
        // アプリケーション初期化処理
        self.event_manager.handle_event(Event::DirectoryChanged(
            self.filesystem.current_path().to_path_buf(),
        ))?;

        Ok(())
    }

    /// アプリケーションを起動
    pub fn run(mut self) -> Result<(), AppError> {
        // 初期化処理
        self.initialize()?;

        // メインウィンドウを作成して起動
        let main_window = MainWindow::new(&self.settings)?;
        main_window.launch()?;

        // アプリケーション終了時の処理
        self.shutdown()?;

        Ok(())
    }

    /// アプリケーション終了処理
    pub fn shutdown(&mut self) -> Result<(), AppError> {
        // 設定を保存
        self.settings.save()?;

        // その他のクリーンアップ処理
        println!("アプリケーションを終了します");

        Ok(())
    }

    /// 設定への参照を取得
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// 設定への可変参照を取得
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// ファイルシステムマネージャーへの参照を取得
    pub fn filesystem(&self) -> &FileSystemManager {
        &self.filesystem
    }

    /// ファイルシステムマネージャーへの可変参照を取得
    pub fn filesystem_mut(&mut self) -> &mut FileSystemManager {
        &mut self.filesystem
    }

    /// イベントマネージャーへの参照を取得
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let result = App::new();
        assert!(result.is_ok());

        let app = result.unwrap();
        assert_eq!(app.settings().window_width(), 1200);
        assert_eq!(app.settings().window_height(), 800);
    }

    #[test]
    fn test_app_initialization() {
        let mut app = App::new().unwrap();
        let result = app.initialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_shutdown() {
        let mut app = App::new().unwrap();
        let result = app.shutdown();
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_accessors() {
        let mut app = App::new().unwrap();

        // 設定への参照テスト
        let settings = app.settings();
        assert_eq!(settings.window_width(), 1200);

        // 設定への可変参照テスト
        let settings_mut = app.settings_mut();
        settings_mut.update_window_state(1024, 768, None, None, false);
        assert_eq!(app.settings().window_width(), 1024);

        // ファイルシステムマネージャーテスト
        let filesystem = app.filesystem();
        assert!(filesystem.current_path().exists());

        // イベントマネージャーテスト
        let _event_manager = app.event_manager();
    }
}
