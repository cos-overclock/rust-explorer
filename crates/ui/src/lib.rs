//! rust-explorer UI クレート
//!
//! ユーザーインターフェース関連のコンポーネントとロジックを含みます。

#![allow(clippy::result_large_err)]

pub mod app;
pub mod components;
pub mod layout;
pub mod window;

pub use app::App;
pub use layout::{LayoutConfig, ResponsiveLayoutManager, ScreenSizeCategory};
pub use window::{MainWindow, WindowState};
