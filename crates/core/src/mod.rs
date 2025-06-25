//! コア機能モジュール
//! 
//! アプリケーションの中核となるビジネスロジックを含みます。

pub mod filesystem;
pub mod event;

pub use filesystem::FileSystemManager;
pub use event::EventManager;