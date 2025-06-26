//! 状態の永続化・復元機能

use rust_explorer_utils::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 状態の永続化設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatePersistenceConfig {
    /// 状態ファイルの保存ディレクトリ
    pub state_dir: PathBuf,
    /// 自動保存の間隔（秒）
    pub auto_save_interval: u64,
    /// 自動保存を有効にするか
    pub auto_save_enabled: bool,
    /// 最大保持するバックアップ数
    pub max_backups: u32,
}

impl Default for StatePersistenceConfig {
    fn default() -> Self {
        Self {
            state_dir: default_state_dir(),
            auto_save_interval: 30, // 30秒
            auto_save_enabled: true,
            max_backups: 5,
        }
    }
}

/// 状態永続化マネージャー
pub struct StatePersistenceManager {
    config: StatePersistenceConfig,
}

impl StatePersistenceManager {
    /// 新しい状態永続化マネージャーを作成
    pub fn new(config: StatePersistenceConfig) -> Result<Self, AppError> {
        let manager = Self { config };

        // 状態ディレクトリを作成
        manager.ensure_state_dir()?;

        Ok(manager)
    }

    /// デフォルト設定でマネージャーを作成
    pub fn with_default_config() -> Result<Self, AppError> {
        Self::new(StatePersistenceConfig::default())
    }

    /// 状態をファイルに保存
    pub fn save_state<T: Serialize>(&self, state: &T, filename: &str) -> Result<(), AppError> {
        let file_path = self.config.state_dir.join(filename);

        // バックアップを作成
        self.create_backup(&file_path)?;

        // JSONとして保存
        let json = serde_json::to_string_pretty(state).map_err(AppError::Json)?;

        fs::write(&file_path, json).map_err(AppError::FileSystem)?;

        // 古いバックアップを清理
        self.cleanup_old_backups(&file_path)?;

        Ok(())
    }

    /// 状態をファイルから復元
    pub fn load_state<T: for<'de> Deserialize<'de>>(&self, filename: &str) -> Result<T, AppError> {
        let file_path = self.config.state_dir.join(filename);

        if !file_path.exists() {
            return Err(AppError::InvalidPath(file_path));
        }

        let content = fs::read_to_string(&file_path).map_err(AppError::FileSystem)?;

        let state: T = serde_json::from_str(&content).map_err(AppError::Json)?;

        Ok(state)
    }

    /// 状態ファイルが存在するかチェック
    pub fn state_exists(&self, filename: &str) -> bool {
        self.config.state_dir.join(filename).exists()
    }

    /// 状態ファイルを削除
    pub fn delete_state(&self, filename: &str) -> Result<(), AppError> {
        let file_path = self.config.state_dir.join(filename);

        if file_path.exists() {
            fs::remove_file(&file_path).map_err(AppError::FileSystem)?;
        }

        Ok(())
    }

    /// すべての状態ファイルを一覧取得
    pub fn list_state_files(&self) -> Result<Vec<String>, AppError> {
        let mut files = Vec::new();

        if !self.config.state_dir.exists() {
            return Ok(files);
        }

        let entries = fs::read_dir(&self.config.state_dir).map_err(AppError::FileSystem)?;

        for entry in entries {
            let entry = entry.map_err(AppError::FileSystem)?;
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        if filename_str.ends_with(".json") {
                            files.push(filename_str.to_string());
                        }
                    }
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// 最新のバックアップから状態を復元
    pub fn restore_from_backup<T: for<'de> Deserialize<'de>>(
        &self,
        filename: &str,
    ) -> Result<T, AppError> {
        let file_path = self.config.state_dir.join(filename);
        let backup_path = self.get_latest_backup_path(&file_path)?;

        let content = fs::read_to_string(&backup_path).map_err(AppError::FileSystem)?;

        let state: T = serde_json::from_str(&content).map_err(AppError::Json)?;

        Ok(state)
    }

    /// 利用可能なバックアップファイルを一覧取得
    pub fn list_backups(&self, filename: &str) -> Result<Vec<PathBuf>, AppError> {
        let file_path = self.config.state_dir.join(filename);
        let backup_prefix = format!(
            "{}.backup.",
            file_path
                .file_name()
                .ok_or_else(|| AppError::Internal("Invalid filename".to_string()))?
                .to_string_lossy()
        );

        let mut backups = Vec::new();

        let entries = fs::read_dir(&self.config.state_dir).map_err(AppError::FileSystem)?;

        for entry in entries {
            let entry = entry.map_err(AppError::FileSystem)?;
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if filename.to_string_lossy().starts_with(&backup_prefix) {
                        backups.push(path);
                    }
                }
            }
        }

        // 新しい順にソート
        backups.sort_by(|a, b| {
            let a_metadata = a.metadata().ok();
            let b_metadata = b.metadata().ok();
            match (a_metadata, b_metadata) {
                (Some(a_meta), Some(b_meta)) => b_meta
                    .modified()
                    .unwrap_or(std::time::UNIX_EPOCH)
                    .cmp(&a_meta.modified().unwrap_or(std::time::UNIX_EPOCH)),
                _ => std::cmp::Ordering::Equal,
            }
        });

        Ok(backups)
    }

    /// 状態ディレクトリを作成
    fn ensure_state_dir(&self) -> Result<(), AppError> {
        if !self.config.state_dir.exists() {
            fs::create_dir_all(&self.config.state_dir).map_err(AppError::FileSystem)?;
        }
        Ok(())
    }

    /// バックアップファイルを作成
    fn create_backup(&self, file_path: &Path) -> Result<(), AppError> {
        if file_path.exists() {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let backup_filename = format!(
                "{}.backup.{}.json",
                file_path
                    .file_stem()
                    .ok_or_else(|| AppError::Internal("Invalid file path".to_string()))?
                    .to_string_lossy(),
                timestamp
            );
            let backup_path = file_path
                .parent()
                .ok_or_else(|| AppError::Internal("Invalid file path".to_string()))?
                .join(backup_filename);

            fs::copy(file_path, &backup_path).map_err(AppError::FileSystem)?;
        }
        Ok(())
    }

    /// 古いバックアップを削除
    fn cleanup_old_backups(&self, file_path: &Path) -> Result<(), AppError> {
        let backups = self.list_backups(
            &file_path
                .file_name()
                .ok_or_else(|| AppError::Internal("Invalid file path".to_string()))?
                .to_string_lossy(),
        )?;

        if backups.len() > self.config.max_backups as usize {
            let to_remove = &backups[self.config.max_backups as usize..];
            for backup in to_remove {
                if let Err(e) = fs::remove_file(backup) {
                    eprintln!("Warning: Failed to remove backup file: {}", e);
                }
            }
        }

        Ok(())
    }

    /// 最新のバックアップファイルパスを取得
    fn get_latest_backup_path(&self, file_path: &Path) -> Result<PathBuf, AppError> {
        let backups = self.list_backups(
            &file_path
                .file_name()
                .ok_or_else(|| AppError::Internal("Invalid file path".to_string()))?
                .to_string_lossy(),
        )?;

        backups
            .first()
            .cloned()
            .ok_or_else(|| AppError::Internal("No backup files found".to_string()))
    }
}

/// デフォルトの状態保存ディレクトリを取得
fn default_state_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("rust-explorer").join("state")
    } else {
        PathBuf::from(".").join("state")
    }
}

/// アプリケーション状態の永続化ヘルパー関数
pub mod state_helpers {
    use super::*;

    /// アプリケーション状態を保存
    pub fn save_app_state<T: Serialize>(state: &T) -> Result<(), AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.save_state(state, "app_state.json")
    }

    /// アプリケーション状態を復元
    pub fn load_app_state<T: for<'de> Deserialize<'de>>() -> Result<T, AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.load_state("app_state.json")
    }

    /// アプリケーション状態ファイルが存在するかチェック
    pub fn app_state_exists() -> Result<bool, AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        Ok(manager.state_exists("app_state.json"))
    }

    /// ウィンドウ状態を保存
    pub fn save_window_state<T: Serialize>(state: &T) -> Result<(), AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.save_state(state, "window_state.json")
    }

    /// ウィンドウ状態を復元
    pub fn load_window_state<T: for<'de> Deserialize<'de>>() -> Result<T, AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.load_state("window_state.json")
    }

    /// セッション状態を保存
    pub fn save_session_state<T: Serialize>(state: &T) -> Result<(), AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.save_state(state, "session_state.json")
    }

    /// セッション状態を復元
    pub fn load_session_state<T: for<'de> Deserialize<'de>>() -> Result<T, AppError> {
        let manager = StatePersistenceManager::with_default_config()?;
        manager.load_state("session_state.json")
    }
}
