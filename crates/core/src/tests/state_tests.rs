//! 状態管理システムのテスト

use crate::state::{AppState, StateManager, TabState, UiState, WindowState, state_utils};
use std::path::PathBuf;

#[test]
fn test_app_state_default() {
    let state = AppState::default();

    assert!(state.tabs.is_empty());
    assert!(state.panes.is_empty());
    assert!(state.active_tab_id.is_none());
    assert_eq!(state.window.width, 1200.0);
    assert_eq!(state.window.height, 800.0);
    assert!(state.ui.sidebar_visible);
    assert!(state.ui.statusbar_visible);
    assert!(state.ui.toolbar_visible);
}

#[test]
fn test_app_state_add_tab() {
    let mut state = AppState::default();
    let tab = TabState::new(
        "tab1".to_string(),
        "Test Tab".to_string(),
        PathBuf::from("/test"),
    );

    state.add_tab(tab.clone());

    assert_eq!(state.tabs.len(), 1);
    assert_eq!(state.active_tab_id, Some("tab1".to_string()));
    assert!(state.tabs[0].active);
    assert_eq!(state.tabs[0].name, "Test Tab");
}

#[test]
fn test_app_state_add_multiple_tabs() {
    let mut state = AppState::default();

    let tab1 = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    let tab2 = TabState::new(
        "tab2".to_string(),
        "Tab 2".to_string(),
        PathBuf::from("/test2"),
    );

    state.add_tab(tab1);
    state.add_tab(tab2);

    assert_eq!(state.tabs.len(), 2);
    assert_eq!(state.active_tab_id, Some("tab2".to_string()));

    // 最新のタブがアクティブで、他はアクティブでない
    assert!(!state.tabs[0].active);
    assert!(state.tabs[1].active);
}

#[test]
fn test_app_state_remove_tab() {
    let mut state = AppState::default();

    let tab1 = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    let tab2 = TabState::new(
        "tab2".to_string(),
        "Tab 2".to_string(),
        PathBuf::from("/test2"),
    );

    state.add_tab(tab1);
    state.add_tab(tab2);

    // アクティブタブを削除
    let result = state.remove_tab("tab2");
    assert!(result.is_ok());
    assert_eq!(state.tabs.len(), 1);
    assert_eq!(state.active_tab_id, Some("tab1".to_string()));
    assert!(state.tabs[0].active);
}

#[test]
fn test_app_state_remove_inactive_tab() {
    let mut state = AppState::default();

    let tab1 = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    let tab2 = TabState::new(
        "tab2".to_string(),
        "Tab 2".to_string(),
        PathBuf::from("/test2"),
    );

    state.add_tab(tab1);
    state.add_tab(tab2);

    // 非アクティブタブを削除
    let result = state.remove_tab("tab1");
    assert!(result.is_ok());
    assert_eq!(state.tabs.len(), 1);
    assert_eq!(state.active_tab_id, Some("tab2".to_string()));
    assert!(state.tabs[0].active);
}

#[test]
fn test_app_state_remove_last_tab() {
    let mut state = AppState::default();

    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    state.add_tab(tab);

    let result = state.remove_tab("tab1");
    assert!(result.is_ok());
    assert!(state.tabs.is_empty());
    assert!(state.active_tab_id.is_none());
}

#[test]
fn test_app_state_remove_nonexistent_tab() {
    let mut state = AppState::default();

    let result = state.remove_tab("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_app_state_set_active_tab() {
    let mut state = AppState::default();

    let tab1 = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    let tab2 = TabState::new(
        "tab2".to_string(),
        "Tab 2".to_string(),
        PathBuf::from("/test2"),
    );

    state.add_tab(tab1);
    state.add_tab(tab2);

    // tab1をアクティブに設定
    let result = state.set_active_tab("tab1");
    assert!(result.is_ok());
    assert_eq!(state.active_tab_id, Some("tab1".to_string()));
    assert!(state.tabs[0].active);
    assert!(!state.tabs[1].active);
}

#[test]
fn test_app_state_get_active_tab() {
    let mut state = AppState::default();

    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    state.add_tab(tab);

    let active_tab = state.get_active_tab();
    assert!(active_tab.is_some());
    assert_eq!(active_tab.unwrap().id, "tab1");
}

#[test]
fn test_state_manager_new() {
    let manager = StateManager::new();
    let state = manager.get_state().unwrap();

    assert!(state.tabs.is_empty());
    assert!(state.active_tab_id.is_none());
}

#[test]
fn test_state_manager_with_state() {
    let mut initial_state = AppState::default();
    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );
    initial_state.add_tab(tab);

    let manager = StateManager::with_state(initial_state);
    let state = manager.get_state().unwrap();

    assert_eq!(state.tabs.len(), 1);
    assert_eq!(state.active_tab_id, Some("tab1".to_string()));
}

#[test]
fn test_state_manager_add_tab() {
    let manager = StateManager::new();
    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );

    let result = manager.add_tab(tab);
    assert!(result.is_ok());

    let state = manager.get_state().unwrap();
    assert_eq!(state.tabs.len(), 1);
    assert_eq!(state.active_tab_id, Some("tab1".to_string()));
}

#[test]
fn test_state_manager_remove_tab() {
    let manager = StateManager::new();
    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test1"),
    );

    manager.add_tab(tab).unwrap();
    let result = manager.remove_tab("tab1");
    assert!(result.is_ok());

    let state = manager.get_state().unwrap();
    assert!(state.tabs.is_empty());
    assert!(state.active_tab_id.is_none());
}

#[test]
fn test_state_manager_update_window_state() {
    let manager = StateManager::new();
    let window_state = WindowState {
        width: 1024.0,
        height: 768.0,
        x: Some(100.0),
        y: Some(50.0),
        maximized: true,
        minimized: false,
    };

    let result = manager.update_window_state(window_state.clone());
    assert!(result.is_ok());

    let state = manager.get_state().unwrap();
    assert_eq!(state.window.width, 1024.0);
    assert_eq!(state.window.height, 768.0);
    assert_eq!(state.window.x, Some(100.0));
    assert_eq!(state.window.y, Some(50.0));
    assert!(state.window.maximized);
    assert!(!state.window.minimized);
}

#[test]
fn test_state_manager_update_ui_state() {
    let manager = StateManager::new();
    let mut ui_state = UiState::default();
    ui_state.sidebar_visible = false;
    ui_state.theme = "dark".to_string();

    let result = manager.update_ui_state(ui_state.clone());
    assert!(result.is_ok());

    let state = manager.get_state().unwrap();
    assert!(!state.ui.sidebar_visible);
    assert_eq!(state.ui.theme, "dark");
}

#[test]
fn test_state_utils_generate_tab_id() {
    let id1 = state_utils::generate_tab_id();
    let id2 = state_utils::generate_tab_id();

    assert!(id1.starts_with("tab_"));
    assert!(id2.starts_with("tab_"));
    assert_ne!(id1, id2); // 異なるIDが生成される
}

#[test]
fn test_state_utils_generate_pane_id() {
    let id1 = state_utils::generate_pane_id();
    let id2 = state_utils::generate_pane_id();

    assert!(id1.starts_with("pane_"));
    assert!(id2.starts_with("pane_"));
    assert_ne!(id1, id2); // 異なるIDが生成される
}

#[test]
fn test_state_utils_create_default_tab() {
    let path = PathBuf::from("/home/user/documents");
    let tab = state_utils::create_default_tab(path.clone());

    assert!(!tab.id.is_empty());
    assert_eq!(tab.name, "documents");
    assert_eq!(tab.current_path, path);
    assert!(!tab.active);
}

#[test]
fn test_window_state_default() {
    let window_state = WindowState::default();

    assert_eq!(window_state.width, 1200.0);
    assert_eq!(window_state.height, 800.0);
    assert!(window_state.x.is_none());
    assert!(window_state.y.is_none());
    assert!(!window_state.maximized);
    assert!(!window_state.minimized);
}

#[test]
fn test_ui_state_default() {
    let ui_state = UiState::default();

    assert!(ui_state.sidebar_visible);
    assert!(ui_state.statusbar_visible);
    assert!(ui_state.toolbar_visible);
    assert_eq!(ui_state.theme, "default");
    assert!(ui_state.custom_properties.is_empty());
}

#[test]
fn test_tab_state_new() {
    let tab = TabState::new(
        "test_id".to_string(),
        "Test Tab".to_string(),
        PathBuf::from("/test/path"),
    );

    assert_eq!(tab.id, "test_id");
    assert_eq!(tab.name, "Test Tab");
    assert_eq!(tab.current_path, PathBuf::from("/test/path"));
    assert!(!tab.active);
}

#[test]
fn test_state_serialization() {
    let mut state = AppState::default();
    let tab = TabState::new(
        "tab1".to_string(),
        "Tab 1".to_string(),
        PathBuf::from("/test"),
    );
    state.add_tab(tab);

    // JSON シリアライゼーション
    let json = serde_json::to_string(&state);
    assert!(json.is_ok());

    // JSON デシリアライゼーション
    let json_str = json.unwrap();
    let deserialized: Result<AppState, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok());

    let restored_state = deserialized.unwrap();
    assert_eq!(restored_state.tabs.len(), 1);
    assert_eq!(restored_state.tabs[0].id, "tab1");
    assert_eq!(restored_state.active_tab_id, Some("tab1".to_string()));
}
