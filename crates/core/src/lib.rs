//! rust-explorer コア機能クレート
//!
//! アプリケーションの中核となるビジネスロジックを含みます。

#![allow(clippy::result_large_err)]

pub mod event;
pub mod filesystem;
pub mod state;

#[cfg(test)]
mod tests;

pub use event::{Event, EventManager};
pub use filesystem::{
    CachedFileSystemManager, FileEntry, FileInfo, FileSystemApi, FileSystemManager, FileType,
};
pub use state::{
    AppState, PanePosition, PaneSize, PaneState, PaneType, StateChangeEvent, StateManager,
    TabState, UiState, WindowState, state_utils,
};
