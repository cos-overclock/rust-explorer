//! システム統合機能
//!
//! OS固有のファイル操作とシステム統合を提供します。

use rust_explorer_utils::AppError;
use std::path::Path;
use std::process::Command;

/// システム統合操作を提供するトレイト
pub trait SystemIntegration {
    /// ファイルを既定のアプリケーションで開く
    fn open_file(&self, path: &Path) -> Result<(), AppError>;

    /// フォルダをファイルマネージャーで開く
    fn open_folder(&self, path: &Path) -> Result<(), AppError>;

    /// 指定されたパスにアクセス可能かチェック
    fn is_accessible(&self, path: &Path) -> bool;
}

/// デフォルトのシステム統合実装
pub struct DefaultSystemIntegration;

impl DefaultSystemIntegration {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultSystemIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemIntegration for DefaultSystemIntegration {
    fn open_file(&self, path: &Path) -> Result<(), AppError> {
        if !path.exists() {
            return Err(AppError::FileSystemCustom(format!(
                "ファイルが存在しません: {}",
                path.display()
            )));
        }

        if !path.is_file() {
            return Err(AppError::FileSystemCustom(format!(
                "指定されたパスはファイルではありません: {}",
                path.display()
            )));
        }

        let result = if cfg!(target_os = "windows") {
            // Windows: start コマンドを使用
            Command::new("cmd")
                .args(["/C", "start", "", &path.to_string_lossy()])
                .spawn()
        } else if cfg!(target_os = "macos") {
            // macOS: open コマンドを使用
            Command::new("open").arg(path).spawn()
        } else if cfg!(target_os = "linux") {
            // Linux: xdg-open コマンドを使用
            Command::new("xdg-open").arg(path).spawn()
        } else {
            return Err(AppError::FileSystemCustom(
                "サポートされていないオペレーティングシステムです".to_string(),
            ));
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::FileSystemCustom(format!(
                "ファイルを開けませんでした: {}. エラー: {}",
                path.display(),
                e
            ))),
        }
    }

    fn open_folder(&self, path: &Path) -> Result<(), AppError> {
        if !path.exists() {
            return Err(AppError::FileSystemCustom(format!(
                "フォルダが存在しません: {}",
                path.display()
            )));
        }

        if !path.is_dir() {
            return Err(AppError::FileSystemCustom(format!(
                "指定されたパスはフォルダではありません: {}",
                path.display()
            )));
        }

        let result = if cfg!(target_os = "windows") {
            // Windows: explorer コマンドを使用
            Command::new("explorer").arg(path).spawn()
        } else if cfg!(target_os = "macos") {
            // macOS: open コマンドを使用
            Command::new("open").arg(path).spawn()
        } else if cfg!(target_os = "linux") {
            // Linux: xdg-open コマンドを使用
            Command::new("xdg-open").arg(path).spawn()
        } else {
            return Err(AppError::FileSystemCustom(
                "サポートされていないオペレーティングシステムです".to_string(),
            ));
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::FileSystemCustom(format!(
                "フォルダを開けませんでした: {}. エラー: {}",
                path.display(),
                e
            ))),
        }
    }

    fn is_accessible(&self, path: &Path) -> bool {
        path.exists() && path.metadata().is_ok()
    }
}

/// ファイルナビゲーション操作を管理する構造体
pub struct FileNavigationManager {
    system_integration: Box<dyn SystemIntegration + Send + Sync>,
}

impl FileNavigationManager {
    /// 新しいFileNavigationManagerを作成
    pub fn new(system_integration: Box<dyn SystemIntegration + Send + Sync>) -> Self {
        Self { system_integration }
    }

    /// デフォルトのFileNavigationManagerを作成
    pub fn with_default() -> Self {
        Self::new(Box::new(DefaultSystemIntegration::new()))
    }

    /// ファイルまたはフォルダを開く
    pub fn open_item(&self, path: &Path) -> Result<(), AppError> {
        if !self.system_integration.is_accessible(path) {
            return Err(AppError::FileSystemCustom(format!(
                "アクセスできません: {}",
                path.display()
            )));
        }

        if path.is_file() {
            self.system_integration.open_file(path)
        } else if path.is_dir() {
            self.system_integration.open_folder(path)
        } else {
            Err(AppError::FileSystemCustom(format!(
                "未知のファイルタイプです: {}",
                path.display()
            )))
        }
    }

    /// ディレクトリに移動可能かチェック
    pub fn can_navigate_to(&self, path: &Path) -> bool {
        path.exists() && path.is_dir() && self.system_integration.is_accessible(path)
    }

    /// 上位ディレクトリのパスを取得
    pub fn get_parent_directory(&self, current_path: &Path) -> Option<std::path::PathBuf> {
        current_path.parent().map(|p| p.to_path_buf())
    }

    /// 指定されたディレクトリに移動可能かチェック
    pub fn validate_navigation(&self, target_path: &Path) -> Result<(), AppError> {
        if !target_path.exists() {
            return Err(AppError::FileSystemCustom(format!(
                "ディレクトリが存在しません: {}",
                target_path.display()
            )));
        }

        if !target_path.is_dir() {
            return Err(AppError::FileSystemCustom(format!(
                "指定されたパスはディレクトリではありません: {}",
                target_path.display()
            )));
        }

        if !self.system_integration.is_accessible(target_path) {
            return Err(AppError::FileSystemCustom(format!(
                "ディレクトリにアクセスできません: {}",
                target_path.display()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // モックシステム統合実装
    struct MockSystemIntegration {
        accessible_paths: Vec<PathBuf>,
        should_fail_open: bool,
    }

    impl MockSystemIntegration {
        fn new() -> Self {
            Self {
                accessible_paths: vec![
                    PathBuf::from("/home/user"),
                    PathBuf::from("/home/user/Documents"),
                    PathBuf::from("/home/user/Documents/test.txt"),
                ],
                should_fail_open: false,
            }
        }

        fn with_fail_open() -> Self {
            Self {
                accessible_paths: vec![PathBuf::from("/home/user")],
                should_fail_open: true,
            }
        }
    }

    impl SystemIntegration for MockSystemIntegration {
        fn open_file(&self, path: &Path) -> Result<(), AppError> {
            if self.should_fail_open {
                return Err(AppError::FileSystemCustom("モックエラー".to_string()));
            }

            if self.accessible_paths.contains(&path.to_path_buf()) {
                Ok(())
            } else {
                Err(AppError::FileSystemCustom(
                    "ファイルが見つかりません".to_string(),
                ))
            }
        }

        fn open_folder(&self, path: &Path) -> Result<(), AppError> {
            if self.should_fail_open {
                return Err(AppError::FileSystemCustom("モックエラー".to_string()));
            }

            if self.accessible_paths.contains(&path.to_path_buf()) {
                Ok(())
            } else {
                Err(AppError::FileSystemCustom(
                    "フォルダが見つかりません".to_string(),
                ))
            }
        }

        fn is_accessible(&self, path: &Path) -> bool {
            self.accessible_paths.contains(&path.to_path_buf())
        }
    }

    #[test]
    fn test_default_system_integration_creation() {
        let integration = DefaultSystemIntegration::new();
        let default_integration = DefaultSystemIntegration::default();

        // 作成が成功することをテスト
        assert!(integration.is_accessible(Path::new(".")));
        assert!(default_integration.is_accessible(Path::new(".")));
    }

    #[test]
    fn test_file_navigation_manager_creation() {
        let manager = FileNavigationManager::with_default();

        // 現在のディレクトリがアクセス可能であることをテスト
        assert!(manager.can_navigate_to(Path::new(".")));
    }

    #[test]
    fn test_file_navigation_manager_with_mock() {
        let mock = Box::new(MockSystemIntegration::new());
        let manager = FileNavigationManager::new(mock);

        // モックで定義されたパスがアクセス可能であることをテスト
        assert!(manager.can_navigate_to(Path::new("/home/user")));
        assert!(!manager.can_navigate_to(Path::new("/nonexistent")));
    }

    #[test]
    fn test_open_item_success() {
        let mock = Box::new(MockSystemIntegration::new());
        let manager = FileNavigationManager::new(mock);

        // 存在するファイルを開く（モックでは成功する）
        let result = manager.open_item(Path::new("/home/user/Documents/test.txt"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_item_failure() {
        let mock = Box::new(MockSystemIntegration::with_fail_open());
        let manager = FileNavigationManager::new(mock);

        // モックでエラーが発生することをテスト
        let result = manager.open_item(Path::new("/home/user"));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_parent_directory() {
        let manager = FileNavigationManager::with_default();

        let current = Path::new("/home/user/Documents");
        let parent = manager.get_parent_directory(current);

        assert!(parent.is_some());
        assert_eq!(parent.unwrap(), PathBuf::from("/home/user"));

        // ルートディレクトリの場合
        let root = Path::new("/");
        let root_parent = manager.get_parent_directory(root);
        assert!(root_parent.is_none());
    }

    #[test]
    fn test_validate_navigation() {
        let mock = Box::new(MockSystemIntegration::new());
        let manager = FileNavigationManager::new(mock);

        // アクセス可能なディレクトリ
        let result = manager.validate_navigation(Path::new("/home/user"));
        assert!(result.is_ok());

        // アクセス不可能なディレクトリ
        let result = manager.validate_navigation(Path::new("/nonexistent"));
        assert!(result.is_err());
    }

    #[test]
    fn test_is_accessible() {
        let integration = DefaultSystemIntegration::new();

        // 現在のディレクトリはアクセス可能
        assert!(integration.is_accessible(Path::new(".")));

        // 存在しないパスはアクセス不可能
        assert!(!integration.is_accessible(Path::new("/this/path/does/not/exist")));
    }
}
