//! ファイルシステム操作

use rust_explorer_utils::AppError;
use std::path::{Path, PathBuf};

/// ファイルシステム管理
pub struct FileSystemManager {
    current_path: PathBuf,
}

impl FileSystemManager {
    /// 新しいファイルシステムマネージャーを作成
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
        }
    }
    
    /// 現在のパスを取得
    pub fn current_path(&self) -> &Path {
        &self.current_path
    }
    
    /// パスを変更
    pub fn change_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AppError> {
        let path = path.as_ref();
        if path.exists() && path.is_dir() {
            self.current_path = path.to_path_buf();
            Ok(())
        } else {
            Err(AppError::InvalidPath(path.to_path_buf()))
        }
    }
    
    /// ディレクトリの内容を一覧取得（将来の実装）
    pub fn list_directory<P: AsRef<Path>>(&self, _path: P) -> Result<Vec<PathBuf>, AppError> {
        // 将来の実装でディレクトリ内容を返す
        Ok(vec![])
    }
}

impl Default for FileSystemManager {
    fn default() -> Self {
        Self::new()
    }
}