//! ステータスバー領域コンポーネント
//!
//! アプリケーションのステータスバー部分を提供します。

use floem::prelude::*;
use std::collections::HashMap;

/// ステータスバーコンポーネントの設定
pub struct StatusBarConfig {
    pub height: f32,
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub show_version: bool,
}

impl Default for StatusBarConfig {
    fn default() -> Self {
        Self {
            height: 25.0,
            background_color: Color::rgb8(230, 230, 230),
            border_color: Color::rgb8(200, 200, 200),
            text_color: Color::rgb8(70, 70, 70),
            show_version: true,
        }
    }
}

/// ステータス情報の種類
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatusType {
    /// メインステータス（左側）
    Main,
    /// ファイル情報
    FileInfo,
    /// 選択状態
    Selection,
    /// バージョン情報（右側）
    Version,
    /// カスタムステータス
    Custom(String),
}

/// ステータス情報のマップ
pub type StatusInfo = HashMap<StatusType, String>;

/// ステータスバーコンポーネントを作成
pub fn status_bar_component(config: StatusBarConfig, status_info: StatusInfo) -> impl IntoView {
    let text_color = config.text_color;
    let show_version = config.show_version;
    
    h_stack((
        // 左側エリア
        create_left_area(status_info.clone(), text_color),
        // スペーサー
        container("").style(|s| s.flex_grow(1.0)),
        // 右側エリア
        create_right_area(status_info, text_color, show_version),
    ))
    .style(move |s| {
        s.width_full()
            .height(config.height)
            .background(config.background_color)
            .border_top(1.0)
            .border_color(config.border_color)
            .items_center()
            .padding_horiz(10.0)
    })
}

/// デフォルト設定でステータスバーコンポーネントを作成
pub fn default_status_bar() -> impl IntoView {
    let mut status_info = StatusInfo::new();
    status_info.insert(StatusType::Main, "準備完了".to_string());
    
    status_bar_component(StatusBarConfig::default(), status_info)
}

/// ファイルエクスプローラー用のステータスバーを作成
pub fn file_explorer_status_bar(
    current_path: String,
    file_count: usize,
    selected_count: usize,
) -> impl IntoView {
    let mut status_info = StatusInfo::new();
    status_info.insert(StatusType::Main, format!("パス: {}", current_path));
    status_info.insert(StatusType::FileInfo, format!("ファイル数: {}", file_count));
    
    if selected_count > 0 {
        status_info.insert(StatusType::Selection, format!("選択中: {}", selected_count));
    }
    
    status_bar_component(StatusBarConfig::default(), status_info)
}

/// 左側エリアの作成
fn create_left_area(status_info: StatusInfo, text_color: Color) -> impl IntoView {
    let main_status = status_info
        .get(&StatusType::Main)
        .cloned()
        .unwrap_or_else(|| "準備完了".to_string());
    
    let file_info = status_info.get(&StatusType::FileInfo).cloned();
    let selection = status_info.get(&StatusType::Selection).cloned();

    h_stack((
        create_status_label(main_status, text_color),
        if let Some(file_info) = file_info {
            create_status_separator_and_label(file_info, text_color).into_any()
        } else {
            container("").into_any()
        },
        if let Some(selection) = selection {
            create_status_separator_and_label(selection, text_color).into_any()
        } else {
            container("").into_any()
        },
    ))
    .style(|s| s.items_center())
}

/// 右側エリアの作成
fn create_right_area(
    status_info: StatusInfo,
    text_color: Color,
    show_version: bool,
) -> impl IntoView {
    let version = if show_version {
        Some(status_info
            .get(&StatusType::Version)
            .cloned()
            .unwrap_or_else(|| format!("rust-explorer v{}", env!("CARGO_PKG_VERSION"))))
    } else {
        None
    };

    h_stack((
        if let Some(version_text) = version {
            create_status_label(version_text, text_color).into_any()
        } else {
            container("").into_any()
        },
    ))
    .style(|s| s.items_center())
}

/// ステータスラベルの作成
fn create_status_label(text: String, text_color: Color) -> impl IntoView {
    label(move || text.clone()).style(move |s| s.font_size(12.0).color(text_color))
}

/// 区切り線付きステータスラベルの作成
fn create_status_separator_and_label(text: String, text_color: Color) -> impl IntoView {
    h_stack((
        label(|| " | ").style(move |s| s.font_size(12.0).color(text_color.multiply_alpha(0.6))),
        create_status_label(text, text_color),
    ))
}

/// ステータス情報を更新するためのヘルパー関数
pub fn create_status_info() -> StatusInfo {
    StatusInfo::new()
}

/// ステータス情報にアイテムを追加するヘルパー関数
pub fn add_status_item(status_info: &mut StatusInfo, status_type: StatusType, message: String) {
    status_info.insert(status_type, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_config_default() {
        let config = StatusBarConfig::default();
        assert_eq!(config.height, 25.0);
        assert!(config.show_version);
    }

    #[test]
    fn test_status_info_creation() {
        let mut status_info = create_status_info();
        add_status_item(&mut status_info, StatusType::Main, "テスト".to_string());
        
        assert_eq!(
            status_info.get(&StatusType::Main),
            Some(&"テスト".to_string())
        );
    }

    #[test]
    fn test_status_type_variants() {
        let main = StatusType::Main;
        let file_info = StatusType::FileInfo;
        let selection = StatusType::Selection;
        let version = StatusType::Version;
        let custom = StatusType::Custom("test".to_string());

        assert_eq!(main, StatusType::Main);
        assert_eq!(file_info, StatusType::FileInfo);
        assert_eq!(selection, StatusType::Selection);
        assert_eq!(version, StatusType::Version);
        assert_eq!(custom, StatusType::Custom("test".to_string()));
    }

    #[test]
    fn test_file_explorer_status_creation() {
        let mut status_info = StatusInfo::new();
        status_info.insert(StatusType::Main, "/home/user".to_string());
        status_info.insert(StatusType::FileInfo, "ファイル数: 10".to_string());
        status_info.insert(StatusType::Selection, "選択中: 2".to_string());

        assert_eq!(status_info.len(), 3);
        assert!(status_info.contains_key(&StatusType::Main));
        assert!(status_info.contains_key(&StatusType::FileInfo));
        assert!(status_info.contains_key(&StatusType::Selection));
    }
}