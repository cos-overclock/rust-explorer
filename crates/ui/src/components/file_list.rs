//! ファイル一覧表示コンポーネント

use floem::{
    View,
    reactive::{RwSignal, SignalGet, SignalUpdate},
    views::{Decorators, dyn_stack, scroll},
};
use rust_explorer_core::FileEntry;
use std::path::PathBuf;

use super::file_item::file_item_component;

/// ファイルリストの表示設定
#[derive(Debug, Clone)]
pub struct FileListConfig {
    /// 隠しファイルを表示するか
    pub show_hidden: bool,
    /// 選択可能な最大アイテム数
    pub max_selection: usize,
    /// リストの高さ（None の場合は自動）
    pub height: Option<f64>,
}

impl Default for FileListConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            max_selection: 100,
            height: None,
        }
    }
}

/// ファイルリストの状態
#[derive(Debug, Clone)]
pub struct FileListState {
    /// 現在表示しているディレクトリ
    pub current_path: PathBuf,
    /// ファイル一覧
    pub entries: Vec<FileEntry>,
    /// 選択されたアイテムのインデックス
    pub selected_indices: Vec<usize>,
    /// 読み込み中フラグ
    pub loading: bool,
    /// エラーメッセージ
    pub error: Option<String>,
}

impl Default for FileListState {
    fn default() -> Self {
        Self {
            current_path: PathBuf::from("."),
            entries: Vec::new(),
            selected_indices: Vec::new(),
            loading: false,
            error: None,
        }
    }
}

/// ファイルリストビューを作成
pub fn file_list_view(
    entries: RwSignal<Vec<FileEntry>>,
    selected_indices: RwSignal<Vec<usize>>,
) -> impl View {
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
            file_item_component(entry, is_selected)
        },
    ))
    .style(|s| s.flex_col().gap(1))
}

/// ファイルリストビューコンポーネント
pub struct FileListView {
    entries: RwSignal<Vec<FileEntry>>,
    selected_indices: RwSignal<Vec<usize>>,
    current_path: RwSignal<PathBuf>,
    config: FileListConfig,
}

impl FileListView {
    /// 新しいファイルリストビューを作成
    pub fn new(initial_path: PathBuf, config: FileListConfig) -> Self {
        let view = Self {
            entries: RwSignal::new(Vec::new()),
            selected_indices: RwSignal::new(Vec::new()),
            current_path: RwSignal::new(initial_path.clone()),
            config,
        };

        // 初期ディレクトリを読み込み
        view.load_directory_sync(initial_path);

        view
    }

    /// デフォルト設定でファイルリストビューを作成
    pub fn with_default(initial_path: PathBuf) -> Self {
        Self::new(initial_path, FileListConfig::default())
    }

    /// 現在のパスを取得
    pub fn current_path(&self) -> PathBuf {
        self.current_path.get()
    }

    /// 選択されたファイルエントリを取得
    pub fn selected_entries(&self) -> Vec<FileEntry> {
        let entries = self.entries.get();
        let indices = self.selected_indices.get();

        indices
            .iter()
            .filter_map(|&index| entries.get(index))
            .cloned()
            .collect()
    }

    /// ディレクトリを変更
    pub fn change_directory(&self, path: PathBuf) {
        self.current_path.set(path.clone());
        self.selected_indices.set(Vec::new());
        self.load_directory_sync(path);
    }

    /// 選択をクリア
    pub fn clear_selection(&self) {
        self.selected_indices.set(Vec::new());
    }

    /// アイテムを選択
    pub fn select_item(&self, index: usize) {
        let entries_len = self.entries.get().len();
        if index < entries_len {
            self.selected_indices.set(vec![index]);
        }
    }

    /// 複数アイテムを選択に追加
    pub fn add_to_selection(&self, index: usize) {
        let entries_len = self.entries.get().len();
        let mut indices = self.selected_indices.get();

        if index < entries_len
            && !indices.contains(&index)
            && indices.len() < self.config.max_selection
        {
            indices.push(index);
            self.selected_indices.set(indices);
        }
    }

    /// 表示を更新
    pub fn refresh(&self) {
        let current_path = self.current_path.get();
        self.load_directory_sync(current_path);
    }

    /// ファイルリストビューを作成
    pub fn view(&self) -> impl View {
        file_list_view(self.entries, self.selected_indices)
    }

    /// ディレクトリを同期的に読み込み
    fn load_directory_sync(&self, path: PathBuf) {
        use std::fs;

        let entries = match fs::read_dir(&path) {
            Ok(entries) => {
                let mut file_entries = Vec::new();

                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    let metadata = entry.metadata().ok();

                    let file_type = if entry_path.is_dir() {
                        rust_explorer_core::FileType::Directory
                    } else if entry_path.is_symlink() {
                        rust_explorer_core::FileType::SymLink
                    } else if entry_path.is_file() {
                        rust_explorer_core::FileType::File
                    } else {
                        rust_explorer_core::FileType::Other
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

                // 隠しファイルのフィルタリング
                if !self.config.show_hidden {
                    file_entries.retain(|entry| !entry.name.starts_with('.'));
                }

                // 名前順でソート（フォルダを先に）
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
            Err(e) => {
                eprintln!("ディレクトリの読み込みに失敗しました: {}", e);
                Vec::new()
            }
        };

        self.entries.set(entries);
    }
}

/// ファイルリストビューコンポーネントを作成（便利関数）
pub fn file_list_view_component(initial_path: PathBuf, config: FileListConfig) -> FileListView {
    FileListView::new(initial_path, config)
}

/// デフォルト設定でファイルリストビューを作成（便利関数）
pub fn default_file_list_view(initial_path: PathBuf) -> FileListView {
    FileListView::with_default(initial_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_explorer_core::{FileEntry, FileType};
    use std::time::SystemTime;

    #[test]
    fn test_file_list_view_creation() {
        let view = FileListView::new(PathBuf::from("/test"), FileListConfig::default());
        assert_eq!(view.current_path(), PathBuf::from("/test"));
    }

    #[test]
    fn test_file_list_config() {
        let config = FileListConfig::default();
        assert!(!config.show_hidden);
        assert_eq!(config.max_selection, 100);
        assert!(config.height.is_none());
    }

    #[test]
    fn test_selection_operations() {
        let view = FileListView::new(PathBuf::from("/test"), FileListConfig::default());

        // 初期状態では何も選択されていない
        assert!(view.selected_entries().is_empty());

        // 選択をクリア
        view.clear_selection();
        assert!(view.selected_entries().is_empty());
    }
}
