//! rust-explorer UI クレート
//!
//! ユーザーインターフェース関連のコンポーネントとロジックを含みます。

#![allow(clippy::result_large_err)]

pub mod app;
pub mod components;
pub mod layout;
pub mod state_integration;
pub mod window;

pub use app::App;
pub use layout::{LayoutConfig, ResponsiveLayoutManager, ScreenSizeCategory};
pub use state_integration::{
    ReactiveStateManager, ReactiveTabState, ReactiveUiState, ReactiveWindowState, reactive_utils,
};
pub use window::{MainWindow, WindowState};
