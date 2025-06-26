//! ファイルシステムAPIのテスト

use crate::filesystem::{FileSystemApi, FileSystemManager, FileType};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio;

/// テスト用の一時ディレクトリとファイルを作成
async fn create_test_structure() -> Result<TempDir, Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // テストファイルを作成
    fs::write(temp_path.join("test_file.txt"), "test content")?;

    // テストディレクトリを作成
    fs::create_dir(temp_path.join("test_dir"))?;
    fs::write(
        temp_path.join("test_dir").join("nested_file.txt"),
        "nested content",
    )?;

    // 隠しファイルを作成
    fs::write(temp_path.join(".hidden_file"), "hidden content")?;

    Ok(temp_dir)
}

#[tokio::test]
async fn test_list_directory_success() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();

    let entries = manager.list_directory(temp_dir.path()).await.unwrap();

    // 3つのエントリが存在することを確認（ファイル、ディレクトリ、隠しファイル）
    assert_eq!(entries.len(), 3);

    // ファイルタイプの確認
    let file_entry = entries.iter().find(|e| e.name == "test_file.txt").unwrap();
    assert_eq!(file_entry.file_type, FileType::File);
    assert!(file_entry.size > 0);

    let dir_entry = entries.iter().find(|e| e.name == "test_dir").unwrap();
    assert_eq!(dir_entry.file_type, FileType::Directory);

    let hidden_entry = entries.iter().find(|e| e.name == ".hidden_file").unwrap();
    assert_eq!(hidden_entry.file_type, FileType::File);
}

#[tokio::test]
async fn test_list_directory_invalid_path() {
    let manager = FileSystemManager::new();
    let invalid_path = PathBuf::from("/nonexistent/path");

    let result = manager.list_directory(&invalid_path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_directory_file_not_directory() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();
    let file_path = temp_dir.path().join("test_file.txt");

    let result = manager.list_directory(&file_path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_file_info_success() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();
    let file_path = temp_dir.path().join("test_file.txt");

    let file_info = manager.get_file_info(&file_path).await.unwrap();

    assert_eq!(file_info.path, file_path);
    assert_eq!(file_info.file_type, FileType::File);
    assert!(file_info.size > 0);
    assert!(file_info.modified.is_some());
}

#[tokio::test]
async fn test_get_file_info_directory() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();
    let dir_path = temp_dir.path().join("test_dir");

    let file_info = manager.get_file_info(&dir_path).await.unwrap();

    assert_eq!(file_info.path, dir_path);
    assert_eq!(file_info.file_type, FileType::Directory);
    assert!(file_info.modified.is_some());
}

#[tokio::test]
async fn test_get_file_info_invalid_path() {
    let manager = FileSystemManager::new();
    let invalid_path = PathBuf::from("/nonexistent/file.txt");

    let result = manager.get_file_info(&invalid_path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_is_accessible() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();

    // 存在するファイルへのアクセス
    let file_path = temp_dir.path().join("test_file.txt");
    assert!(manager.is_accessible(&file_path));

    // 存在するディレクトリへのアクセス
    let dir_path = temp_dir.path().join("test_dir");
    assert!(manager.is_accessible(&dir_path));

    // 存在しないパスへのアクセス
    let invalid_path = PathBuf::from("/nonexistent/path");
    assert!(!manager.is_accessible(&invalid_path));
}

#[tokio::test]
async fn test_directory_entries_sorted() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // 複数のファイルを作成（意図的に名前順でない順序で）
    fs::write(temp_path.join("z_file.txt"), "content").unwrap();
    fs::write(temp_path.join("a_file.txt"), "content").unwrap();
    fs::write(temp_path.join("m_file.txt"), "content").unwrap();

    let manager = FileSystemManager::new();
    let entries = manager.list_directory(temp_path).await.unwrap();

    // エントリが名前順でソートされていることを確認
    let names: Vec<&String> = entries.iter().map(|e| &e.name).collect();
    let mut sorted_names = names.clone();
    sorted_names.sort();

    assert_eq!(names, sorted_names);
}

#[tokio::test]
async fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let manager = FileSystemManager::new();

    let entries = manager.list_directory(temp_dir.path()).await.unwrap();
    assert!(entries.is_empty());
}

#[tokio::test]
async fn test_nested_directory_access() {
    let temp_dir = create_test_structure().await.unwrap();
    let manager = FileSystemManager::new();
    let nested_dir = temp_dir.path().join("test_dir");

    let entries = manager.list_directory(&nested_dir).await.unwrap();
    assert_eq!(entries.len(), 1);

    let nested_file = entries
        .iter()
        .find(|e| e.name == "nested_file.txt")
        .unwrap();
    assert_eq!(nested_file.file_type, FileType::File);
}
