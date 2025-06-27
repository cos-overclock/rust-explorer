//! メインコンテンツ領域コンポーネント
//!
//! アプリケーションのメインコンテンツ部分を提供します。

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::text::Weight;
use rust_explorer_config::Settings;
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;

use super::{breadcrumb_view, file_item_with_double_click, navigation_helpers};

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
            content_type: ContentType::FileExplorer,
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

/// ファイルエクスプローラーコンテンツの作成
fn create_file_explorer_content(_settings: Rc<RefCell<Settings>>) -> impl IntoView {
    use floem::reactive::RwSignal;
    use rust_explorer_core::FileNavigationManager;
    use std::sync::Arc;

    // 現在のディレクトリを取得（フォールバックは/home）
    let current_dir = env::current_dir().unwrap_or_else(|_| {
        env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    });

    // リアクティブな現在のパス
    let current_path = RwSignal::new(current_dir.clone());
    let current_path_for_nav = current_path;

    // ファイルナビゲーションマネージャーを作成
    let _nav_manager = Arc::new(FileNavigationManager::with_default());

    // UI用のナビゲーションマネージャーを作成
    let ui_nav_manager = Arc::new(
        super::FileNavigationManager::with_default(current_dir.clone())
            .on_path_change(move |new_path| {
                current_path_for_nav.set(new_path);
            })
            .on_error(move |error| {
                eprintln!("ナビゲーションエラー: {}", error);
            }),
    );

    let ui_nav_for_list = ui_nav_manager.clone();

    v_stack((
        // ナビゲーションツールバー
        navigation_helpers::navigation_toolbar(ui_nav_manager.clone())
            .style(|s| s.margin_bottom(10.0)),
        // パンくずナビゲーション
        breadcrumb_view(current_path).style(|s| s.margin_bottom(10.0)),
        // ファイルリストエリア
        create_file_list_container_with_navigation(current_dir, ui_nav_for_list),
    ))
    .style(|s| s.size_full().gap(5.0))
}

/// 同期的にディレクトリを読み込み
fn load_directory_sync(path: &PathBuf) -> Vec<rust_explorer_core::FileEntry> {
    use rust_explorer_core::{FileEntry, FileType};
    use std::fs;

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut file_entries = Vec::new();

            for entry in entries.flatten() {
                let entry_path = entry.path();
                let metadata = entry.metadata().ok();

                let file_type = if entry_path.is_dir() {
                    FileType::Directory
                } else if entry_path.is_symlink() {
                    FileType::SymLink
                } else if entry_path.is_file() {
                    FileType::File
                } else {
                    FileType::Other
                };

                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = metadata.and_then(|m| m.modified().ok());

                let file_entry = FileEntry {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry_path,
                    file_type,
                    size,
                    modified,
                };

                file_entries.push(file_entry);
            }

            // フォルダを先にソート
            file_entries.sort_by(|a, b| {
                use rust_explorer_core::FileType;
                match (&a.file_type, &b.file_type) {
                    (FileType::Directory, FileType::Directory)
                    | (FileType::File, FileType::File) => a.name.cmp(&b.name),
                    (FileType::Directory, _) => std::cmp::Ordering::Less,
                    (_, FileType::Directory) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });

            file_entries
        }
        Err(_) => Vec::new(),
    }
}

/// ナビゲーション付きファイルリストコンテナの作成
fn create_file_list_container_with_navigation(
    current_dir: PathBuf,
    nav_manager: std::sync::Arc<super::FileNavigationManager>,
) -> impl IntoView {
    use floem::reactive::RwSignal;
    use floem::views::{Decorators, dyn_stack, scroll};
    use rust_explorer_core::FileEntry;

    let entries = RwSignal::new(Vec::<FileEntry>::new());
    let selected_indices = RwSignal::new(Vec::<usize>::new());

    // 初期ファイル読み込み
    let file_entries = load_directory_sync(&current_dir);
    entries.set(file_entries);

    container(
        scroll(dyn_stack(
            move || entries.get(),
            |entry| entry.name.clone(),
            move |entry| {
                let index = entries
                    .get()
                    .iter()
                    .position(|e| e.name == entry.name)
                    .unwrap_or(0);
                let is_selected = selected_indices.get().contains(&index);

                // ダブルクリックハンドラー付きのファイルアイテムを作成
                let nav_manager_clone = nav_manager.clone();
                file_item_with_double_click(entry, is_selected, move |entry| {
                    nav_manager_clone.handle_double_click(&entry);
                })
            },
        ))
        .style(|s| s.flex_col().gap(1)),
    )
    .style(|s| {
        s.size_full()
            .border(1.0)
            .border_color(Color::rgb8(200, 200, 200))
            .border_radius(8.0)
            .background(Color::rgb8(255, 255, 255))
    })
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
        matches!(config.content_type, ContentType::FileExplorer);
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
