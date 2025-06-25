//! アプリケーション設定

use rust_explorer_utils::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// ウィンドウ幅
    pub window_width: u32,
    /// ウィンドウ高さ
    pub window_height: u32,
    /// ウィンドウのX座標
    pub window_x: Option<i32>,
    /// ウィンドウのY座標
    pub window_y: Option<i32>,
    /// 最小ウィンドウ幅
    pub min_window_width: u32,
    /// 最小ウィンドウ高さ
    pub min_window_height: u32,
    /// ウィンドウが最大化されているか
    pub window_maximized: bool,
    /// ダークテーマを使用するか
    pub dark_theme: bool,
    /// デフォルトディレクトリ
    pub default_directory: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_width: 1200,
            window_height: 800,
            window_x: None,
            window_y: None,
            min_window_width: 800,
            min_window_height: 600,
            window_maximized: false,
            dark_theme: true,
            default_directory: None,
        }
    }
}

impl Settings {
    /// 設定を読み込み
    pub fn load() -> Result<Self, AppError> {
        // 将来的にはJSONファイルから設定を読み込む
        // 現在はデフォルト設定を返す
        Ok(Self::default())
    }

    /// 設定を保存
    pub fn save(&self) -> Result<(), AppError> {
        // 将来的にはJSONファイルに設定を保存する
        // 現在は何もしない
        Ok(())
    }

    /// ウィンドウ幅を取得
    pub fn window_width(&self) -> u32 {
        self.window_width
    }

    /// ウィンドウ高さを取得
    pub fn window_height(&self) -> u32 {
        self.window_height
    }

    /// ウィンドウ位置を取得
    pub fn window_position(&self) -> (Option<i32>, Option<i32>) {
        (self.window_x, self.window_y)
    }

    /// ダークテーマ使用かどうか
    pub fn is_dark_theme(&self) -> bool {
        self.dark_theme
    }

    /// 最小ウィンドウサイズを取得
    pub fn min_window_size(&self) -> (u32, u32) {
        (self.min_window_width, self.min_window_height)
    }

    /// ウィンドウが最大化されているかどうか
    pub fn is_window_maximized(&self) -> bool {
        self.window_maximized
    }

    /// ウィンドウ状態を更新
    pub fn update_window_state(
        &mut self,
        width: u32,
        height: u32,
        x: Option<i32>,
        y: Option<i32>,
        maximized: bool,
    ) {
        self.window_width = width;
        self.window_height = height;
        self.window_x = x;
        self.window_y = y;
        self.window_maximized = maximized;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.window_width, 1200);
        assert_eq!(settings.window_height, 800);
        assert_eq!(settings.min_window_width, 800);
        assert_eq!(settings.min_window_height, 600);
        assert!(!settings.window_maximized);
        assert!(settings.dark_theme);
    }

    #[test]
    fn test_settings_load() {
        let result = Settings::load();
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.window_width(), 1200);
        assert_eq!(settings.window_height(), 800);
    }

    #[test]
    fn test_settings_save() {
        let settings = Settings::default();
        let result = settings.save();
        assert!(result.is_ok());
    }

    #[test]
    fn test_window_getters() {
        let settings = Settings::default();
        assert_eq!(settings.window_width(), 1200);
        assert_eq!(settings.window_height(), 800);
        assert_eq!(settings.window_position(), (None, None));
        assert_eq!(settings.min_window_size(), (800, 600));
        assert!(!settings.is_window_maximized());
        assert!(settings.is_dark_theme());
    }

    #[test]
    fn test_update_window_state() {
        let mut settings = Settings::default();
        settings.update_window_state(1024, 768, Some(100), Some(50), true);

        assert_eq!(settings.window_width, 1024);
        assert_eq!(settings.window_height, 768);
        assert_eq!(settings.window_x, Some(100));
        assert_eq!(settings.window_y, Some(50));
        assert!(settings.window_maximized);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: Result<Settings, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());

        let restored = deserialized.unwrap();
        assert_eq!(restored.window_width, settings.window_width);
        assert_eq!(restored.window_height, settings.window_height);
    }
}
