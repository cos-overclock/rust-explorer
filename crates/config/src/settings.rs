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
}
