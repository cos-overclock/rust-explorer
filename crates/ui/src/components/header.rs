//! ヘッダー領域コンポーネント
//!
//! アプリケーションのヘッダー部分を提供します。

use floem::prelude::*;
use floem::text::Weight;

/// ヘッダーコンポーネントの設定
pub struct HeaderConfig {
    pub title: String,
    pub height: f32,
    pub background_color: Color,
    pub border_color: Color,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            title: "rust-explorer".to_string(),
            height: 40.0,
            background_color: Color::rgb8(240, 240, 240),
            border_color: Color::rgb8(200, 200, 200),
        }
    }
}

/// ヘッダーコンポーネントを作成
pub fn header_component(config: HeaderConfig) -> impl IntoView {
    h_stack((
        // アプリケーションタイトル
        label(move || config.title.clone()).style(|s| {
            s.font_size(18.0)
                .font_weight(Weight::BOLD)
                .margin_left(10.0)
                .color(Color::rgb8(50, 50, 50))
        }),
        // スペーサー（将来的にメニューやツールバーボタン用）
        container("").style(|s| s.flex_grow(1.0)),
        // 将来的なツールバーエリア
        create_toolbar_area(),
    ))
    .style(move |s| {
        s.width_full()
            .height(config.height)
            .background(config.background_color)
            .border_bottom(1.0)
            .border_color(config.border_color)
            .items_center()
    })
}

/// デフォルト設定でヘッダーコンポーネントを作成
pub fn default_header() -> impl IntoView {
    header_component(HeaderConfig::default())
}

/// ツールバーエリアの作成（将来の拡張用）
fn create_toolbar_area() -> impl IntoView {
    container("")
        .style(|s| s.margin_right(10.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_config_default() {
        let config = HeaderConfig::default();
        assert_eq!(config.title, "rust-explorer");
        assert_eq!(config.height, 40.0);
    }

    #[test]
    fn test_header_config_custom() {
        let config = HeaderConfig {
            title: "Custom Title".to_string(),
            height: 50.0,
            background_color: Color::rgb8(255, 255, 255),
            border_color: Color::rgb8(100, 100, 100),
        };
        assert_eq!(config.title, "Custom Title");
        assert_eq!(config.height, 50.0);
    }
}
