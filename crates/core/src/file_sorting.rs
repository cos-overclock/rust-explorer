//! ファイル・フォルダソート機能
//!
//! ファイルエントリのソートとフィルタ機能を提供します。

use crate::{FileEntry, FileType};
use std::cmp::Ordering;
use std::path::Path;

/// ソート条件
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCriteria {
    /// 名前順
    Name,
    /// サイズ順
    Size,
    /// 更新日時順
    Modified,
    /// 種類順（拡張子）
    Type,
}

impl Default for SortCriteria {
    fn default() -> Self {
        Self::Name
    }
}

/// ソート方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// 昇順
    Ascending,
    /// 降順
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Ascending
    }
}

/// ソート設定
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortConfig {
    /// ソート条件
    pub criteria: SortCriteria,
    /// ソート方向
    pub direction: SortDirection,
    /// フォルダを最初にソートするか
    pub folders_first: bool,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            criteria: SortCriteria::default(),
            direction: SortDirection::default(),
            folders_first: true,
        }
    }
}

/// フィルタ条件
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FilterCriteria {
    /// 隠しファイル・フォルダを表示するか
    pub show_hidden: bool,
    /// ファイル名検索文字列（部分一致）
    pub name_filter: Option<String>,
    /// 拡張子フィルタ
    pub extension_filter: Option<String>,
    /// 最小サイズ（バイト）
    pub min_size: Option<u64>,
    /// 最大サイズ（バイト）
    pub max_size: Option<u64>,
}

/// ファイルソート・フィルタマネージャー
pub struct FileSortFilterManager {
    sort_config: SortConfig,
    filter_criteria: FilterCriteria,
}

impl FileSortFilterManager {
    /// 新しいマネージャーを作成
    pub fn new() -> Self {
        Self {
            sort_config: SortConfig::default(),
            filter_criteria: FilterCriteria::default(),
        }
    }

    /// ソート設定を更新
    pub fn update_sort_config(&mut self, config: SortConfig) {
        self.sort_config = config;
    }

    /// フィルタ条件を更新
    pub fn update_filter_criteria(&mut self, criteria: FilterCriteria) {
        self.filter_criteria = criteria;
    }

    /// 現在のソート設定を取得
    pub fn sort_config(&self) -> SortConfig {
        self.sort_config
    }

    /// 現在のフィルタ条件を取得
    pub fn filter_criteria(&self) -> &FilterCriteria {
        &self.filter_criteria
    }

    /// ファイルエントリをソートしてフィルタする
    pub fn process_entries(&self, entries: &mut Vec<FileEntry>) {
        // フィルタを適用
        self.apply_filter(entries);

        // ソートを適用
        self.apply_sort(entries);
    }

    /// フィルタを適用
    fn apply_filter(&self, entries: &mut Vec<FileEntry>) {
        entries.retain(|entry| self.passes_filter(entry));
    }

    /// エントリがフィルタ条件を満たすかチェック
    fn passes_filter(&self, entry: &FileEntry) -> bool {
        // 隠しファイル・フォルダのチェック
        if !self.filter_criteria.show_hidden && is_hidden_file(&entry.name) {
            return false;
        }

        // ファイル名フィルタ
        if let Some(ref name_filter) = self.filter_criteria.name_filter {
            if !name_filter.is_empty()
                && !entry
                    .name
                    .to_lowercase()
                    .contains(&name_filter.to_lowercase())
            {
                return false;
            }
        }

        // 拡張子フィルタ
        if let Some(ref ext_filter) = self.filter_criteria.extension_filter {
            if !ext_filter.is_empty() {
                if let Some(ext) = entry.path.extension().and_then(|s| s.to_str()) {
                    if !ext.to_lowercase().eq(&ext_filter.to_lowercase()) {
                        return false;
                    }
                } else {
                    return false; // 拡張子がない場合は除外
                }
            }
        }

        // サイズフィルタ（ファイルのみ）
        if entry.file_type == FileType::File {
            if let Some(min_size) = self.filter_criteria.min_size {
                if entry.size < min_size {
                    return false;
                }
            }

            if let Some(max_size) = self.filter_criteria.max_size {
                if entry.size > max_size {
                    return false;
                }
            }
        }

        true
    }

    /// ソートを適用
    fn apply_sort(&self, entries: &mut [FileEntry]) {
        entries.sort_by(|a, b| self.compare_entries(a, b));
    }

    /// エントリを比較
    fn compare_entries(&self, a: &FileEntry, b: &FileEntry) -> Ordering {
        // フォルダ優先ソート
        if self.sort_config.folders_first {
            match (&a.file_type, &b.file_type) {
                (FileType::Directory, FileType::Directory) => {
                    // 両方フォルダの場合は通常比較
                }
                (FileType::Directory, _) => return Ordering::Less,
                (_, FileType::Directory) => return Ordering::Greater,
                _ => {
                    // 両方ファイルの場合は通常比較
                }
            }
        }

        // ソート条件に基づく比較
        let result = match self.sort_config.criteria {
            SortCriteria::Name => compare_names(&a.name, &b.name),
            SortCriteria::Size => compare_sizes(a.size, b.size),
            SortCriteria::Modified => compare_modified_times(a.modified, b.modified),
            SortCriteria::Type => compare_types(&a.path, &b.path),
        };

        // ソート方向を適用
        match self.sort_config.direction {
            SortDirection::Ascending => result,
            SortDirection::Descending => result.reverse(),
        }
    }

    /// ソート方向を切り替え
    pub fn toggle_sort_direction(&mut self) {
        self.sort_config.direction = match self.sort_config.direction {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        };
    }

    /// ソート条件を変更（同じ条件の場合は方向を切り替え）
    pub fn set_sort_criteria(&mut self, criteria: SortCriteria) {
        if self.sort_config.criteria == criteria {
            self.toggle_sort_direction();
        } else {
            self.sort_config.criteria = criteria;
            self.sort_config.direction = SortDirection::Ascending;
        }
    }

    /// フィルタをクリア
    pub fn clear_filters(&mut self) {
        self.filter_criteria = FilterCriteria::default();
    }

    /// フィルタが適用されているかチェック
    pub fn has_active_filters(&self) -> bool {
        self.filter_criteria.name_filter.is_some()
            || self.filter_criteria.extension_filter.is_some()
            || self.filter_criteria.min_size.is_some()
            || self.filter_criteria.max_size.is_some()
            || self.filter_criteria.show_hidden
    }
}

impl Default for FileSortFilterManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 隠しファイル・フォルダかどうかをチェック
fn is_hidden_file(name: &str) -> bool {
    name.starts_with('.')
}

/// ファイル名を比較（大文字小文字を区別しない自然順序）
fn compare_names(a: &str, b: &str) -> Ordering {
    // 大文字小文字を区別しない比較
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    // 自然順序ソート（数字部分を数値として比較）
    natural_sort_compare(&a_lower, &b_lower)
}

/// サイズを比較
fn compare_sizes(a: u64, b: u64) -> Ordering {
    a.cmp(&b)
}

/// 更新日時を比較
fn compare_modified_times(
    a: Option<std::time::SystemTime>,
    b: Option<std::time::SystemTime>,
) -> Ordering {
    match (a, b) {
        (Some(a_time), Some(b_time)) => a_time.cmp(&b_time),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

/// ファイルタイプ（拡張子）を比較
fn compare_types(a: &Path, b: &Path) -> Ordering {
    let a_ext = a.extension().and_then(|s| s.to_str()).unwrap_or("");
    let b_ext = b.extension().and_then(|s| s.to_str()).unwrap_or("");

    a_ext.to_lowercase().cmp(&b_ext.to_lowercase())
}

/// 自然順序ソート（簡易版）
fn natural_sort_compare(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    loop {
        match (a_chars.peek(), b_chars.peek()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(a_char), Some(b_char)) => {
                if a_char.is_ascii_digit() && b_char.is_ascii_digit() {
                    // 数字部分の比較
                    let a_num = extract_number(&mut a_chars);
                    let b_num = extract_number(&mut b_chars);
                    match a_num.cmp(&b_num) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    // 通常文字の比較
                    let a_next = a_chars.next().unwrap();
                    let b_next = b_chars.next().unwrap();
                    match a_next.cmp(&b_next) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
            }
        }
    }
}

/// 数字を抽出
fn extract_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> u64 {
    let mut num_str = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    num_str.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_test_entry(
        name: &str,
        file_type: FileType,
        size: u64,
        modified: Option<SystemTime>,
    ) -> FileEntry {
        FileEntry {
            name: name.to_string(),
            path: PathBuf::from(name),
            file_type,
            size,
            modified,
        }
    }

    #[test]
    fn test_sort_config_default() {
        let config = SortConfig::default();
        assert_eq!(config.criteria, SortCriteria::Name);
        assert_eq!(config.direction, SortDirection::Ascending);
        assert!(config.folders_first);
    }

    #[test]
    fn test_filter_criteria_default() {
        let criteria = FilterCriteria::default();
        assert!(!criteria.show_hidden);
        assert!(criteria.name_filter.is_none());
        assert!(criteria.extension_filter.is_none());
        assert!(criteria.min_size.is_none());
        assert!(criteria.max_size.is_none());
    }

    #[test]
    fn test_sort_by_name() {
        let mut manager = FileSortFilterManager::new();
        manager.update_sort_config(SortConfig {
            criteria: SortCriteria::Name,
            direction: SortDirection::Ascending,
            folders_first: false,
        });

        let mut entries = vec![
            create_test_entry("zebra.txt", FileType::File, 100, None),
            create_test_entry("apple.txt", FileType::File, 200, None),
            create_test_entry("banana.txt", FileType::File, 150, None),
        ];

        manager.process_entries(&mut entries);

        assert_eq!(entries[0].name, "apple.txt");
        assert_eq!(entries[1].name, "banana.txt");
        assert_eq!(entries[2].name, "zebra.txt");
    }

    #[test]
    fn test_sort_by_size() {
        let mut manager = FileSortFilterManager::new();
        manager.update_sort_config(SortConfig {
            criteria: SortCriteria::Size,
            direction: SortDirection::Ascending,
            folders_first: false,
        });

        let mut entries = vec![
            create_test_entry("large.txt", FileType::File, 1000, None),
            create_test_entry("small.txt", FileType::File, 100, None),
            create_test_entry("medium.txt", FileType::File, 500, None),
        ];

        manager.process_entries(&mut entries);

        assert_eq!(entries[0].name, "small.txt");
        assert_eq!(entries[1].name, "medium.txt");
        assert_eq!(entries[2].name, "large.txt");
    }

    #[test]
    fn test_folders_first() {
        let mut manager = FileSortFilterManager::new();
        manager.update_sort_config(SortConfig {
            criteria: SortCriteria::Name,
            direction: SortDirection::Ascending,
            folders_first: true,
        });

        let mut entries = vec![
            create_test_entry("file.txt", FileType::File, 100, None),
            create_test_entry("folder", FileType::Directory, 0, None),
            create_test_entry("another_file.txt", FileType::File, 200, None),
        ];

        manager.process_entries(&mut entries);

        // フォルダが最初に来る
        assert_eq!(entries[0].name, "folder");
        assert_eq!(entries[0].file_type, FileType::Directory);
    }

    #[test]
    fn test_filter_hidden_files() {
        let mut manager = FileSortFilterManager::new();
        manager.update_filter_criteria(FilterCriteria {
            show_hidden: false,
            ..Default::default()
        });

        let mut entries = vec![
            create_test_entry("visible.txt", FileType::File, 100, None),
            create_test_entry(".hidden.txt", FileType::File, 200, None),
            create_test_entry("normal.txt", FileType::File, 150, None),
        ];

        manager.process_entries(&mut entries);

        // 隠しファイルが除外される
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| !e.name.starts_with('.')));
    }

    #[test]
    fn test_filter_by_name() {
        let mut manager = FileSortFilterManager::new();
        manager.update_filter_criteria(FilterCriteria {
            name_filter: Some("test".to_string()),
            ..Default::default()
        });

        let mut entries = vec![
            create_test_entry("test_file.txt", FileType::File, 100, None),
            create_test_entry("other.txt", FileType::File, 200, None),
            create_test_entry("testing.txt", FileType::File, 150, None),
        ];

        manager.process_entries(&mut entries);

        // "test"を含むファイルのみ残る
        assert_eq!(entries.len(), 2);
        assert!(
            entries
                .iter()
                .all(|e| e.name.to_lowercase().contains("test"))
        );
    }

    #[test]
    fn test_filter_by_extension() {
        let mut manager = FileSortFilterManager::new();
        manager.update_filter_criteria(FilterCriteria {
            extension_filter: Some("txt".to_string()),
            ..Default::default()
        });

        let mut entries = vec![
            create_test_entry("document.txt", FileType::File, 100, None),
            create_test_entry("image.png", FileType::File, 200, None),
            create_test_entry("note.txt", FileType::File, 150, None),
        ];

        manager.process_entries(&mut entries);

        // .txtファイルのみ残る
        assert_eq!(entries.len(), 2);
        assert!(
            entries
                .iter()
                .all(|e| e.path.extension().and_then(|s| s.to_str()) == Some("txt"))
        );
    }

    #[test]
    fn test_filter_by_size() {
        let mut manager = FileSortFilterManager::new();
        manager.update_filter_criteria(FilterCriteria {
            min_size: Some(150),
            max_size: Some(250),
            ..Default::default()
        });

        let mut entries = vec![
            create_test_entry("small.txt", FileType::File, 100, None),
            create_test_entry("medium.txt", FileType::File, 200, None),
            create_test_entry("large.txt", FileType::File, 300, None),
        ];

        manager.process_entries(&mut entries);

        // 150-250バイトのファイルのみ残る
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "medium.txt");
    }

    #[test]
    fn test_toggle_sort_direction() {
        let mut manager = FileSortFilterManager::new();

        // 初期状態は昇順
        assert_eq!(manager.sort_config().direction, SortDirection::Ascending);

        manager.toggle_sort_direction();
        assert_eq!(manager.sort_config().direction, SortDirection::Descending);

        manager.toggle_sort_direction();
        assert_eq!(manager.sort_config().direction, SortDirection::Ascending);
    }

    #[test]
    fn test_set_sort_criteria() {
        let mut manager = FileSortFilterManager::new();

        // 初期状態はName, Ascending
        assert_eq!(manager.sort_config().criteria, SortCriteria::Name);
        assert_eq!(manager.sort_config().direction, SortDirection::Ascending);

        // 異なる条件を設定すると昇順にリセット
        manager.set_sort_criteria(SortCriteria::Size);
        assert_eq!(manager.sort_config().criteria, SortCriteria::Size);
        assert_eq!(manager.sort_config().direction, SortDirection::Ascending);

        // 同じ条件を設定すると方向を切り替え
        manager.set_sort_criteria(SortCriteria::Size);
        assert_eq!(manager.sort_config().criteria, SortCriteria::Size);
        assert_eq!(manager.sort_config().direction, SortDirection::Descending);
    }

    #[test]
    fn test_has_active_filters() {
        let mut manager = FileSortFilterManager::new();

        // 初期状態ではフィルタなし
        assert!(!manager.has_active_filters());

        // 隠しファイル表示を有効にしてもフィルタありとみなさない
        manager.update_filter_criteria(FilterCriteria {
            show_hidden: true,
            ..Default::default()
        });
        assert!(!manager.has_active_filters());

        // 名前フィルタを設定
        manager.update_filter_criteria(FilterCriteria {
            name_filter: Some("test".to_string()),
            ..Default::default()
        });
        assert!(manager.has_active_filters());
    }

    #[test]
    fn test_clear_filters() {
        let mut manager = FileSortFilterManager::new();

        // フィルタを設定
        manager.update_filter_criteria(FilterCriteria {
            name_filter: Some("test".to_string()),
            extension_filter: Some("txt".to_string()),
            min_size: Some(100),
            max_size: Some(1000),
            show_hidden: true,
        });

        assert!(manager.has_active_filters());

        // フィルタをクリア
        manager.clear_filters();
        assert!(!manager.has_active_filters());
        assert_eq!(manager.filter_criteria(), &FilterCriteria::default());
    }

    #[test]
    fn test_natural_sort() {
        let mut manager = FileSortFilterManager::new();
        manager.update_sort_config(SortConfig {
            criteria: SortCriteria::Name,
            direction: SortDirection::Ascending,
            folders_first: false,
        });

        let mut entries = vec![
            create_test_entry("file10.txt", FileType::File, 100, None),
            create_test_entry("file2.txt", FileType::File, 200, None),
            create_test_entry("file1.txt", FileType::File, 150, None),
        ];

        manager.process_entries(&mut entries);

        // 自然順序ソート: file1, file2, file10
        assert_eq!(entries[0].name, "file1.txt");
        assert_eq!(entries[1].name, "file2.txt");
        assert_eq!(entries[2].name, "file10.txt");
    }

    #[test]
    fn test_is_hidden_file() {
        assert!(is_hidden_file(".hidden"));
        assert!(is_hidden_file(".config"));
        assert!(!is_hidden_file("visible.txt"));
        assert!(!is_hidden_file("normal"));
    }
}
