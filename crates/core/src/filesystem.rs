//! ファイルシステム操作

use rust_explorer_utils::AppError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;

/// ファイルタイプ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    SymLink,
    Other,
}

/// ファイルエントリ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

/// ファイル情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub permissions: Option<String>,
}

/// ファイルシステムAPI トレイト
#[async_trait::async_trait]
pub trait FileSystemApi {
    /// ディレクトリ一覧取得
    async fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>, AppError>;

    /// ファイル情報取得
    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, AppError>;

    /// アクセス可能かチェック
    fn is_accessible(&self, path: &Path) -> bool;
}

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
    pub async fn list_directory_sync<P: AsRef<Path>>(
        &self,
        _path: P,
    ) -> Result<Vec<PathBuf>, AppError> {
        // 将来の実装でディレクトリ内容を返す
        Ok(vec![])
    }
}

/// ファイルシステムAPI の実装
#[async_trait::async_trait]
impl FileSystemApi for FileSystemManager {
    async fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>, AppError> {
        if !path.exists() {
            return Err(AppError::InvalidPath(path.to_path_buf()));
        }

        if !path.is_dir() {
            return Err(AppError::InvalidPath(path.to_path_buf()));
        }

        let mut entries = Vec::new();
        let mut dir = fs::read_dir(path)
            .await
            .map_err(AppError::FileSystem)?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(AppError::FileSystem)?
        {
            let path = entry.path();
            let metadata = entry
                .metadata()
                .await
                .map_err(AppError::FileSystem)?;

            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_file() {
                FileType::File
            } else if metadata.file_type().is_symlink() {
                FileType::SymLink
            } else {
                FileType::Other
            };

            let name = entry.file_name().to_string_lossy().to_string();
            let size = metadata.len();
            let modified = metadata.modified().ok();

            entries.push(FileEntry {
                name,
                path,
                file_type,
                size,
                modified,
            });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, AppError> {
        if !path.exists() {
            return Err(AppError::InvalidPath(path.to_path_buf()));
        }

        let metadata = fs::metadata(path)
            .await
            .map_err(AppError::FileSystem)?;

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_file() {
            FileType::File
        } else if metadata.file_type().is_symlink() {
            FileType::SymLink
        } else {
            FileType::Other
        };

        let size = metadata.len();
        let modified = metadata.modified().ok();
        let permissions = None; // クロスプラットフォーム対応のため簡略化

        Ok(FileInfo {
            path: path.to_path_buf(),
            file_type,
            size,
            modified,
            permissions,
        })
    }

    fn is_accessible(&self, path: &Path) -> bool {
        path.exists() && (path.is_file() || path.is_dir())
    }
}

impl Default for FileSystemManager {
    fn default() -> Self {
        Self::new()
    }
}

/// キャッシュ付きファイルシステム管理（将来の実装）
pub struct CachedFileSystemManager {
    inner: FileSystemManager,
    // cache: HashMap<PathBuf, (Vec<FileEntry>, SystemTime)>,
}

impl CachedFileSystemManager {
    pub fn new() -> Self {
        Self {
            inner: FileSystemManager::new(),
        }
    }
}

#[async_trait::async_trait]
impl FileSystemApi for CachedFileSystemManager {
    async fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>, AppError> {
        // 将来的にキャッシュ機能を実装
        self.inner.list_directory(path).await
    }

    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, AppError> {
        self.inner.get_file_info(path).await
    }

    fn is_accessible(&self, path: &Path) -> bool {
        self.inner.is_accessible(path)
    }
}

impl Default for CachedFileSystemManager {
    fn default() -> Self {
        Self::new()
    }
}
