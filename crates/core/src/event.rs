//! イベント管理システム

use rust_explorer_utils::AppError;

/// イベントタイプ
#[derive(Debug, Clone)]
pub enum Event {
    /// ファイルが選択された
    FileSelected(std::path::PathBuf),
    /// ディレクトリが変更された
    DirectoryChanged(std::path::PathBuf),
    /// タブが作成された
    TabCreated(String),
    /// タブが閉じられた
    TabClosed(String),
}

/// イベント管理
pub struct EventManager {
    // 将来的にはイベントハンドラーのリストを保持
}

impl EventManager {
    /// 新しいイベントマネージャーを作成
    pub fn new() -> Self {
        Self {}
    }
    
    /// イベントを処理
    pub fn handle_event(&self, event: Event) -> Result<(), AppError> {
        match event {
            Event::FileSelected(path) => {
                // ファイル選択処理
                println!("File selected: {:?}", path);
            }
            Event::DirectoryChanged(path) => {
                // ディレクトリ変更処理
                println!("Directory changed: {:?}", path);
            }
            Event::TabCreated(name) => {
                // タブ作成処理
                println!("Tab created: {}", name);
            }
            Event::TabClosed(name) => {
                // タブ閉じる処理
                println!("Tab closed: {}", name);
            }
        }
        Ok(())
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}