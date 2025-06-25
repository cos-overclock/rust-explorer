//! メインウィンドウの実装

use rust_explorer_config::Settings;
use rust_explorer_utils::AppError;

/// メインウィンドウ
pub struct MainWindow {
    title: String,
    width: u32,
    height: u32,
}

impl MainWindow {
    /// 新しいメインウィンドウを作成
    pub fn new(settings: &Settings) -> Result<Self, AppError> {
        Ok(MainWindow {
            title: "rust-explorer".to_string(),
            width: settings.window_width(),
            height: settings.window_height(),
        })
    }

    /// ウィンドウタイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// ウィンドウサイズを取得
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
