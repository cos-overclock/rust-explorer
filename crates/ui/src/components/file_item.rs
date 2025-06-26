//! ファイル/フォルダ表示項目コンポーネント

use floem::peniko::Color;
use floem::{
    View,
    views::{Decorators, h_stack, label, text, v_stack},
};
use rust_explorer_core::{FileEntry, FileType};
use std::time::SystemTime;

/// ファイルアイテムのビューを作成
pub fn file_item_view(entry: FileEntry, selected: bool) -> impl View {
    let entry_for_icon = entry.clone();
    let entry_for_name = entry.clone();
    let entry_for_size = entry.clone();
    let entry_for_time = entry;

    h_stack((
        // ファイル/フォルダアイコン
        file_icon_view(entry_for_icon),
        // ファイル情報
        v_stack((
            // ファイル名
            label(move || entry_for_name.name.clone())
                .style(|s| s.font_weight(floem::text::Weight::BOLD)),
            // ファイルサイズと更新日時
            h_stack((
                label(move || format_file_size(entry_for_size.size)),
                label(move || format_modified_time(entry_for_time.modified)),
            ))
            .style(|s| s.gap(10)),
        ))
        .style(|s| s.flex_col().flex()),
    ))
    .style(move |s| {
        let mut style = s
            .padding(8)
            .border_radius(4)
            .cursor(floem::style::CursorStyle::Pointer);

        if selected {
            style = style.background(Color::rgb8(66, 135, 245).multiply_alpha(0.2));
        }

        style
    })
}

/// ファイル/フォルダのアイコンビューを作成
fn file_icon_view(entry: FileEntry) -> impl View {
    let icon_text = match entry.file_type {
        FileType::Directory => "📁",
        FileType::File => {
            // 拡張子に基づいてアイコンを選択
            match entry.path.extension().and_then(|s| s.to_str()) {
                Some("txt") | Some("md") => "📄",
                Some("rs") => "⚙️",
                Some("json") | Some("toml") | Some("yaml") | Some("yml") => "⚙️",
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("bmp") => "🖼️",
                Some("mp3") | Some("wav") | Some("flac") => "🎵",
                Some("mp4") | Some("avi") | Some("mkv") => "🎬",
                Some("zip") | Some("rar") | Some("7z") | Some("tar") => "📦",
                _ => "📄",
            }
        }
        FileType::SymLink => "🔗",
        FileType::Other => "❓",
    };

    text(icon_text).style(|s| s.font_size(20).padding(4))
}

/// ファイルサイズをフォーマット
fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if size == 0 {
        return "0 B".to_string();
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
                        let datetime = chrono::DateTime::from_timestamp(secs as i64, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        datetime.format("%Y/%m/%d").to_string()
                    }
                }
                Err(_) => "不明".to_string(),
            }
        }
        None => "-".to_string(),
    }
}

/// ファイルアイテムコンポーネントを作成（便利関数）
pub fn file_item_component(entry: FileEntry, selected: bool) -> impl View {
    file_item_view(entry, selected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_format_modified_time() {
        let now = SystemTime::now();

        // 現在時刻
        let formatted = format_modified_time(Some(now));
        assert_eq!(formatted, "今");

        // None の場合
        let formatted = format_modified_time(None);
        assert_eq!(formatted, "-");
    }

    #[test]
    fn test_file_item_view_creation() {
        let entry = FileEntry {
            name: "test.txt".to_string(),
            path: PathBuf::from("/test/test.txt"),
            file_type: FileType::File,
            size: 1024,
            modified: Some(SystemTime::now()),
        };

        let _view = file_item_view(entry, false);
        // コンパイルできることをテスト
    }
}
