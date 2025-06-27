//! モダンなファイルアイテムコンポーネント
//!
//! Files CommunityとLapceにインスパイアされたモダンなファイル表示

use crate::theme::get_theme;
use floem::IntoView;
use floem::peniko::Color;
use floem::views::{Decorators, container, h_stack, label, svg, text, v_stack};
use rust_explorer_core::{FileEntry, FileType};
use std::time::SystemTime;

/// ファイルアイテムの表示モード
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileItemDisplayMode {
    /// リスト表示
    List,
    /// グリッド表示
    Grid,
    /// コンパクト表示
    Compact,
}

/// モダンファイルアイテムの設定
#[derive(Debug, Clone)]
pub struct ModernFileItemConfig {
    /// 表示モード
    pub display_mode: FileItemDisplayMode,
    /// アイコンサイズ
    pub icon_size: f32,
    /// 選択状態を表示するか
    pub show_selection: bool,
    /// 詳細情報を表示するか
    pub show_details: bool,
    /// ホバーエフェクトを有効にするか
    pub enable_hover: bool,
}

impl Default for ModernFileItemConfig {
    fn default() -> Self {
        Self {
            display_mode: FileItemDisplayMode::List,
            icon_size: 24.0,
            show_selection: true,
            show_details: true,
            enable_hover: true,
        }
    }
}

/// モダンファイルアイテムビューを作成
pub fn modern_file_item_view(
    entry: FileEntry,
    selected: bool,
    config: ModernFileItemConfig,
) -> impl IntoView {
    let _theme = get_theme();
    let entry_for_icon = entry.clone();
    let entry_for_name = entry.clone();
    let entry_for_details = entry;

    match config.display_mode {
        FileItemDisplayMode::List => create_list_item(
            entry_for_icon,
            entry_for_name,
            entry_for_details,
            selected,
            config,
        )
        .into_any(),
        FileItemDisplayMode::Grid => create_grid_item(
            entry_for_icon,
            entry_for_name,
            entry_for_details,
            selected,
            config,
        )
        .into_any(),
        FileItemDisplayMode::Compact => {
            create_compact_item(entry_for_icon, entry_for_name, selected, config).into_any()
        }
    }
}

/// リスト表示アイテムを作成
fn create_list_item(
    entry_icon: FileEntry,
    entry_name: FileEntry,
    entry_details: FileEntry,
    selected: bool,
    config: ModernFileItemConfig,
) -> impl IntoView {
    container(
        h_stack((
            // ファイルアイコン
            create_modern_file_icon(&entry_icon, config.icon_size),
            // ファイル情報
            h_stack((
                // ファイル名
                label(move || entry_name.name.clone()).style(move |s| {
                    let theme_arc = get_theme();
                    let theme = theme_arc.read().unwrap();
                    s.font_size(theme.typography.body_medium)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(if selected {
                            theme.colors.on_primary
                        } else {
                            theme.colors.on_surface
                        })
                        .flex()
                        .min_width(0.0) // Allow text truncation
                }),
                // 詳細情報
                if config.show_details {
                    h_stack((
                        // ファイルサイズ
                        label(move || format_file_size(entry_details.size)).style(move |s| {
                            let theme_arc = get_theme();
                            let theme = theme_arc.read().unwrap();
                            s.font_size(theme.typography.body_small)
                                .color(if selected {
                                    theme.colors.on_primary.multiply_alpha(0.8)
                                } else {
                                    theme.colors.on_surface_variant
                                })
                                .width(80.0)
                        }),
                        // 更新日時
                        label(move || format_modified_time(entry_details.modified)).style(
                            move |s| {
                                let theme_arc = get_theme();
                                let theme = theme_arc.read().unwrap();
                                s.font_size(theme.typography.body_small)
                                    .color(if selected {
                                        theme.colors.on_primary.multiply_alpha(0.8)
                                    } else {
                                        theme.colors.on_surface_variant
                                    })
                                    .width(120.0)
                            },
                        ),
                    ))
                    .style(move |s| {
                        let theme_arc = get_theme();
                        let theme = theme_arc.read().unwrap();
                        s.gap(theme.spacing.lg)
                    })
                    .into_any()
                } else {
                    container(text("")).into_any()
                },
            ))
            .style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.flex()
                    .items_center()
                    .justify_between()
                    .min_width(0.0)
                    .gap(theme.spacing.md)
            }),
        ))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.items_center().gap(theme.spacing.md).width_full()
        }),
    )
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let mut style = s
            .width_full()
            .height(40.0)
            .padding_horiz(theme.spacing.md)
            .padding_vert(theme.spacing.sm)
            .border_radius(theme.border_radius.sm)
            .cursor(floem::style::CursorStyle::Pointer);

        if selected {
            style = style.background(theme.colors.primary);
        } else if config.enable_hover {
            let hover_color = theme.colors.hover;
            style = style.hover(move |s| s.background(hover_color));
        }

        style
    })
}

/// グリッド表示アイテムを作成
fn create_grid_item(
    entry_icon: FileEntry,
    entry_name: FileEntry,
    entry_details: FileEntry,
    selected: bool,
    config: ModernFileItemConfig,
) -> impl IntoView {
    let grid_icon_size = config.icon_size * 2.0; // グリッドでは大きなアイコン

    container(
        v_stack((
            // 大きなファイルアイコン
            create_modern_file_icon(&entry_icon, grid_icon_size),
            // ファイル名
            label(move || entry_name.name.clone()).style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.font_size(theme.typography.body_small)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(if selected {
                        theme.colors.on_primary
                    } else {
                        theme.colors.on_surface
                    })
                    .line_height(theme.typography.line_height_tight)
                // .max_lines(2) // floem 0.2 では利用不可
            }),
            // 詳細情報（オプション）
            if config.show_details && entry_details.file_type == FileType::File {
                label(move || format_file_size(entry_details.size))
                    .style(move |s| {
                        let theme_arc = get_theme();
                        let theme = theme_arc.read().unwrap();
                        s.font_size(theme.typography.label_small)
                            .color(if selected {
                                theme.colors.on_primary.multiply_alpha(0.7)
                            } else {
                                theme.colors.on_surface_variant
                            })
                    })
                    .into_any()
            } else {
                container(text("")).into_any()
            },
        ))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.items_center()
                .gap(theme.spacing.sm)
                .padding(theme.spacing.md)
        }),
    )
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let mut style = s
            .width(120.0)
            .height(120.0)
            .border_radius(theme.border_radius.md)
            .cursor(floem::style::CursorStyle::Pointer)
            .items_center()
            .justify_center();

        if selected {
            style = style.background(theme.colors.primary);
        } else if config.enable_hover {
            style = style.hover(move |s| s.background(theme.colors.hover));
        }

        style
    })
}

/// コンパクト表示アイテムを作成
fn create_compact_item(
    entry_icon: FileEntry,
    entry_name: FileEntry,
    selected: bool,
    config: ModernFileItemConfig,
) -> impl IntoView {
    let compact_icon_size = config.icon_size * 0.8; // コンパクトでは小さなアイコン

    container(
        h_stack((
            // 小さなファイルアイコン
            create_modern_file_icon(&entry_icon, compact_icon_size),
            // ファイル名
            label(move || entry_name.name.clone()).style(move |s| {
                let theme_arc = get_theme();
                let theme = theme_arc.read().unwrap();
                s.font_size(theme.typography.body_small)
                    .color(if selected {
                        theme.colors.on_primary
                    } else {
                        theme.colors.on_surface
                    })
                    .flex()
                    .min_width(0.0)
            }),
        ))
        .style(move |s| {
            let theme_arc = get_theme();
            let theme = theme_arc.read().unwrap();
            s.items_center().gap(theme.spacing.sm).width_full()
        }),
    )
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        let mut style = s
            .width_full()
            .height(28.0)
            .padding_horiz(theme.spacing.sm)
            .padding_vert(theme.spacing.xs)
            .border_radius(theme.border_radius.sm)
            .cursor(floem::style::CursorStyle::Pointer);

        if selected {
            style = style.background(theme.colors.primary);
        } else if config.enable_hover {
            style = style.hover(move |s| s.background(theme.colors.hover));
        }

        style
    })
}

/// モダンなファイルアイコンを作成
fn create_modern_file_icon(entry: &FileEntry, size: f32) -> impl IntoView + use<> {
    let (icon_svg, icon_color) = get_file_icon_and_color(entry);

    container(
        svg(icon_svg.clone())
            .style(move |s| s.width(size * 0.7).height(size * 0.7).color(icon_color)),
    )
    .style(move |s| {
        let theme_arc = get_theme();
        let theme = theme_arc.read().unwrap();
        s.width(size)
            .height(size)
            .items_center()
            .justify_center()
            .border_radius(theme.border_radius.sm)
            .background(icon_color.multiply_alpha(0.1))
    })
}

/// ファイルタイプに基づいてアイコンと色を取得
fn get_file_icon_and_color(entry: &FileEntry) -> (String, Color) {
    match entry.file_type {
        FileType::Directory => (
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z"/>
            </svg>"#.to_string(),
            Color::rgb8(255, 193, 7), // Amber
        ),
        FileType::File => {
            let extension = entry.path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            match extension.as_str() {
                "rs" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(255, 87, 34), // Deep Orange
                ),
                "txt" | "md" | "doc" | "docx" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(33, 150, 243), // Blue
                ),
                "json" | "toml" | "yaml" | "yml" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M5,3H7V5H5V10A2,2 0 0,1 3,12A2,2 0 0,1 5,14V19H7V21H5C3.93,20.73 3,20.1 3,19V15A2,2 0 0,0 1,13H0V11H1A2,2 0 0,0 3,9V5C3,3.9 3.9,3 5,3M19,3A2,2 0 0,1 21,5V9A2,2 0 0,0 23,11H24V13H23A2,2 0 0,0 21,15V19A2,2 0 0,1 19,21H17V19H19V14A2,2 0 0,1 21,12A2,2 0 0,1 19,10V5H17V3H19Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(156, 39, 176), // Purple
                ),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M8.5,13.5L11,16.5L14.5,12L19,18H5M21,19V5C21,3.89 20.1,3 19,3H5A2,2 0 0,0 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(76, 175, 80), // Green
                ),
                "mp3" | "wav" | "flac" | "ogg" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12,3V12.26C11.5,12.09 11,12 10.5,12A2.5,2.5 0 0,0 8,14.5A2.5,2.5 0 0,0 10.5,17A2.5,2.5 0 0,0 13,14.5V7H16V5H12M10.5,15.5A1,1 0 0,1 9.5,14.5A1,1 0 0,1 10.5,13.5A1,1 0 0,1 11.5,14.5A1,1 0 0,1 10.5,15.5Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(233, 30, 99), // Pink
                ),
                "mp4" | "avi" | "mkv" | "mov" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M17,10.5V7A1,1 0 0,0 16,6H4A1,1 0 0,0 3,7V17A1,1 0 0,0 4,18H16A1,1 0 0,0 17,17V13.5L21,17.5V6.5L17,10.5Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(255, 152, 0), // Orange
                ),
                "zip" | "rar" | "7z" | "tar" | "gz" => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M14,17H12V15H14M14,13H12V11H14M12,9H14V7H12M12,19H14V21H10V19H12M14,3H12V5H14V3M14,1V3H16V5H18V9H16V7H14V9H12V7H10V9H8V5H10V3H12V1H14Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(158, 158, 158), // Gray
                ),
                _ => (
                    r#"<svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/>
                    </svg>"#.to_string(),
                    Color::rgb8(96, 125, 139), // Blue Gray
                ),
            }
        },
        FileType::SymLink => (
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M10.59,13.41C11,13.8 11,14.4 10.59,14.81C10.2,15.2 9.6,15.2 9.19,14.81L7.78,13.4L7.06,12.69L7.78,12L9.19,10.59C9.6,10.2 10.2,10.2 10.59,10.59C11,11 11,11.6 10.59,12L10.24,12.35L11.76,12.35L14.5,12.35C15.61,12.35 16.5,11.46 16.5,10.35C16.5,9.24 15.61,8.35 14.5,8.35L12.5,8.35V6.85L14.5,6.85C16.43,6.85 18,8.42 18,10.35C18,12.28 16.43,13.85 14.5,13.85L11.76,13.85L10.24,13.85L10.59,13.41M14.83,21.19L16.24,19.78L17.95,18.07L16.24,16.36L14.83,17.77L15.18,18.12L13.66,18.12L10.92,18.12C9.81,18.12 8.92,17.23 8.92,16.12C8.92,15.01 9.81,14.12 10.92,14.12L12.92,14.12V12.62L10.92,12.62C8.99,12.62 7.42,14.19 7.42,16.12C7.42,18.05 8.99,19.62 10.92,19.62L13.66,19.62L15.18,19.62L14.83,19.97Z"/>
            </svg>"#.to_string(),
            Color::rgb8(63, 81, 181), // Indigo
        ),
        FileType::Other => (
            r#"<svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M12,17A5,5 0 0,1 7,12A5,5 0 0,1 12,7A5,5 0 0,1 17,12A5,5 0 0,1 12,17Z"/>
            </svg>"#.to_string(),
            Color::rgb8(121, 85, 72), // Brown
        ),
    }
}

/// ファイルサイズをフォーマット
fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if size == 0 {
        return "—".to_string();
    }

    let mut size_f = size as f64;
    let mut unit_index = 0;

    while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
        size_f /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}

/// 更新日時をフォーマット
fn format_modified_time(modified: Option<SystemTime>) -> String {
    match modified {
        Some(time) => {
            match time.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let secs = duration.as_secs();
                    let now_secs = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let diff = now_secs.saturating_sub(secs);

                    if diff < 60 {
                        "今".to_string()
                    } else if diff < 3600 {
                        format!("{}分前", diff / 60)
                    } else if diff < 86400 {
                        format!("{}時間前", diff / 3600)
                    } else if diff < 604800 {
                        format!("{}日前", diff / 86400)
                    } else {
                        // 実際の日付を表示
                        format!(
                            "{}/{}/{}",
                            1970 + (secs / 31536000),
                            1 + ((secs % 31536000) / 2628000),
                            1 + ((secs % 2628000) / 86400)
                        )
                    }
                }
                Err(_) => "不明".to_string(),
            }
        }
        None => "—".to_string(),
    }
}

/// ダブルクリック可能なモダンファイルアイテムを作成
pub fn modern_file_item_with_double_click<F>(
    entry: FileEntry,
    selected: bool,
    config: ModernFileItemConfig,
    on_double_click: F,
) -> impl IntoView
where
    F: Fn(FileEntry) + 'static,
{
    let entry_for_click = entry.clone();

    modern_file_item_view(entry, selected, config).on_event_stop(
        floem::event::EventListener::DoubleClick,
        move |_event| {
            on_double_click(entry_for_click.clone());
        },
    )
}

/// デフォルト設定のモダンファイルアイテムを作成
pub fn default_modern_file_item(entry: FileEntry, selected: bool) -> impl IntoView {
    modern_file_item_view(entry, selected, ModernFileItemConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_modern_file_item_config() {
        let config = ModernFileItemConfig::default();
        assert_eq!(config.display_mode, FileItemDisplayMode::List);
        assert_eq!(config.icon_size, 24.0);
        assert!(config.show_selection);
        assert!(config.show_details);
        assert!(config.enable_hover);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "—");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
    }

    #[test]
    fn test_file_icon_and_color() {
        let rust_file = FileEntry {
            name: "main.rs".to_string(),
            path: PathBuf::from("main.rs"),
            file_type: FileType::File,
            size: 1024,
            modified: None,
        };

        let (icon, color) = get_file_icon_and_color(&rust_file);
        assert!(icon.contains("svg"));
        assert_eq!(color, Color::rgb8(255, 87, 34));
    }

    #[test]
    fn test_directory_icon() {
        let directory = FileEntry {
            name: "src".to_string(),
            path: PathBuf::from("src"),
            file_type: FileType::Directory,
            size: 0,
            modified: None,
        };

        let (icon, color) = get_file_icon_and_color(&directory);
        assert!(icon.contains("svg"));
        assert_eq!(color, Color::rgb8(255, 193, 7));
    }
}
