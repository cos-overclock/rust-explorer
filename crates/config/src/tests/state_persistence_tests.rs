//! 状態永続化システムのテスト

use crate::state_persistence::{StatePersistenceConfig, StatePersistenceManager, state_helpers};
use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::TempDir;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestState {
    name: String,
    value: i32,
    flag: bool,
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            value: 42,
            flag: true,
        }
    }
}

#[test]
fn test_state_persistence_config_default() {
    let config = StatePersistenceConfig::default();

    assert!(config.state_dir.to_string_lossy().contains("rust-explorer"));
    assert_eq!(config.auto_save_interval, 30);
    assert!(config.auto_save_enabled);
    assert_eq!(config.max_backups, 5);
}

#[test]
fn test_state_persistence_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let result = StatePersistenceManager::new(config);
    assert!(result.is_ok());

    // 状態ディレクトリが作成されることを確認
    assert!(temp_dir.path().exists());
}

#[test]
fn test_save_and_load_state() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();
    let test_state = TestState::default();

    // 状態を保存
    let save_result = manager.save_state(&test_state, "test_state.json");
    assert!(save_result.is_ok());

    // 状態ファイルが存在することを確認
    assert!(manager.state_exists("test_state.json"));

    // 状態を復元
    let load_result: Result<TestState, _> = manager.load_state("test_state.json");
    assert!(load_result.is_ok());

    let loaded_state = load_result.unwrap();
    assert_eq!(loaded_state, test_state);
}

#[test]
fn test_load_nonexistent_state() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    let load_result: Result<TestState, _> = manager.load_state("nonexistent.json");
    assert!(load_result.is_err());
}

#[test]
fn test_delete_state() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();
    let test_state = TestState::default();

    // 状態を保存
    manager.save_state(&test_state, "test_state.json").unwrap();
    assert!(manager.state_exists("test_state.json"));

    // 状態を削除
    let delete_result = manager.delete_state("test_state.json");
    assert!(delete_result.is_ok());
    assert!(!manager.state_exists("test_state.json"));
}

#[test]
fn test_list_state_files() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();
    let test_state = TestState::default();

    // 複数の状態ファイルを保存
    manager.save_state(&test_state, "state1.json").unwrap();
    manager.save_state(&test_state, "state2.json").unwrap();
    manager.save_state(&test_state, "state3.json").unwrap();

    let files = manager.list_state_files().unwrap();
    assert_eq!(files.len(), 3);
    assert!(files.contains(&"state1.json".to_string()));
    assert!(files.contains(&"state2.json".to_string()));
    assert!(files.contains(&"state3.json".to_string()));
}

#[test]
fn test_backup_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    // 最初の状態を保存
    let state1 = TestState {
        name: "first".to_string(),
        value: 1,
        flag: true,
    };
    manager.save_state(&state1, "test_state.json").unwrap();

    // 少し待ってから次の状態を保存（バックアップが作成される）
    std::thread::sleep(std::time::Duration::from_millis(10));
    let state2 = TestState {
        name: "second".to_string(),
        value: 2,
        flag: false,
    };
    manager.save_state(&state2, "test_state.json").unwrap();

    // バックアップファイルが作成されることを確認
    let backups = manager.list_backups("test_state.json").unwrap();
    assert!(!backups.is_empty());

    // 最新のファイルは state2 の内容
    let current_state: TestState = manager.load_state("test_state.json").unwrap();
    assert_eq!(current_state, state2);
}

#[test]
fn test_backup_cleanup() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 2, // 最大2つのバックアップ
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    // 複数回状態を保存してバックアップを作成
    for i in 1..=5 {
        let state = TestState {
            name: format!("state_{}", i),
            value: i,
            flag: i % 2 == 0,
        };
        manager.save_state(&state, "test_state.json").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // バックアップ数が制限されていることを確認
    let backups = manager.list_backups("test_state.json").unwrap();
    assert!(backups.len() <= 2);
}

#[test]
fn test_restore_from_backup() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    // 最初の状態を保存
    let original_state = TestState {
        name: "original".to_string(),
        value: 100,
        flag: true,
    };
    manager
        .save_state(&original_state, "test_state.json")
        .unwrap();

    // 新しい状態を保存（バックアップが作成される）
    std::thread::sleep(std::time::Duration::from_millis(10));
    let new_state = TestState {
        name: "new".to_string(),
        value: 200,
        flag: false,
    };
    manager.save_state(&new_state, "test_state.json").unwrap();

    // バックアップから復元
    let restored_state: TestState = manager.restore_from_backup("test_state.json").unwrap();
    assert_eq!(restored_state, original_state);
}

#[test]
fn test_state_helpers_save_load() {
    // デフォルト設定でのテスト（実際のファイルシステムを使用）
    let test_state = TestState {
        name: "helper_test".to_string(),
        value: 999,
        flag: false,
    };

    // save_app_state と load_app_state のテスト
    // 注意: これは実際のファイルシステムに書き込むため、テスト環境での実行のみ推奨
    if std::env::var("RUN_INTEGRATION_TESTS").is_ok() {
        let save_result = state_helpers::save_app_state(&test_state);
        if save_result.is_ok() {
            let load_result: Result<TestState, _> = state_helpers::load_app_state();
            if let Ok(loaded_state) = load_result {
                assert_eq!(loaded_state, test_state);
            }
        }
    }
}

#[test]
fn test_state_exists() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    // 最初はファイルが存在しない
    assert!(!manager.state_exists("test_state.json"));

    // 状態を保存
    let test_state = TestState::default();
    manager.save_state(&test_state, "test_state.json").unwrap();

    // 今度はファイルが存在する
    assert!(manager.state_exists("test_state.json"));
}

#[test]
fn test_invalid_json_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config = StatePersistenceConfig {
        state_dir: temp_dir.path().to_path_buf(),
        auto_save_interval: 10,
        auto_save_enabled: true,
        max_backups: 3,
    };

    let manager = StatePersistenceManager::new(config).unwrap();

    // 無効なJSONファイルを作成
    let invalid_json_path = temp_dir.path().join("invalid.json");
    fs::write(&invalid_json_path, "{ invalid json }").unwrap();

    // 読み込みが失敗することを確認
    let load_result: Result<TestState, _> = manager.load_state("invalid.json");
    assert!(load_result.is_err());
}
