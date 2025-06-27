//! UI コンポーネント
//!
//! 再利用可能なUIコンポーネントを含みます。

pub mod breadcrumb;
pub mod error_dialog;
pub mod file_item;
pub mod file_list;
pub mod file_navigation;
pub mod header;
pub mod main_content;
pub mod sort_filter;
pub mod status_bar;

// 将来のコンポーネント用のモジュール宣言
// pub mod tabs;
// pub mod pane;

// 公開API
pub use breadcrumb::{
    BreadcrumbConfig, BreadcrumbItem, BreadcrumbNavigation, breadcrumb_navigation, breadcrumb_view,
    default_breadcrumb_navigation,
};
pub use error_dialog::{
    ErrorAction, ErrorActionType, ErrorDisplayInfo, ErrorDisplayManager, display_error_globally,
    error_dialog_component, error_display_area, global_error_manager,
};
pub use file_item::{file_item_component, file_item_view, file_item_with_double_click};
pub use file_list::{
    FileListConfig, FileListState, FileListView, default_file_list_view, file_list_view,
    file_list_view_component,
};
pub use file_navigation::{
    FileNavigationConfig, FileNavigationManager, FileNavigationState, navigation_helpers,
    with_double_click_handler,
};
pub use header::{HeaderConfig, default_header, header_component};
pub use main_content::{
    ContentType, MainContentConfig, default_main_content, main_content_component,
};
pub use sort_filter::{
    SortFilterConfig, SortFilterUIManager, filter_toolbar, simple_filter_bar, sort_filter_toolbar,
    sort_toolbar,
};
pub use status_bar::{
    StatusBarConfig, StatusInfo, StatusType, add_status_item, create_status_info,
    default_status_bar, file_explorer_status_bar, status_bar_component,
};
