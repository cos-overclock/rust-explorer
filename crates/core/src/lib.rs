//! rust-explorer コア機能クレート
//!
//! アプリケーションの中核となるビジネスロジックを含みます。

pub mod event;
pub mod filesystem;

pub use event::{Event, EventManager};
pub use filesystem::FileSystemManager;
