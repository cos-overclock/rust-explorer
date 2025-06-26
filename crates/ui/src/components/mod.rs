//! UI コンポーネント
//!
//! 再利用可能なUIコンポーネントを含みます。

pub mod error_dialog;
pub mod header;
pub mod main_content;
pub mod status_bar;

// 将来のコンポーネント用のモジュール宣言
// pub mod tabs;
// pub mod pane;
// pub mod file_list;

// 公開API
pub use error_dialog::{
    ErrorAction, ErrorActionType, ErrorDisplayInfo, ErrorDisplayManager, display_error_globally,
    error_dialog_component, error_display_area, global_error_manager,
};
pub use header::{HeaderConfig, default_header, header_component};
pub use main_content::{
    ContentType, MainContentConfig, default_main_content, main_content_component,
};
pub use status_bar::{
    StatusBarConfig, StatusInfo, StatusType, add_status_item, create_status_info,
    default_status_bar, file_explorer_status_bar, status_bar_component,
};
