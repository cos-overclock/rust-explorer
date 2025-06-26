//! アプリケーション状態管理
//!
//! アプリケーション全体の状態を管理するシステム

use rust_explorer_utils::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// ウィンドウ状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// ウィンドウ幅
    pub width: f64,
    /// ウィンドウ高さ
    pub height: f64,
    /// ウィンドウX座標
    pub x: Option<f64>,
    /// ウィンドウY座標
    pub y: Option<f64>,
    /// 最大化状態
    pub maximized: bool,
    /// 最小化状態
    pub minimized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
            x: None,
            y: None,
            maximized: false,
            minimized: false,
        }
    }
}

/// タブ状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabState {
    /// タブID
    pub id: String,
    /// タブ名
    pub name: String,
    /// 現在のパス
    pub current_path: PathBuf,
    /// アクティブフラグ
    pub active: bool,
    /// タブの作成時刻
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TabState {
    pub fn new(id: String, name: String, current_path: PathBuf) -> Self {
        Self {
            id,
            name,
            current_path,
            active: false,
            created_at: chrono::Utc::now(),
        }
    }
}

/// ペイン状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaneState {
    /// ペインID
    pub id: String,
    /// ペインタイプ
    pub pane_type: PaneType,
    /// 位置
    pub position: PanePosition,
    /// サイズ
    pub size: PaneSize,
    /// 表示状態
    pub visible: bool,
}

/// ペインタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaneType {
    /// ファイルリスト
    FileList,
    /// プレビュー
    Preview,
    /// プロパティ
    Properties,
    /// ログ
    Log,
}

/// ペイン位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanePosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

/// ペインサイズ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaneSize {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub flex: Option<f64>,
}

/// UI状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiState {
    /// サイドバー表示状態
    pub sidebar_visible: bool,
    /// ステータスバー表示状態
    pub statusbar_visible: bool,
    /// ツールバー表示状態
    pub toolbar_visible: bool,
    /// テーマ
    pub theme: String,
    /// カスタムプロパティ
    pub custom_properties: HashMap<String, String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            sidebar_visible: true,
            statusbar_visible: true,
            toolbar_visible: true,
            theme: "default".to_string(),
            custom_properties: HashMap::new(),
        }
    }
}

/// アプリケーション状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// ウィンドウ状態
    pub window: WindowState,
    /// タブ一覧
    pub tabs: Vec<TabState>,
    /// ペイン一覧
    pub panes: Vec<PaneState>,
    /// UI状態
    pub ui: UiState,
    /// 現在のアクティブタブID
    pub active_tab_id: Option<String>,
    /// 最後に保存された時刻
    pub last_saved: chrono::DateTime<chrono::Utc>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            window: WindowState::default(),
            tabs: Vec::new(),
            panes: Vec::new(),
            ui: UiState::default(),
            active_tab_id: None,
            last_saved: chrono::Utc::now(),
        }
    }
}

impl AppState {
    /// 新しいタブを追加
    pub fn add_tab(&mut self, tab: TabState) {
        // 他のタブを非アクティブにする
        for existing_tab in &mut self.tabs {
            existing_tab.active = false;
        }

        let tab_id = tab.id.clone();
        let mut new_tab = tab;
        new_tab.active = true;

        self.tabs.push(new_tab);
        self.active_tab_id = Some(tab_id);
        self.last_saved = chrono::Utc::now();
    }

    /// タブを削除
    pub fn remove_tab(&mut self, tab_id: &str) -> Result<(), AppError> {
        let tab_index = self
            .tabs
            .iter()
            .position(|t| t.id == tab_id)
            .ok_or_else(|| AppError::Internal(format!("Tab not found: {}", tab_id)))?;

        let _removed_tab = self.tabs.remove(tab_index);

        // アクティブタブが削除された場合、新しいアクティブタブを設定
        if self.active_tab_id.as_deref() == Some(tab_id) {
            if !self.tabs.is_empty() {
                // 削除されたタブの前のタブをアクティブにする、なければ次のタブ
                let new_active_index = if tab_index > 0 { tab_index - 1 } else { 0 };
                if let Some(new_active_tab) = self.tabs.get_mut(new_active_index) {
                    new_active_tab.active = true;
                    self.active_tab_id = Some(new_active_tab.id.clone());
                }
            } else {
                self.active_tab_id = None;
            }
        }

        self.last_saved = chrono::Utc::now();
        Ok(())
    }

    /// アクティブタブを変更
    pub fn set_active_tab(&mut self, tab_id: &str) -> Result<(), AppError> {
        // すべてのタブを非アクティブにする
        for tab in &mut self.tabs {
            tab.active = false;
        }

        // 指定されたタブをアクティブにする
        let tab = self
            .tabs
            .iter_mut()
            .find(|t| t.id == tab_id)
            .ok_or_else(|| AppError::Internal(format!("Tab not found: {}", tab_id)))?;

        tab.active = true;
        self.active_tab_id = Some(tab_id.to_string());
        self.last_saved = chrono::Utc::now();
        Ok(())
    }

    /// アクティブタブを取得
    pub fn get_active_tab(&self) -> Option<&TabState> {
        self.active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| t.id == *id))
    }

    /// アクティブタブを可変参照で取得
    pub fn get_active_tab_mut(&mut self) -> Option<&mut TabState> {
        self.active_tab_id
            .clone()
            .and_then(move |id| self.tabs.iter_mut().find(|t| t.id == *id))
    }

    /// ペインを追加
    pub fn add_pane(&mut self, pane: PaneState) {
        self.panes.push(pane);
        self.last_saved = chrono::Utc::now();
    }

    /// ペインを削除
    pub fn remove_pane(&mut self, pane_id: &str) -> Result<(), AppError> {
        let pane_index = self
            .panes
            .iter()
            .position(|p| p.id == pane_id)
            .ok_or_else(|| AppError::Internal(format!("Pane not found: {}", pane_id)))?;

        self.panes.remove(pane_index);
        self.last_saved = chrono::Utc::now();
        Ok(())
    }
}

/// 状態変更イベント
#[derive(Debug, Clone)]
pub enum StateChangeEvent {
    /// ウィンドウ状態変更
    WindowChanged(WindowState),
    /// タブ追加
    TabAdded(TabState),
    /// タブ削除
    TabRemoved(String),
    /// アクティブタブ変更
    ActiveTabChanged(String),
    /// ペイン追加
    PaneAdded(PaneState),
    /// ペイン削除
    PaneRemoved(String),
    /// UI状態変更
    UiStateChanged(UiState),
}

/// 状態管理マネージャー
pub struct StateManager {
    /// 現在の状態
    state: Arc<RwLock<AppState>>,
    /// 状態変更イベントのコールバック
    change_callbacks: Arc<RwLock<Vec<StateChangeCallback>>>,
}

/// 状態変更コールバックの型エイリアス
type StateChangeCallback = Box<dyn Fn(&StateChangeEvent) + Send + Sync>;

impl StateManager {
    /// 新しい状態管理マネージャーを作成
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::default())),
            change_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 初期状態から状態管理マネージャーを作成
    pub fn with_state(initial_state: AppState) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial_state)),
            change_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 現在の状態を取得
    pub fn get_state(&self) -> Result<AppState, AppError> {
        self.state
            .read()
            .map_err(|e| AppError::Internal(format!("Failed to read state: {}", e)))
            .map(|state| state.clone())
    }

    /// 状態を更新
    pub fn update_state<F>(&self, updater: F) -> Result<(), AppError>
    where
        F: FnOnce(&mut AppState) -> Result<Option<StateChangeEvent>, AppError>,
    {
        let mut state = self
            .state
            .write()
            .map_err(|e| AppError::Internal(format!("Failed to write state: {}", e)))?;

        if let Some(event) = updater(&mut state)? {
            // イベントをトリガー
            self.trigger_event(event);
        }

        Ok(())
    }

    /// ウィンドウ状態を更新
    pub fn update_window_state(&self, window_state: WindowState) -> Result<(), AppError> {
        self.update_state(|state| {
            state.window = window_state.clone();
            state.last_saved = chrono::Utc::now();
            Ok(Some(StateChangeEvent::WindowChanged(window_state)))
        })
    }

    /// タブを追加
    pub fn add_tab(&self, tab: TabState) -> Result<(), AppError> {
        let tab_clone = tab.clone();
        self.update_state(|state| {
            state.add_tab(tab);
            Ok(Some(StateChangeEvent::TabAdded(tab_clone)))
        })
    }

    /// タブを削除
    pub fn remove_tab(&self, tab_id: &str) -> Result<(), AppError> {
        let tab_id = tab_id.to_string();
        self.update_state(|state| {
            state.remove_tab(&tab_id)?;
            Ok(Some(StateChangeEvent::TabRemoved(tab_id)))
        })
    }

    /// アクティブタブを設定
    pub fn set_active_tab(&self, tab_id: &str) -> Result<(), AppError> {
        let tab_id = tab_id.to_string();
        self.update_state(|state| {
            state.set_active_tab(&tab_id)?;
            Ok(Some(StateChangeEvent::ActiveTabChanged(tab_id)))
        })
    }

    /// UI状態を更新
    pub fn update_ui_state(&self, ui_state: UiState) -> Result<(), AppError> {
        self.update_state(|state| {
            state.ui = ui_state.clone();
            state.last_saved = chrono::Utc::now();
            Ok(Some(StateChangeEvent::UiStateChanged(ui_state)))
        })
    }

    /// 状態変更イベントのコールバックを登録
    pub fn on_state_change<F>(&self, callback: F) -> Result<(), AppError>
    where
        F: Fn(&StateChangeEvent) + Send + Sync + 'static,
    {
        let mut callbacks = self
            .change_callbacks
            .write()
            .map_err(|e| AppError::Internal(format!("Failed to write callbacks: {}", e)))?;

        callbacks.push(Box::new(callback));
        Ok(())
    }

    /// 状態変更イベントをトリガー
    fn trigger_event(&self, event: StateChangeEvent) {
        if let Ok(callbacks) = self.change_callbacks.read() {
            for callback in callbacks.iter() {
                callback(&event);
            }
        }
    }

    /// デバッグ用: 状態を文字列として出力
    pub fn debug_state(&self) -> Result<String, AppError> {
        let state = self.get_state()?;
        Ok(format!("{:#?}", state))
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StateManager {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            change_callbacks: Arc::clone(&self.change_callbacks),
        }
    }
}

/// 状態管理ユーティリティ
pub mod state_utils {
    use super::*;

    /// タブIDを生成
    pub fn generate_tab_id() -> String {
        format!(
            "tab_{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        )
    }

    /// ペインIDを生成
    pub fn generate_pane_id() -> String {
        format!(
            "pane_{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        )
    }

    /// デフォルトタブを作成
    pub fn create_default_tab(path: PathBuf) -> TabState {
        let id = generate_tab_id();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "New Tab".to_string());

        TabState::new(id, name, path)
    }

    /// デフォルトペインを作成
    pub fn create_default_pane(pane_type: PaneType, position: PanePosition) -> PaneState {
        PaneState {
            id: generate_pane_id(),
            pane_type,
            position,
            size: PaneSize {
                width: None,
                height: None,
                flex: Some(1.0),
            },
            visible: true,
        }
    }
}
