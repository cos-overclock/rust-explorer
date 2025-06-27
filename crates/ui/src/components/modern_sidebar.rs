//! モダンなサイドバーコンポーネント
//!
//! Files CommunityとLapceにインスパイアされたモダンなサイドバー

use crate::theme::get_theme;
use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, button, container, h_stack, label, scroll, svg, text, v_stack};
use std::path::PathBuf;
use std::sync::Arc;

/// サイドバーセクション
#[derive(Debug, Clone)]
pub struct SidebarSection {
    pub title: String,
    pub items: Vec<SidebarItem>,
    pub collapsible: bool,
    pub collapsed: bool,
}

/// サイドバーアイテム
#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub path: Option<PathBuf>,
    pub item_type: SidebarItemType,
    pub selected: bool,
    pub badge_count: Option<u32>,
}

/// サイドバーアイテムのタイプ
#[derive(Debug, Clone, PartialEq)]
pub enum SidebarItemType {
    /// クイックアクセス
    QuickAccess,
    /// ドライブ
    Drive,
    /// お気に入り
    Favorite,
    /// 最近使用したファイル
    Recent,
    /// フォルダ
    Folder,
    /// タグ
    Tag,
}

/// モダンサイドバーの設定
#[derive(Debug, Clone)]
pub struct ModernSidebarConfig {
    /// サイドバーの幅
    pub width: f32,
    /// 最小幅
    pub min_width: f32,
    /// 最大幅
    pub max_width: f32,
    /// リサイズ可能か
    pub resizable: bool,
    /// 折りたたみ可能か
    pub collapsible: bool,
    /// 初期状態で表示するか
    pub initially_visible: bool,
}

impl Default for ModernSidebarConfig {
    fn default() -> Self {
        Self {
            width: 280.0,
            min_width: 200.0,
            max_width: 400.0,
            resizable: true,
            collapsible: true,
            initially_visible: true,
        }
    }
}

/// モダンサイドバー管理
pub struct ModernSidebar {
    config: ModernSidebarConfig,
    sections: RwSignal<Vec<SidebarSection>>,
    visible: RwSignal<bool>,
    width: RwSignal<f32>,
}

impl ModernSidebar {
    /// 新しいモダンサイドバーを作成
    pub fn new(config: ModernSidebarConfig) -> Self {
        let sections = vec![
            // クイックアクセスセクション
            SidebarSection {
                title: "クイックアクセス".to_string(),
                collapsible: false,
                collapsed: false,
                items: vec![
                    SidebarItem {
                        id: "desktop".to_string(),
                        label: "デスクトップ".to_string(),
                        icon: r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M21,16H3V4H21M21,2H3C1.89,2 1,2.89 1,4V16A2,2 0 0,0 3,18H10V20H8V22H16V20H14V18H21A2,2 0 0,0 23,16V4C23,2.89 22.1,2 21,2Z"/>
                        </svg>"#.to_string(),
                        path: Some(get_desktop_path()),
                        item_type: SidebarItemType::QuickAccess,
                        selected: false,
                        badge_count: None,
                    },
                    SidebarItem {
                        id: "documents".to_string(),
                        label: "ドキュメント".to_string(),
                        icon: r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/>
                        </svg>"#.to_string(),
                        path: Some(get_documents_path()),
                        item_type: SidebarItemType::QuickAccess,
                        selected: false,
                        badge_count: None,
                    },
                    SidebarItem {
                        id: "downloads".to_string(),
                        label: "ダウンロード".to_string(),
                        icon: r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z"/>
                        </svg>"#.to_string(),
                        path: Some(get_downloads_path()),
                        item_type: SidebarItemType::QuickAccess,
                        selected: false,
                        badge_count: None,
                    },
                    SidebarItem {
                        id: "pictures".to_string(),
                        label: "ピクチャ".to_string(),
                        icon: r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M8.5,13.5L11,16.5L14.5,12L19,18H5M21,19V5C21,3.89 20.1,3 19,3H5A2,2 0 0,0 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19Z"/>
                        </svg>"#.to_string(),
                        path: Some(get_pictures_path()),
                        item_type: SidebarItemType::QuickAccess,
                        selected: false,
                        badge_count: None,
                    },
                ],
            },

            // このPCセクション
            SidebarSection {
                title: "このPC".to_string(),
                collapsible: true,
                collapsed: false,
                items: vec![
                    SidebarItem {
                        id: "drive_c".to_string(),
                        label: "ローカルディスク (C:)".to_string(),
                        icon: r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M6,2H18A2,2 0 0,1 20,4V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V4A2,2 0 0,1 6,2M12,4A6,6 0 0,0 6,10C6,13.31 8.69,16 12.1,16L11.22,13.77C10.95,13.29 11.11,12.68 11.59,12.4L12.45,11.9C12.93,11.63 13.54,11.79 13.82,12.27L15.74,14.69C17.12,13.59 18,11.9 18,10A6,6 0 0,0 12,4Z"/>
                        </svg>"#.to_string(),
                        path: Some(PathBuf::from("/")),
                        item_type: SidebarItemType::Drive,
                        selected: false,
                        badge_count: None,
                    },
                ],
            },

            // お気に入りセクション
            SidebarSection {
                title: "お気に入り".to_string(),
                collapsible: true,
                collapsed: false,
                items: vec![],
            },
        ];

        Self {
            sections: RwSignal::new(sections),
            visible: RwSignal::new(config.initially_visible),
            width: RwSignal::new(config.width),
            config,
        }
    }

    /// デフォルト設定でモダンサイドバーを作成
    pub fn with_default() -> Self {
        Self::new(ModernSidebarConfig::default())
    }

    /// サイドバーの表示/非表示を切り替え
    pub fn toggle_visibility(&self) {
        self.visible.update(|visible| *visible = !*visible);
    }

    /// サイドバーの幅を設定
    pub fn set_width(&self, width: f32) {
        let clamped_width = width.clamp(self.config.min_width, self.config.max_width);
        self.width.set(clamped_width);
    }

    /// アイテムを選択
    pub fn select_item(&self, item_id: String) {
        self.sections.update(|sections| {
            for section in sections.iter_mut() {
                for item in section.items.iter_mut() {
                    item.selected = item.id == item_id;
                }
            }
        });
    }

    /// セクションの折りたたみ状態を切り替え
    pub fn toggle_section(&self, section_title: String) {
        self.sections.update(|sections| {
            for section in sections.iter_mut() {
                if section.title == section_title && section.collapsible {
                    section.collapsed = !section.collapsed;
                    break;
                }
            }
        });
    }

    /// サイドバービューを作成
    pub fn build(self) -> impl IntoView {
        let visible = self.visible;
        let width = self.width;
        let _sections = self.sections;
        let sidebar_self = Arc::new(self);

        container(if visible.get() {
            container(scroll(
                v_stack((
                    // サイドバーヘッダー
                    create_sidebar_header(),
                    // セクションリスト（簡略版 - 初期セクションのみ）
                    v_stack((create_sidebar_section(
                        SidebarSection {
                            title: "クイックアクセス".to_string(),
                            collapsible: false,
                            collapsed: false,
                            items: vec![],
                        },
                        sidebar_self.clone(),
                    ),))
                    .style(move |s| {
                        let theme_arc = get_theme();
                        let theme = theme_arc.read().unwrap();
                        s.gap(theme.spacing.sm)
                    }),
                ))
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.gap(theme.spacing.md).padding(theme.spacing.md)
                }),
            ))
            .style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.width(width.get())
                    .height_full()
                    .background(theme.colors.surface)
                    .border_right(1.0)
                    .border_color(theme.colors.border)
            })
            .into_any()
        } else {
            container(text(""))
                .style(|s| s.display(floem::style::Display::None))
                .into_any()
        })
        .style(|s| s.height_full())
    }
}

/// サイドバーヘッダーを作成
fn create_sidebar_header() -> impl IntoView {
    h_stack((label(|| "ナビゲーション").style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.font_size(theme.typography.title_small)
            .font_weight(floem::text::Weight::SEMIBOLD)
            .color(theme.colors.on_surface)
    }),))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width_full()
            .items_center()
            .justify_between()
            .padding_bottom(theme.spacing.sm)
            .border_bottom(1.0)
            .border_color(theme.colors.border_variant)
    })
}

/// サイドバーセクションを作成
fn create_sidebar_section(section: SidebarSection, sidebar: Arc<ModernSidebar>) -> impl IntoView {
    let section_title = section.title.clone();
    let collapsible = section.collapsible;
    let collapsed = section.collapsed;

    v_stack((
        // セクションヘッダー
        create_section_header(
            section_title.clone(),
            collapsible,
            collapsed,
            sidebar.clone(),
        ),
        // セクションアイテム（簡略版）
        if !collapsed && !section.items.is_empty() {
            container(text("項目があります"))
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.gap(theme.spacing.xs).margin_left(theme.spacing.md)
                })
                .into_any()
        } else {
            container(text(""))
                .style(|s| s.display(floem::style::Display::None))
                .into_any()
        },
    ))
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width_full().gap(theme.spacing.sm)
    })
}

/// セクションヘッダーを作成
fn create_section_header(
    title: String,
    collapsible: bool,
    collapsed: bool,
    sidebar: Arc<ModernSidebar>,
) -> impl IntoView {
    let title_for_label = title.clone();
    let title_for_action = title.clone();

    if collapsible {
        button(
            h_stack((
                // 折りたたみアイコン
                svg({
                    let icon_svg = if collapsed {
                        r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z"/>
                        </svg>"#
                    } else {
                        r#"<svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/>
                        </svg>"#
                    };
                    icon_svg.to_string()
                })
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.width(16.0)
                        .height(16.0)
                        .color(theme.colors.on_surface_variant)
                }),
                // セクションタイトル
                label(move || title_for_label.clone()).style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.font_size(theme.typography.label_large)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(theme.colors.on_surface_variant)
                        .margin_left(theme.spacing.sm)
                }),
            ))
            .style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.items_center().gap(theme.spacing.sm)
            }),
        )
        .action(move || sidebar.toggle_section(title_for_action.clone()))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.width_full()
                .padding_vert(theme.spacing.sm)
                .border(0.0)
                .background(Color::TRANSPARENT)
                .cursor(floem::style::CursorStyle::Pointer)
                .hover(move |s| s.background(theme.colors.hover))
        })
        .into_any()
    } else {
        label(move || title.clone())
            .style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.font_size(theme.typography.label_large)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(theme.colors.on_surface_variant)
                    .padding_vert(theme.spacing.sm)
            })
            .into_any()
    }
}

/// サイドバーアイテムを作成
#[allow(dead_code)]
fn create_sidebar_item(item: SidebarItem, sidebar: Arc<ModernSidebar>) -> impl IntoView {
    let item_id = item.id.clone();
    let selected = item.selected;

    button(
        h_stack((
            // アイコン
            svg(item.icon.clone()).style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.width(20.0).height(20.0).color(if selected {
                    theme.colors.primary
                } else {
                    theme.colors.on_surface_variant
                })
            }),
            // ラベル
            label(move || item.label.clone()).style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.font_size(theme.typography.body_medium)
                    .color(if selected {
                        theme.colors.primary
                    } else {
                        theme.colors.on_surface
                    })
                    .flex()
            }),
            // バッジ（オプション）
            if let Some(count) = item.badge_count {
                container(label(move || count.to_string()).style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.font_size(theme.typography.label_small)
                        .color(theme.colors.on_primary)
                }))
                .style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.min_width(20.0)
                        .height(20.0)
                        .padding_horiz(theme.spacing.sm)
                        .border_radius(theme.border_radius.full)
                        .background(theme.colors.primary)
                        .items_center()
                        .justify_center()
                })
                .into_any()
            } else {
                container(text("")).into_any()
            },
        ))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.items_center().gap(theme.spacing.md).width_full()
        }),
    )
    .action(move || {
        sidebar.select_item(item_id.clone());
    })
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let mut style = s
            .width_full()
            .padding_horiz(theme.spacing.md)
            .padding_vert(theme.spacing.sm)
            .border_radius(theme.border_radius.sm)
            .border(0.0)
            .cursor(floem::style::CursorStyle::Pointer);

        if selected {
            style = style.background(theme.colors.selected);
        } else {
            style = style
                .background(Color::TRANSPARENT)
                .hover(move |s| s.background(theme.colors.hover));
        }

        style
    })
}

/// プラットフォーム固有のパスを取得
fn get_desktop_path() -> PathBuf {
    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("/home"))
}

fn get_documents_path() -> PathBuf {
    dirs::document_dir().unwrap_or_else(|| PathBuf::from("/home"))
}

fn get_downloads_path() -> PathBuf {
    dirs::download_dir().unwrap_or_else(|| PathBuf::from("/home"))
}

fn get_pictures_path() -> PathBuf {
    dirs::picture_dir().unwrap_or_else(|| PathBuf::from("/home"))
}

/// デフォルトのモダンサイドバーを作成
pub fn default_modern_sidebar() -> impl IntoView {
    ModernSidebar::with_default().build()
}

/// カスタム設定のモダンサイドバーを作成
pub fn modern_sidebar_component(config: ModernSidebarConfig) -> impl IntoView {
    ModernSidebar::new(config).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_sidebar_config() {
        let config = ModernSidebarConfig::default();
        assert_eq!(config.width, 280.0);
        assert_eq!(config.min_width, 200.0);
        assert_eq!(config.max_width, 400.0);
        assert!(config.resizable);
        assert!(config.collapsible);
        assert!(config.initially_visible);
    }

    #[test]
    fn test_sidebar_item_types() {
        assert_eq!(SidebarItemType::QuickAccess, SidebarItemType::QuickAccess);
        assert_ne!(SidebarItemType::Drive, SidebarItemType::Folder);
    }

    #[test]
    fn test_sidebar_creation() {
        let sidebar = ModernSidebar::with_default();
        assert!(sidebar.visible.get());
        assert_eq!(sidebar.width.get(), 280.0);
    }

    #[test]
    fn test_sidebar_toggle() {
        let sidebar = ModernSidebar::with_default();
        assert!(sidebar.visible.get());

        sidebar.toggle_visibility();
        assert!(!sidebar.visible.get());

        sidebar.toggle_visibility();
        assert!(sidebar.visible.get());
    }

    #[test]
    fn test_width_clamping() {
        let sidebar = ModernSidebar::with_default();

        sidebar.set_width(100.0); // Too small
        assert_eq!(sidebar.width.get(), 200.0); // Clamped to min

        sidebar.set_width(500.0); // Too large
        assert_eq!(sidebar.width.get(), 400.0); // Clamped to max

        sidebar.set_width(300.0); // Valid
        assert_eq!(sidebar.width.get(), 300.0);
    }
}
