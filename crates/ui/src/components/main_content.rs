//! メインコンテンツ領域コンポーネント
//!
//! アプリケーションのメインコンテンツ部分を提供します。

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::text::Weight;
use rust_explorer_config::Settings;
use std::cell::RefCell;
use std::rc::Rc;

/// メインコンテンツコンポーネントの設定
pub struct MainContentConfig {
    pub background_color: Color,
    pub padding: f32,
    pub content_type: ContentType,
}

/// コンテンツタイプの定義
#[derive(Clone, Debug)]
pub enum ContentType {
    /// ウェルカム画面
    Welcome,
    /// ファイルエクスプローラー
    FileExplorer,
    /// エラー表示
    Error(String),
}

impl Default for MainContentConfig {
    fn default() -> Self {
        Self {
            background_color: Color::rgb8(250, 250, 250),
            padding: 20.0,
            content_type: ContentType::Welcome,
        }
    }
}

/// メインコンテンツコンポーネントを作成
pub fn main_content_component(
    config: MainContentConfig,
    settings: Rc<RefCell<Settings>>,
) -> impl IntoView {
    container(match config.content_type {
        ContentType::Welcome => create_welcome_content().into_any(),
        ContentType::FileExplorer => create_file_explorer_content(settings).into_any(),
        ContentType::Error(message) => create_error_content(message).into_any(),
    })
    .style(move |s| {
        s.size_full()
            .background(config.background_color)
            .padding(config.padding)
    })
}

/// デフォルト設定でメインコンテンツコンポーネントを作成
pub fn default_main_content(settings: Rc<RefCell<Settings>>) -> impl IntoView {
    main_content_component(MainContentConfig::default(), settings)
}

/// ウェルカムコンテンツの作成
fn create_welcome_content() -> impl IntoView {
    let counter = RwSignal::new(0);

    v_stack((
        label(|| "rust-explorer - Main Window").style(|s| {
            s.font_size(24.0)
                .font_weight(Weight::BOLD)
                .margin_bottom(20.0)
                .color(Color::rgb8(50, 50, 50))
        }),
        label(|| "基本レイアウトシステムが実装されました")
            .style(|s| s.font_size(16.0).margin_bottom(30.0)),
        // インタラクティブなデモエリア
        create_demo_section(counter),
        // 機能紹介エリア
        create_features_section(),
    ))
    .style(|s| s.items_center().justify_center().gap(20.0))
}

/// デモセクションの作成
fn create_demo_section(mut counter: RwSignal<i32>) -> impl IntoView {
    v_stack((
        label(|| "インタラクティブデモ").style(|s| {
            s.font_size(18.0)
                .font_weight(Weight::BOLD)
                .margin_bottom(15.0)
        }),
        h_stack((
            button("カウント +")
                .action(move || counter += 1)
                .style(|s| s.margin_right(10.0).padding(8.0)),
            label(move || format!("カウント: {}", counter.get()))
                .style(|s| s.font_size(16.0).margin_right(10.0)),
            button("カウント -")
                .action(move || counter -= 1)
                .style(|s| s.padding(8.0)),
        ))
        .style(|s| s.gap(10.0).items_center()),
    ))
    .style(|s| s.items_center())
}

/// 機能紹介セクションの作成
fn create_features_section() -> impl IntoView {
    v_stack((
        label(|| "実装済み機能").style(|s| {
            s.font_size(18.0)
                .font_weight(Weight::BOLD)
                .margin_bottom(15.0)
        }),
        v_stack((
            create_feature_item(
                "✓ 基本レイアウトシステム",
                "ヘッダー・メイン・ステータスバー構造",
            ),
            create_feature_item("✓ レスポンシブデザイン", "ウィンドウリサイズに対応"),
            create_feature_item("✓ コンポーネント分離", "再利用可能な設計"),
            create_feature_item("✓ 設定管理", "ウィンドウサイズ・位置の永続化"),
            create_feature_item("⚠ 最小サイズ制限", "floem 0.2制限により警告のみ"),
        ))
        .style(|s| s.gap(8.0)),
    ))
    .style(|s| s.items_center())
}

/// 機能アイテムの作成
fn create_feature_item(title: &'static str, description: &'static str) -> impl IntoView {
    v_stack((
        label(move || title).style(|s| {
            s.font_size(14.0)
                .font_weight(Weight::BOLD)
                .color(Color::rgb8(50, 50, 50))
        }),
        label(move || description).style(|s| {
            s.font_size(12.0)
                .color(Color::rgb8(100, 100, 100))
                .margin_left(20.0)
        }),
    ))
    .style(|s| s.items_start())
}

/// ファイルエクスプローラーコンテンツの作成（将来の実装用）
fn create_file_explorer_content(_settings: Rc<RefCell<Settings>>) -> impl IntoView {
    v_stack((
        label(|| "ファイルエクスプローラー").style(|s| {
            s.font_size(20.0)
                .font_weight(Weight::BOLD)
                .margin_bottom(20.0)
        }),
        label(|| "ファイルエクスプローラー機能は将来のバージョンで実装予定です")
            .style(|s| s.font_size(14.0)),
    ))
    .style(|s| s.items_center().justify_center())
}

/// エラーコンテンツの作成
fn create_error_content(message: String) -> impl IntoView {
    v_stack((
        label(|| "エラー").style(|s| {
            s.font_size(20.0)
                .font_weight(Weight::BOLD)
                .color(Color::rgb8(220, 53, 69))
                .margin_bottom(20.0)
        }),
        label(move || message.clone())
            .style(|s| s.font_size(14.0).color(Color::rgb8(108, 117, 125))),
    ))
    .style(|s| s.items_center().justify_center())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_explorer_config::Settings;

    #[test]
    fn test_main_content_config_default() {
        let config = MainContentConfig::default();
        assert_eq!(config.padding, 20.0);
        matches!(config.content_type, ContentType::Welcome);
    }

    #[test]
    fn test_content_type_variants() {
        let welcome = ContentType::Welcome;
        let file_explorer = ContentType::FileExplorer;
        let error = ContentType::Error("Test error".to_string());

        matches!(welcome, ContentType::Welcome);
        matches!(file_explorer, ContentType::FileExplorer);
        matches!(error, ContentType::Error(_));
    }

    #[test]
    fn test_main_content_config_custom() {
        let config = MainContentConfig {
            background_color: Color::rgb8(255, 255, 255),
            padding: 30.0,
            content_type: ContentType::FileExplorer,
        };
        assert_eq!(config.padding, 30.0);
        matches!(config.content_type, ContentType::FileExplorer);
    }
}
