//! ファイル・フォルダナビゲーション機能

use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::Decorators;
use floem::{IntoView, View};
use rust_explorer_core::{FileEntry, FileNavigationManager as CoreFileNavigationManager, FileType};
use rust_explorer_utils::AppError;
use std::path::PathBuf;
use std::sync::Arc;

/// ファイルナビゲーションの設定
#[derive(Debug, Clone)]
pub struct FileNavigationConfig {
    /// ダブルクリックでファイル/フォルダを開くかどうか
    pub enable_double_click: bool,
    /// 上位階層への移動を有効にするかどうか
    pub enable_parent_navigation: bool,
    /// ナビゲーション履歴を有効にするかどうか
    pub enable_history: bool,
}

impl Default for FileNavigationConfig {
    fn default() -> Self {
        Self {
            enable_double_click: true,
            enable_parent_navigation: true,
            enable_history: true,
        }
    }
}

/// ファイルナビゲーションの状態
#[derive(Debug, Clone)]
pub struct FileNavigationState {
    /// 現在のディレクトリパス
    pub current_path: PathBuf,
    /// ナビゲーション履歴（戻る用）
    pub history_back: Vec<PathBuf>,
    /// ナビゲーション履歴（進む用）
    pub history_forward: Vec<PathBuf>,
    /// 最後に発生したエラー
    pub last_error: Option<String>,
}

impl FileNavigationState {
    pub fn new(initial_path: PathBuf) -> Self {
        Self {
            current_path: initial_path,
            history_back: Vec::new(),
            history_forward: Vec::new(),
            last_error: None,
        }
    }
}

/// ファイルナビゲーション管理
pub struct FileNavigationManager {
    /// ナビゲーション状態
    state: RwSignal<FileNavigationState>,
    /// ナビゲーション設定
    config: FileNavigationConfig,
    /// ファイルナビゲーション機能
    navigation_manager: Arc<CoreFileNavigationManager>,
    /// パス変更通知コールバック
    on_path_change: Option<Box<dyn Fn(PathBuf) + Send + Sync>>,
    /// エラー通知コールバック
    on_error: Option<Box<dyn Fn(String) + Send + Sync>>,
}

impl FileNavigationManager {
    /// 新しいファイルナビゲーション管理を作成
    pub fn new(
        initial_path: PathBuf,
        config: FileNavigationConfig,
        navigation_manager: Arc<CoreFileNavigationManager>,
    ) -> Self {
        Self {
            state: RwSignal::new(FileNavigationState::new(initial_path)),
            config,
            navigation_manager,
            on_path_change: None,
            on_error: None,
        }
    }

    /// デフォルト設定でファイルナビゲーション管理を作成
    pub fn with_default(initial_path: PathBuf) -> Self {
        let navigation_manager = Arc::new(CoreFileNavigationManager::with_default());
        Self::new(
            initial_path,
            FileNavigationConfig::default(),
            navigation_manager,
        )
    }

    /// パス変更通知コールバックを設定
    pub fn on_path_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(PathBuf) + Send + Sync + 'static,
    {
        self.on_path_change = Some(Box::new(callback));
        self
    }

    /// エラー通知コールバックを設定
    pub fn on_error<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_error = Some(Box::new(callback));
        self
    }

    /// 現在のパスを取得
    pub fn current_path(&self) -> PathBuf {
        self.state.get().current_path
    }

    /// ファイルエントリをダブルクリック処理
    pub fn handle_double_click(&self, entry: &FileEntry) {
        if !self.config.enable_double_click {
            return;
        }

        match entry.file_type {
            FileType::Directory => {
                // ディレクトリの場合は移動
                if let Err(e) = self.navigate_to(&entry.path) {
                    self.handle_error(format!("ディレクトリ移動エラー: {}", e));
                }
            }
            FileType::File => {
                // ファイルの場合は開く
                if let Err(e) = self.navigation_manager.open_item(&entry.path) {
                    self.handle_error(format!("ファイルオープンエラー: {}", e));
                }
            }
            _ => {
                // その他のタイプは既定の処理
                if let Err(e) = self.navigation_manager.open_item(&entry.path) {
                    self.handle_error(format!("アイテムオープンエラー: {}", e));
                }
            }
        }
    }

    /// 指定されたディレクトリに移動
    pub fn navigate_to(&self, path: &PathBuf) -> Result<(), AppError> {
        // 移動可能性をチェック
        self.navigation_manager.validate_navigation(path)?;

        let current_path = self.current_path();

        // 履歴に現在のパスを追加
        if self.config.enable_history && current_path != *path {
            self.state.update(|state| {
                state.history_back.push(current_path);
                // 進む履歴をクリア
                state.history_forward.clear();
                // 履歴の上限を制限（例：50エントリ）
                if state.history_back.len() > 50 {
                    state.history_back.remove(0);
                }
            });
        }

        // パスを更新
        self.state.update(|state| {
            state.current_path = path.clone();
            state.last_error = None;
        });

        // コールバック呼び出し
        if let Some(callback) = &self.on_path_change {
            callback(path.clone());
        }

        Ok(())
    }

    /// 上位ディレクトリに移動
    pub fn navigate_up(&self) -> Result<(), AppError> {
        if !self.config.enable_parent_navigation {
            return Err(AppError::FileSystemCustom(
                "親ディレクトリナビゲーションが無効です".to_string(),
            ));
        }

        let current_path = self.current_path();

        if let Some(parent) = self.navigation_manager.get_parent_directory(&current_path) {
            self.navigate_to(&parent)
        } else {
            Err(AppError::FileSystemCustom(
                "上位ディレクトリが存在しません".to_string(),
            ))
        }
    }

    /// 履歴で戻る
    pub fn go_back(&self) -> Result<(), AppError> {
        if !self.config.enable_history {
            return Err(AppError::FileSystemCustom(
                "履歴ナビゲーションが無効です".to_string(),
            ));
        }

        let current_path = self.current_path();
        let mut state = self.state.get();

        if let Some(back_path) = state.history_back.pop() {
            // 現在のパスを進む履歴に追加
            state.history_forward.push(current_path);
            state.current_path = back_path.clone();
            self.state.set(state);

            // コールバック呼び出し
            if let Some(callback) = &self.on_path_change {
                callback(back_path);
            }

            Ok(())
        } else {
            Err(AppError::FileSystemCustom(
                "戻る履歴がありません".to_string(),
            ))
        }
    }

    /// 履歴で進む
    pub fn go_forward(&self) -> Result<(), AppError> {
        if !self.config.enable_history {
            return Err(AppError::FileSystemCustom(
                "履歴ナビゲーションが無効です".to_string(),
            ));
        }

        let current_path = self.current_path();
        let mut state = self.state.get();

        if let Some(forward_path) = state.history_forward.pop() {
            // 現在のパスを戻る履歴に追加
            state.history_back.push(current_path);
            state.current_path = forward_path.clone();
            self.state.set(state);

            // コールバック呼び出し
            if let Some(callback) = &self.on_path_change {
                callback(forward_path);
            }

            Ok(())
        } else {
            Err(AppError::FileSystemCustom(
                "進む履歴がありません".to_string(),
            ))
        }
    }

    /// 戻る履歴があるかどうか
    pub fn can_go_back(&self) -> bool {
        !self.state.get().history_back.is_empty()
    }

    /// 進む履歴があるかどうか
    pub fn can_go_forward(&self) -> bool {
        !self.state.get().history_forward.is_empty()
    }

    /// 上位ディレクトリに移動可能かどうか
    pub fn can_go_up(&self) -> bool {
        if !self.config.enable_parent_navigation {
            return false;
        }

        let current_path = self.current_path();
        self.navigation_manager
            .get_parent_directory(&current_path)
            .is_some()
    }

    /// エラー処理
    fn handle_error(&self, error_message: String) {
        self.state.update(|state| {
            state.last_error = Some(error_message.clone());
        });

        if let Some(callback) = &self.on_error {
            callback(error_message);
        }
    }

    /// 最後のエラーを取得
    pub fn last_error(&self) -> Option<String> {
        self.state.get().last_error
    }

    /// エラーをクリア
    pub fn clear_error(&self) {
        self.state.update(|state| {
            state.last_error = None;
        });
    }
}

/// ファイルアイテムにダブルクリックイベントを追加
pub fn with_double_click_handler<V: View + 'static>(
    view: V,
    entry: FileEntry,
    navigation_manager: Arc<FileNavigationManager>,
) -> impl View {
    view.on_event_stop(floem::event::EventListener::DoubleClick, move |_event| {
        navigation_manager.handle_double_click(&entry);
    })
}

/// ナビゲーションボタン用のヘルパー関数
pub mod navigation_helpers {
    use super::*;
    use floem::peniko::Color;
    use floem::views::{Decorators, button, h_stack, text};

    /// 戻るボタンを作成
    pub fn back_button(navigation_manager: Arc<FileNavigationManager>) -> impl IntoView {
        let can_go_back = navigation_manager.can_go_back();

        button(text("← 戻る"))
            .action(move || {
                if let Err(e) = navigation_manager.go_back() {
                    eprintln!("戻るエラー: {}", e);
                }
            })
            .style(move |s| {
                s.padding_horiz(12)
                    .padding_vert(6)
                    .border_radius(4)
                    .color(if can_go_back {
                        Color::rgb8(59, 130, 246)
                    } else {
                        Color::rgb8(156, 163, 175)
                    })
                    .background(Color::rgb8(248, 250, 252))
                    .border(1)
                    .border_color(Color::rgb8(229, 231, 235))
                    .cursor(if can_go_back {
                        floem::style::CursorStyle::Pointer
                    } else {
                        floem::style::CursorStyle::Default
                    })
                    .hover(|s| {
                        if can_go_back {
                            s.background(Color::rgb8(239, 246, 255))
                        } else {
                            s
                        }
                    })
            })
            .disabled(move || !can_go_back)
    }

    /// 進むボタンを作成
    pub fn forward_button(navigation_manager: Arc<FileNavigationManager>) -> impl IntoView {
        let can_go_forward = navigation_manager.can_go_forward();

        button(text("進む →"))
            .action(move || {
                if let Err(e) = navigation_manager.go_forward() {
                    eprintln!("進むエラー: {}", e);
                }
            })
            .style(move |s| {
                s.padding_horiz(12)
                    .padding_vert(6)
                    .border_radius(4)
                    .color(if can_go_forward {
                        Color::rgb8(59, 130, 246)
                    } else {
                        Color::rgb8(156, 163, 175)
                    })
                    .background(Color::rgb8(248, 250, 252))
                    .border(1)
                    .border_color(Color::rgb8(229, 231, 235))
                    .cursor(if can_go_forward {
                        floem::style::CursorStyle::Pointer
                    } else {
                        floem::style::CursorStyle::Default
                    })
                    .hover(|s| {
                        if can_go_forward {
                            s.background(Color::rgb8(239, 246, 255))
                        } else {
                            s
                        }
                    })
            })
            .disabled(move || !can_go_forward)
    }

    /// 上位フォルダボタンを作成
    pub fn up_button(navigation_manager: Arc<FileNavigationManager>) -> impl IntoView {
        let can_go_up = navigation_manager.can_go_up();

        button(text("↑ 上位フォルダ"))
            .action(move || {
                if let Err(e) = navigation_manager.navigate_up() {
                    eprintln!("上位フォルダエラー: {}", e);
                }
            })
            .style(move |s| {
                s.padding_horiz(12)
                    .padding_vert(6)
                    .border_radius(4)
                    .color(if can_go_up {
                        Color::rgb8(59, 130, 246)
                    } else {
                        Color::rgb8(156, 163, 175)
                    })
                    .background(Color::rgb8(248, 250, 252))
                    .border(1)
                    .border_color(Color::rgb8(229, 231, 235))
                    .cursor(if can_go_up {
                        floem::style::CursorStyle::Pointer
                    } else {
                        floem::style::CursorStyle::Default
                    })
                    .hover(|s| {
                        if can_go_up {
                            s.background(Color::rgb8(239, 246, 255))
                        } else {
                            s
                        }
                    })
            })
            .disabled(move || !can_go_up)
    }

    /// ナビゲーションツールバーを作成
    pub fn navigation_toolbar(navigation_manager: Arc<FileNavigationManager>) -> impl IntoView {
        h_stack((
            back_button(navigation_manager.clone()),
            forward_button(navigation_manager.clone()),
            up_button(navigation_manager),
        ))
        .style(|s| {
            s.gap(8)
                .padding(8)
                .background(Color::rgb8(250, 250, 250))
                .border_radius(6)
                .border(1)
                .border_color(Color::rgb8(229, 231, 235))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_file_navigation_config_default() {
        let config = FileNavigationConfig::default();
        assert!(config.enable_double_click);
        assert!(config.enable_parent_navigation);
        assert!(config.enable_history);
    }

    #[test]
    fn test_file_navigation_state_creation() {
        let path = PathBuf::from("/home/user");
        let state = FileNavigationState::new(path.clone());

        assert_eq!(state.current_path, path);
        assert!(state.history_back.is_empty());
        assert!(state.history_forward.is_empty());
        assert!(state.last_error.is_none());
    }

    #[test]
    fn test_file_navigation_manager_creation() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir.clone());

        assert_eq!(manager.current_path(), current_dir);
        assert!(!manager.can_go_back());
        assert!(!manager.can_go_forward());
    }

    #[test]
    fn test_navigate_to_valid_directory() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir.clone());

        // 同じディレクトリに移動（有効な操作）
        let result = manager.navigate_to(&current_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_navigate_up() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir);

        // 上位ディレクトリに移動可能かテスト
        let can_go_up = manager.can_go_up();

        if can_go_up {
            let result = manager.navigate_up();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_history_functionality() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir.clone());

        // 初期状態では履歴がない
        assert!(!manager.can_go_back());
        assert!(!manager.can_go_forward());

        // 同じパスに移動しても履歴は追加されない
        let _ = manager.navigate_to(&current_dir);
        assert!(!manager.can_go_back());
    }

    #[test]
    fn test_double_click_file_entry() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir.clone());

        // ディレクトリエントリのダブルクリック
        let dir_entry = FileEntry {
            name: "test_dir".to_string(),
            path: current_dir.clone(),
            file_type: FileType::Directory,
            size: 0,
            modified: None,
        };

        // エラーが発生しないことをテスト（実際の移動は現在のディレクトリなので問題なし）
        manager.handle_double_click(&dir_entry);

        // エラーが設定されていないことを確認
        assert!(manager.last_error().is_none());
    }

    #[test]
    fn test_error_handling() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir);

        // 存在しないディレクトリに移動を試行
        let invalid_path = PathBuf::from("/this/path/does/not/exist");
        let result = manager.navigate_to(&invalid_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_clear_error() {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manager = FileNavigationManager::with_default(current_dir);

        // エラーを人工的に設定
        manager.handle_error("テストエラー".to_string());
        assert!(manager.last_error().is_some());

        // エラーをクリア
        manager.clear_error();
        assert!(manager.last_error().is_none());
    }
}
