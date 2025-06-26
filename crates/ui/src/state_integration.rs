//! floem RwSignalを使ったUI状態統合

use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use rust_explorer_core::{
    AppState, StateManager, TabState, UiState, WindowState,
};
use rust_explorer_utils::AppError;

/// UIとfloemの状態を結合するリアクティブラッパー
#[derive(Clone)]
pub struct ReactiveStateManager {
    /// 内部の状態管理マネージャー
    state_manager: StateManager,
    /// floemのリアクティブな状態
    reactive_state: RwSignal<AppState>,
}

impl ReactiveStateManager {
    /// 新しいリアクティブ状態管理マネージャーを作成
    pub fn new(initial_state: AppState) -> Self {
        let state_manager = StateManager::with_state(initial_state.clone());
        let reactive_state = RwSignal::new(initial_state);

        let manager = Self {
            state_manager,
            reactive_state,
        };

        // 状態変更イベントをリアクティブ状態に反映
        manager.setup_state_sync();

        manager
    }

    /// デフォルト状態でマネージャーを作成
    pub fn with_default() -> Self {
        Self::new(AppState::default())
    }

    /// floemのリアクティブ状態を取得
    pub fn reactive_state(&self) -> RwSignal<AppState> {
        self.reactive_state
    }

    /// 内部の状態管理マネージャーを取得
    pub fn state_manager(&self) -> &StateManager {
        &self.state_manager
    }

    /// ウィンドウ状態を更新
    pub fn update_window_state(&self, window_state: WindowState) -> Result<(), AppError> {
        self.state_manager.update_window_state(window_state)
    }

    /// タブを追加
    pub fn add_tab(&self, tab: TabState) -> Result<(), AppError> {
        self.state_manager.add_tab(tab)
    }

    /// タブを削除
    pub fn remove_tab(&self, tab_id: &str) -> Result<(), AppError> {
        self.state_manager.remove_tab(tab_id)
    }

    /// アクティブタブを設定
    pub fn set_active_tab(&self, tab_id: &str) -> Result<(), AppError> {
        self.state_manager.set_active_tab(tab_id)
    }

    /// UI状態を更新
    pub fn update_ui_state(&self, ui_state: UiState) -> Result<(), AppError> {
        self.state_manager.update_ui_state(ui_state)
    }

    /// 状態変更イベントの同期設定
    fn setup_state_sync(&self) {
        let _reactive_state = self.reactive_state;

        if let Err(e) = self.state_manager.on_state_change(move |_event| {
            // 状態が変更されたときにリアクティブ状態を更新
            // 注意: ここでstate_managerから最新の状態を取得する必要がある
            // 実装上の制約で直接アクセスできないため、別の方法を検討
        }) {
            eprintln!("Failed to setup state sync: {}", e);
        }
    }
}

/// ウィンドウ状態のリアクティブラッパー
#[derive(Clone)]
pub struct ReactiveWindowState {
    signal: RwSignal<WindowState>,
}

impl ReactiveWindowState {
    pub fn new(initial_state: WindowState) -> Self {
        Self {
            signal: RwSignal::new(initial_state),
        }
    }

    pub fn signal(&self) -> RwSignal<WindowState> {
        self.signal
    }

    pub fn get(&self) -> WindowState {
        self.signal.get()
    }

    pub fn update_size(&self, width: f64, height: f64) {
        self.signal.update(|state| {
            state.width = width;
            state.height = height;
        });
    }

    pub fn update_position(&self, x: f64, y: f64) {
        self.signal.update(|state| {
            state.x = Some(x);
            state.y = Some(y);
        });
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.signal.update(|state| {
            state.maximized = maximized;
        });
    }

    pub fn set_minimized(&self, minimized: bool) {
        self.signal.update(|state| {
            state.minimized = minimized;
        });
    }
}

/// タブ状態のリアクティブラッパー
#[derive(Clone)]
pub struct ReactiveTabState {
    signal: RwSignal<Vec<TabState>>,
    active_tab_id: RwSignal<Option<String>>,
}

impl ReactiveTabState {
    pub fn new(initial_tabs: Vec<TabState>, active_tab_id: Option<String>) -> Self {
        Self {
            signal: RwSignal::new(initial_tabs),
            active_tab_id: RwSignal::new(active_tab_id),
        }
    }

    pub fn tabs_signal(&self) -> RwSignal<Vec<TabState>> {
        self.signal
    }

    pub fn active_tab_id_signal(&self) -> RwSignal<Option<String>> {
        self.active_tab_id
    }

    pub fn get_tabs(&self) -> Vec<TabState> {
        self.signal.get()
    }

    pub fn get_active_tab_id(&self) -> Option<String> {
        self.active_tab_id.get()
    }

    pub fn add_tab(&self, tab: TabState) {
        let tab_id = tab.id.clone();

        self.signal.update(|tabs| {
            // 他のタブを非アクティブにする
            for existing_tab in tabs.iter_mut() {
                existing_tab.active = false;
            }

            let mut new_tab = tab;
            new_tab.active = true;
            tabs.push(new_tab);
        });

        self.active_tab_id.set(Some(tab_id));
    }

    pub fn remove_tab(&self, tab_id: &str) {
        let tab_id = tab_id.to_string();
        let current_active = self.active_tab_id.get();

        self.signal.update(|tabs| {
            if let Some(index) = tabs.iter().position(|t| t.id == tab_id) {
                tabs.remove(index);

                // アクティブタブが削除された場合、新しいアクティブタブを設定
                if current_active.as_ref() == Some(&tab_id) && !tabs.is_empty() {
                    let new_active_index = if index > 0 { index - 1 } else { 0 };
                    if let Some(new_active_tab) = tabs.get_mut(new_active_index) {
                        new_active_tab.active = true;
                    }
                }
            }
        });

        // アクティブタブIDを更新
        if current_active.as_ref() == Some(&tab_id) {
            let new_active_id = self
                .signal
                .get()
                .iter()
                .find(|t| t.active)
                .map(|t| t.id.clone());
            self.active_tab_id.set(new_active_id);
        }
    }

    pub fn set_active_tab(&self, tab_id: &str) {
        let tab_id = tab_id.to_string();

        self.signal.update(|tabs| {
            // すべてのタブを非アクティブにする
            for tab in tabs.iter_mut() {
                tab.active = false;
            }

            // 指定されたタブをアクティブにする
            if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.active = true;
            }
        });

        self.active_tab_id.set(Some(tab_id));
    }
}

/// UI状態のリアクティブラッパー
#[derive(Clone)]
pub struct ReactiveUiState {
    signal: RwSignal<UiState>,
}

impl ReactiveUiState {
    pub fn new(initial_state: UiState) -> Self {
        Self {
            signal: RwSignal::new(initial_state),
        }
    }

    pub fn signal(&self) -> RwSignal<UiState> {
        self.signal
    }

    pub fn get(&self) -> UiState {
        self.signal.get()
    }

    pub fn toggle_sidebar(&self) {
        self.signal.update(|state| {
            state.sidebar_visible = !state.sidebar_visible;
        });
    }

    pub fn toggle_statusbar(&self) {
        self.signal.update(|state| {
            state.statusbar_visible = !state.statusbar_visible;
        });
    }

    pub fn toggle_toolbar(&self) {
        self.signal.update(|state| {
            state.toolbar_visible = !state.toolbar_visible;
        });
    }

    pub fn set_theme(&self, theme: String) {
        self.signal.update(|state| {
            state.theme = theme;
        });
    }

    pub fn set_custom_property(&self, key: String, value: String) {
        self.signal.update(|state| {
            state.custom_properties.insert(key, value);
        });
    }

    pub fn remove_custom_property(&self, key: &str) {
        self.signal.update(|state| {
            state.custom_properties.remove(key);
        });
    }
}

/// リアクティブ状態統合のためのコンビニエンス関数
pub mod reactive_utils {
    use super::*;
    use rust_explorer_core::state_utils;
    use std::path::PathBuf;

    /// デフォルトタブでリアクティブタブ状態を作成
    pub fn create_default_reactive_tabs(initial_path: PathBuf) -> ReactiveTabState {
        let default_tab = state_utils::create_default_tab(initial_path);
        let tabs = vec![default_tab.clone()];
        let active_tab_id = Some(default_tab.id);

        ReactiveTabState::new(tabs, active_tab_id)
    }

    /// デフォルトのリアクティブ状態管理マネージャーを作成
    pub fn create_default_reactive_manager() -> ReactiveStateManager {
        let mut initial_state = AppState::default();

        // デフォルトタブを追加
        let default_tab = state_utils::create_default_tab(
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
        );
        initial_state.add_tab(default_tab);

        ReactiveStateManager::new(initial_state)
    }

    /// 状態をリアクティブコンポーネントに分解
    pub fn decompose_reactive_state(
        state: &AppState,
    ) -> (ReactiveWindowState, ReactiveTabState, ReactiveUiState) {
        let window_state = ReactiveWindowState::new(state.window.clone());
        let tab_state = ReactiveTabState::new(state.tabs.clone(), state.active_tab_id.clone());
        let ui_state = ReactiveUiState::new(state.ui.clone());

        (window_state, tab_state, ui_state)
    }
}
