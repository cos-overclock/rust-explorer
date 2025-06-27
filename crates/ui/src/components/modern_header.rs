//! モダンなヘッダーコンポーネント
//!
//! Files CommunityとLapceにインスパイアされたモダンなヘッダーデザイン

use crate::theme::get_theme;
use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, button, container, h_stack, label, svg, text};

/// モダンヘッダーの設定
#[derive(Debug, Clone)]
pub struct ModernHeaderConfig {
    /// ヘッダーの高さ
    pub height: f32,
    /// タイトルを表示するか
    pub show_title: bool,
    /// ツールバーを表示するか
    pub show_toolbar: bool,
    /// ウィンドウコントロールを表示するか
    pub show_window_controls: bool,
}

impl Default for ModernHeaderConfig {
    fn default() -> Self {
        Self {
            height: 48.0,
            show_title: true,
            show_toolbar: true,
            show_window_controls: true,
        }
    }
}

/// モダンヘッダーコンポーネント
pub struct ModernHeader {
    config: ModernHeaderConfig,
    title: RwSignal<String>,
}

impl ModernHeader {
    /// 新しいモダンヘッダーを作成
    pub fn new(config: ModernHeaderConfig) -> Self {
        Self {
            config,
            title: RwSignal::new("rust-explorer".to_string()),
        }
    }

    /// デフォルト設定でモダンヘッダーを作成
    pub fn with_default() -> Self {
        Self::new(ModernHeaderConfig::default())
    }

    /// タイトルを設定
    pub fn set_title(&self, title: String) {
        self.title.set(title);
    }

    /// ヘッダービューを作成
    pub fn build(self) -> impl IntoView {
        let title = self.title;
        let config = self.config;

        container(
            h_stack((
                // アプリロゴとタイトル
                create_app_branding(title, config.show_title),
                // 中央のツールバーエリア
                if config.show_toolbar {
                    create_toolbar().into_any()
                } else {
                    container(text("")).style(|s| s.flex()).into_any()
                },
                // 右側のコントロール
                if config.show_window_controls {
                    create_window_controls().into_any()
                } else {
                    container(text("")).into_any()
                },
            ))
            .style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.height(config.height)
                    .width_full()
                    .items_center()
                    .padding_horiz(theme.spacing.md)
                    .gap(theme.spacing.md)
            }),
        )
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.width_full()
                .background(theme.colors.surface)
                .border_bottom(1.0)
                .border_color(theme.colors.border)
        })
    }
}

/// アプリのブランディング部分を作成
fn create_app_branding(title: RwSignal<String>, show_title: bool) -> impl IntoView {
    h_stack((
        // アプリアイコン
        create_app_icon(),
        // アプリタイトル
        if show_title {
            label(move || title.get())
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.font_size(theme.typography.title_medium)
                        .font_weight(floem::text::Weight::SEMIBOLD)
                        .color(theme.colors.on_surface)
                        .margin_left(theme.spacing.sm)
                })
                .into_any()
        } else {
            container(text("")).into_any()
        },
    ))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.items_center().gap(theme.spacing.sm)
    })
}

/// アプリアイコンを作成
fn create_app_icon() -> impl IntoView {
    // SVGアイコン（フォルダアイコン）
    let icon_svg = r#"
        <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z"/>
        </svg>
    "#;

    container(svg(icon_svg.trim().to_string()).style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width(24.0).height(24.0).color(theme.colors.primary)
    }))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width(32.0)
            .height(32.0)
            .items_center()
            .justify_center()
            .border_radius(theme.border_radius.md)
            .background(theme.colors.primary.multiply_alpha(0.1))
    })
}

/// ツールバーエリアを作成
fn create_toolbar() -> impl IntoView {
    h_stack((
        // 検索バー（簡略版）
        create_search_bar(),
        // ビュー切り替えボタン
        create_view_toggle_buttons(),
    ))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.flex()
            .items_center()
            .justify_center()
            .gap(theme.spacing.lg)
    })
}

/// 検索バーを作成
fn create_search_bar() -> impl IntoView {
    container(
        h_stack((
            // 検索アイコン
            svg(r#"
                <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
                </svg>
            "#.trim().to_string())
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.width(16.0)
                        .height(16.0)
                        .color(theme.colors.on_surface_variant)
                }),

            // 検索プレースホルダー
            label(|| "ファイルを検索...")
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.font_size(theme.typography.body_medium)
                        .color(theme.colors.on_surface_variant)
                        .margin_left(theme.spacing.sm)
                }),
        ))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.items_center()
                .gap(theme.spacing.sm)
        })
    )
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let surface_container = theme.colors.surface_container;
        let border_color = theme.colors.border;
        let border_focus = theme.colors.border_focus;
        let surface = theme.colors.surface;
        s.width(300.0)
            .height(32.0)
            .padding_horiz(theme.spacing.md)
            .border_radius(theme.border_radius.full)
            .background(theme.colors.surface_variant)
            .border(1.0)
            .border_color(Color::TRANSPARENT)
            .items_center()
            .cursor(floem::style::CursorStyle::Text)
            .hover(move |s| {
                s.background(surface_container)
                    .border_color(border_color)
            })
            .focus(move |s| {
                s.border_color(border_focus)
                    .background(surface)
            })
    })
}

/// ビュー切り替えボタンを作成
fn create_view_toggle_buttons() -> impl IntoView {
    h_stack((
        // リストビューボタン
        create_icon_button(
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"/>
            </svg>"#,
            "リストビュー"
        ),

        // グリッドビューボタン
        create_icon_button(
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M4 11h5V5H4v6zm0 7h5v-6H4v6zm6 0h5v-6h-5v6zm6 0h5v-6h-5v6zm-6-7h5V5h-5v6zm6-6v6h5V5h-5z"/>
            </svg>"#,
            "グリッドビュー"
        ),
    ))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.gap(theme.spacing.xs)
            .padding(theme.spacing.xs)
            .border_radius(theme.border_radius.md)
            .background(theme.colors.surface_variant)
    })
}

/// アイコンボタンを作成
fn create_icon_button(icon_svg: &'static str, _tooltip: &'static str) -> impl IntoView {
    button(svg(icon_svg.to_string()).style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width(16.0)
            .height(16.0)
            .color(theme.colors.on_surface_variant)
    }))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let hover_color = theme.colors.hover;
        let pressed_color = theme.colors.pressed;
        s.width(32.0)
            .height(32.0)
            .border_radius(theme.border_radius.sm)
            .border(0.0)
            .background(Color::TRANSPARENT)
            .items_center()
            .justify_center()
            .cursor(floem::style::CursorStyle::Pointer)
            .hover(move |s| s.background(hover_color))
            .active(move |s| s.background(pressed_color))
    })
}

/// ウィンドウコントロールを作成
fn create_window_controls() -> impl IntoView {
    h_stack((
        // 最小化ボタン
        create_window_control_button(
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19h12v2H6z"/>
            </svg>"#,
            WindowControlType::Minimize
        ),

        // 最大化ボタン
        create_window_control_button(
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M4 4h16v16H4V4zm2 2v12h12V6H6z"/>
            </svg>"#,
            WindowControlType::Maximize
        ),

        // 閉じるボタン
        create_window_control_button(
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>"#,
            WindowControlType::Close
        ),
    ))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.gap(theme.spacing.xs)
    })
}

/// ウィンドウコントロールの種類
#[derive(Debug, Clone, Copy)]
enum WindowControlType {
    Minimize,
    Maximize,
    Close,
}

/// ウィンドウコントロールボタンを作成
fn create_window_control_button(
    icon_svg: &'static str,
    control_type: WindowControlType,
) -> impl IntoView {
    button(svg(icon_svg.to_string()).style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width(12.0)
            .height(12.0)
            .color(theme.colors.on_surface_variant)
    }))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let hover_color = theme.colors.hover;
        let pressed_color = theme.colors.pressed;
        let base_style = s
            .width(36.0)
            .height(32.0)
            .border_radius(0.0)
            .border(0.0)
            .background(Color::TRANSPARENT)
            .items_center()
            .justify_center()
            .cursor(floem::style::CursorStyle::Pointer);

        match control_type {
            WindowControlType::Close => base_style
                .hover(|s| s.background(Color::rgb8(196, 43, 28)))
                .active(|s| s.background(Color::rgb8(166, 23, 8))),
            _ => base_style
                .hover(move |s| s.background(hover_color))
                .active(move |s| s.background(pressed_color)),
        }
    })
}

/// デフォルトのモダンヘッダーを作成
pub fn default_modern_header() -> impl IntoView {
    ModernHeader::with_default().build()
}

/// カスタム設定のモダンヘッダーを作成
pub fn modern_header_component(config: ModernHeaderConfig) -> impl IntoView {
    ModernHeader::new(config).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_header_config() {
        let config = ModernHeaderConfig::default();
        assert_eq!(config.height, 48.0);
        assert!(config.show_title);
        assert!(config.show_toolbar);
        assert!(config.show_window_controls);
    }

    #[test]
    fn test_modern_header_creation() {
        let header = ModernHeader::with_default();
        assert_eq!(header.title.get(), "rust-explorer");

        header.set_title("New Title".to_string());
        assert_eq!(header.title.get(), "New Title");
    }
}
