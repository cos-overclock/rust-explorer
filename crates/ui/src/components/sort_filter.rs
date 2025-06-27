//! ソート・フィルタUIコンポーネント

use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, button, container, h_stack, label, text, text_input, v_stack};
use rust_explorer_core::{
    FileSortFilterManager, FilterCriteria, SortConfig, SortCriteria, SortDirection,
};
use std::sync::Arc;

/// ソート・フィルタツールバーの設定
#[derive(Debug, Clone)]
pub struct SortFilterConfig {
    /// ツールバーを表示するか
    pub show_toolbar: bool,
    /// 詳細フィルタを表示するか
    pub show_advanced_filters: bool,
}

impl Default for SortFilterConfig {
    fn default() -> Self {
        Self {
            show_toolbar: true,
            show_advanced_filters: false,
        }
    }
}

/// ソート・フィルタUIマネージャー
pub struct SortFilterUIManager {
    /// ソート・フィルタマネージャー
    manager: Arc<std::sync::Mutex<FileSortFilterManager>>,
    /// 現在のソート設定
    sort_config: RwSignal<SortConfig>,
    /// 現在のフィルタ条件
    filter_criteria: RwSignal<FilterCriteria>,
    /// 名前フィルタ入力
    name_filter_input: RwSignal<String>,
    /// 拡張子フィルタ入力
    extension_filter_input: RwSignal<String>,
    /// 最小サイズ入力
    min_size_input: RwSignal<String>,
    /// 最大サイズ入力
    max_size_input: RwSignal<String>,
    /// フィルタ変更通知コールバック
    on_filter_change: Option<Box<dyn Fn() + Send + Sync>>,
}

impl SortFilterUIManager {
    /// 新しいUIマネージャーを作成
    pub fn new() -> Self {
        let manager = Arc::new(std::sync::Mutex::new(FileSortFilterManager::new()));
        let initial_sort = manager.lock().unwrap().sort_config();
        let initial_filter = manager.lock().unwrap().filter_criteria().clone();

        Self {
            manager,
            sort_config: RwSignal::new(initial_sort),
            filter_criteria: RwSignal::new(initial_filter.clone()),
            name_filter_input: RwSignal::new(initial_filter.name_filter.unwrap_or_default()),
            extension_filter_input: RwSignal::new(
                initial_filter.extension_filter.unwrap_or_default(),
            ),
            min_size_input: RwSignal::new(
                initial_filter
                    .min_size
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            ),
            max_size_input: RwSignal::new(
                initial_filter
                    .max_size
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            ),
            on_filter_change: None,
        }
    }

    /// フィルタ変更通知コールバックを設定
    pub fn on_filter_change<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_filter_change = Some(Box::new(callback));
        self
    }

    /// ソート条件を変更
    pub fn set_sort_criteria(&self, criteria: SortCriteria) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.set_sort_criteria(criteria);
            self.sort_config.set(manager.sort_config());
            self.notify_change();
        }
    }

    /// ソート方向を切り替え
    pub fn toggle_sort_direction(&self) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.toggle_sort_direction();
            self.sort_config.set(manager.sort_config());
            self.notify_change();
        }
    }

    /// 隠しファイル表示を切り替え
    pub fn toggle_show_hidden(&self) {
        let mut criteria = self.filter_criteria.get();
        criteria.show_hidden = !criteria.show_hidden;
        self.update_filter_criteria(criteria);
    }

    /// フィルタ条件を更新
    fn update_filter_criteria(&self, criteria: FilterCriteria) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.update_filter_criteria(criteria.clone());
            self.filter_criteria.set(criteria);
            self.notify_change();
        }
    }

    /// 名前フィルタを更新
    pub fn update_name_filter(&self, filter: String) {
        self.name_filter_input.set(filter.clone());
        let mut criteria = self.filter_criteria.get();
        criteria.name_filter = if filter.is_empty() {
            None
        } else {
            Some(filter)
        };
        self.update_filter_criteria(criteria);
    }

    /// 拡張子フィルタを更新
    pub fn update_extension_filter(&self, filter: String) {
        self.extension_filter_input.set(filter.clone());
        let mut criteria = self.filter_criteria.get();
        criteria.extension_filter = if filter.is_empty() {
            None
        } else {
            Some(filter)
        };
        self.update_filter_criteria(criteria);
    }

    /// サイズフィルタを更新
    pub fn update_size_filter(&self, min_str: String, max_str: String) {
        self.min_size_input.set(min_str.clone());
        self.max_size_input.set(max_str.clone());

        let mut criteria = self.filter_criteria.get();
        criteria.min_size = min_str.parse().ok();
        criteria.max_size = max_str.parse().ok();
        self.update_filter_criteria(criteria);
    }

    /// フィルタをクリア
    pub fn clear_filters(&self) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.clear_filters();
            let criteria = manager.filter_criteria().clone();
            self.filter_criteria.set(criteria);
            self.name_filter_input.set(String::new());
            self.extension_filter_input.set(String::new());
            self.min_size_input.set(String::new());
            self.max_size_input.set(String::new());
            self.notify_change();
        }
    }

    /// フィルタが適用されているかチェック
    pub fn has_active_filters(&self) -> bool {
        if let Ok(manager) = self.manager.lock() {
            manager.has_active_filters()
        } else {
            false
        }
    }

    /// ファイルエントリを処理（ソート・フィルタ適用）
    pub fn process_entries(&self, entries: &mut Vec<rust_explorer_core::FileEntry>) {
        if let Ok(manager) = self.manager.lock() {
            manager.process_entries(entries);
        }
    }

    /// 変更通知
    fn notify_change(&self) {
        if let Some(callback) = &self.on_filter_change {
            callback();
        }
    }

    /// 現在のソート設定を取得
    pub fn current_sort_config(&self) -> SortConfig {
        self.sort_config.get()
    }

    /// 現在のフィルタ条件を取得
    pub fn current_filter_criteria(&self) -> FilterCriteria {
        self.filter_criteria.get()
    }
}

impl Default for SortFilterUIManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ソートツールバーを作成
pub fn sort_toolbar(manager: Arc<SortFilterUIManager>) -> impl IntoView {
    let sort_config = manager.sort_config;

    h_stack((
        // 名前でソート
        create_sort_button("名前", SortCriteria::Name, sort_config, manager.clone()),
        // サイズでソート
        create_sort_button("サイズ", SortCriteria::Size, sort_config, manager.clone()),
        // 更新日時でソート
        create_sort_button(
            "更新日時",
            SortCriteria::Modified,
            sort_config,
            manager.clone(),
        ),
        // 種類でソート
        create_sort_button("種類", SortCriteria::Type, sort_config, manager),
    ))
    .style(|s| {
        s.gap(8)
            .padding(8)
            .background(Color::rgb8(248, 249, 250))
            .border_radius(6)
            .border(1)
            .border_color(Color::rgb8(229, 231, 235))
    })
}

/// ソートボタンを作成
fn create_sort_button(
    label_text: &'static str,
    criteria: SortCriteria,
    sort_config: RwSignal<SortConfig>,
    manager: Arc<SortFilterUIManager>,
) -> impl IntoView {
    button(h_stack((
        text(label_text),
        // ソート方向アイコン
        label(move || {
            let config = sort_config.get();
            if config.criteria == criteria {
                match config.direction {
                    SortDirection::Ascending => " ▲",
                    SortDirection::Descending => " ▼",
                }
            } else {
                ""
            }
        })
        .style(|s| s.font_size(10)),
    )))
    .action(move || {
        manager.set_sort_criteria(criteria);
    })
    .style(move |s| {
        let config = sort_config.get();
        let is_active = config.criteria == criteria;

        s.padding_horiz(12)
            .padding_vert(6)
            .border_radius(4)
            .color(if is_active {
                Color::rgb8(59, 130, 246)
            } else {
                Color::rgb8(75, 85, 99)
            })
            .background(if is_active {
                Color::rgb8(239, 246, 255)
            } else {
                Color::rgb8(255, 255, 255)
            })
            .border(1)
            .border_color(if is_active {
                Color::rgb8(147, 197, 253)
            } else {
                Color::rgb8(209, 213, 219)
            })
            .cursor(floem::style::CursorStyle::Pointer)
            .hover(|s| {
                s.background(if is_active {
                    Color::rgb8(219, 234, 254)
                } else {
                    Color::rgb8(249, 250, 251)
                })
            })
    })
}

/// フィルタツールバーを作成
pub fn filter_toolbar(manager: Arc<SortFilterUIManager>) -> impl IntoView {
    let filter_criteria = manager.filter_criteria;
    let has_filters = move || {
        let criteria = filter_criteria.get();
        criteria.name_filter.is_some()
            || criteria.extension_filter.is_some()
            || criteria.min_size.is_some()
            || criteria.max_size.is_some()
    };

    v_stack((
        // 基本フィルタコントロール
        h_stack((
            // 隠しファイル表示切り替え
            button(h_stack((
                label(move || {
                    if filter_criteria.get().show_hidden {
                        "☑"
                    } else {
                        "☐"
                    }
                }),
                text(" 隠しファイルを表示"),
            )))
            .action({
                let manager = manager.clone();
                move || manager.toggle_show_hidden()
            })
            .style(|s| {
                s.padding_horiz(8)
                    .padding_vert(4)
                    .border_radius(4)
                    .cursor(floem::style::CursorStyle::Pointer)
            }),
            // フィルタクリアボタン
            button(text("フィルタをクリア"))
                .action({
                    let manager = manager.clone();
                    move || manager.clear_filters()
                })
                .style(move |s| {
                    s.padding_horiz(8)
                        .padding_vert(4)
                        .border_radius(4)
                        .color(if has_filters() {
                            Color::rgb8(239, 68, 68)
                        } else {
                            Color::rgb8(156, 163, 175)
                        })
                        .cursor(if has_filters() {
                            floem::style::CursorStyle::Pointer
                        } else {
                            floem::style::CursorStyle::Default
                        })
                })
                .disabled(move || !has_filters()),
        ))
        .style(|s| s.gap(12).items_center()),
        // 検索・フィルタ入力
        h_stack((
            // 名前フィルタ
            v_stack((
                label(|| "名前で検索").style(|s| s.font_size(12)),
                text_input(manager.name_filter_input)
                    .on_event_stop(floem::event::EventListener::KeyUp, {
                        let manager = manager.clone();
                        let input_signal = manager.name_filter_input;
                        move |_| manager.update_name_filter(input_signal.get())
                    })
                    .style(|s| {
                        s.width(150)
                            .padding(6)
                            .border(1)
                            .border_color(Color::rgb8(209, 213, 219))
                            .border_radius(4)
                    }),
            )),
            // 拡張子フィルタ
            v_stack((
                label(|| "拡張子").style(|s| s.font_size(12)),
                text_input(manager.extension_filter_input)
                    .on_event_stop(floem::event::EventListener::KeyUp, {
                        let manager = manager.clone();
                        let input_signal = manager.extension_filter_input;
                        move |_| manager.update_extension_filter(input_signal.get())
                    })
                    .style(|s| {
                        s.width(100)
                            .padding(6)
                            .border(1)
                            .border_color(Color::rgb8(209, 213, 219))
                            .border_radius(4)
                    }),
            )),
            // サイズフィルタ
            v_stack((
                label(|| "サイズ (バイト)").style(|s| s.font_size(12)),
                h_stack((
                    text_input(manager.min_size_input)
                        .on_event_stop(floem::event::EventListener::KeyUp, {
                            let manager = manager.clone();
                            let min_input = manager.min_size_input;
                            let max_input = manager.max_size_input;
                            move |_| manager.update_size_filter(min_input.get(), max_input.get())
                        })
                        .style(|s| {
                            s.width(80)
                                .padding(6)
                                .border(1)
                                .border_color(Color::rgb8(209, 213, 219))
                                .border_radius(4)
                        }),
                    text(" - "),
                    text_input(manager.max_size_input)
                        .on_event_stop(floem::event::EventListener::KeyUp, {
                            let manager = manager.clone();
                            let min_input = manager.min_size_input;
                            let max_input = manager.max_size_input;
                            move |_| manager.update_size_filter(min_input.get(), max_input.get())
                        })
                        .style(|s| {
                            s.width(80)
                                .padding(6)
                                .border(1)
                                .border_color(Color::rgb8(209, 213, 219))
                                .border_radius(4)
                        }),
                ))
                .style(|s| s.items_center().gap(4)),
            )),
        ))
        .style(|s| s.gap(16).items_end()),
    ))
    .style(|s| {
        s.gap(12)
            .padding(12)
            .background(Color::rgb8(248, 249, 250))
            .border_radius(6)
            .border(1)
            .border_color(Color::rgb8(229, 231, 235))
    })
}

/// ソート・フィルタツールバー全体を作成
pub fn sort_filter_toolbar(
    manager: Arc<SortFilterUIManager>,
    config: SortFilterConfig,
) -> impl IntoView {
    if !config.show_toolbar {
        return container(text(""))
            .style(|s| s.display(floem::style::Display::None))
            .into_any();
    }

    v_stack((
        // ソートツールバー
        container(sort_toolbar(manager.clone())).style(|s| s.margin_bottom(8)),
        // フィルタツールバー
        if config.show_advanced_filters {
            container(filter_toolbar(manager)).into_any()
        } else {
            container(text(""))
                .style(|s| s.display(floem::style::Display::None))
                .into_any()
        },
    ))
    .style(|s| s.gap(0))
    .into_any()
}

/// 簡易フィルタバーを作成（名前検索のみ）
pub fn simple_filter_bar(manager: Arc<SortFilterUIManager>) -> impl IntoView {
    let filter_criteria = manager.filter_criteria;

    h_stack((
        // 隠しファイル表示切り替え
        button(h_stack((
            label(move || {
                if filter_criteria.get().show_hidden {
                    "☑"
                } else {
                    "☐"
                }
            }),
            text(" 隠しファイル"),
        )))
        .action({
            let manager = manager.clone();
            move || manager.toggle_show_hidden()
        })
        .style(|s| {
            s.padding_horiz(8)
                .padding_vert(4)
                .border_radius(4)
                .cursor(floem::style::CursorStyle::Pointer)
        }),
        // 名前検索
        text_input(manager.name_filter_input)
            .on_event_stop(floem::event::EventListener::KeyUp, {
                let manager = manager.clone();
                let input_signal = manager.name_filter_input;
                move |_| manager.update_name_filter(input_signal.get())
            })
            .style(|s| {
                s.width(200)
                    .padding(6)
                    .border(1)
                    .border_color(Color::rgb8(209, 213, 219))
                    .border_radius(4)
            }),
    ))
    .style(|s| {
        s.gap(12)
            .items_center()
            .padding(8)
            .background(Color::rgb8(248, 249, 250))
            .border_radius(6)
            .border(1)
            .border_color(Color::rgb8(229, 231, 235))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_filter_config_default() {
        let config = SortFilterConfig::default();
        assert!(config.show_toolbar);
        assert!(!config.show_advanced_filters);
    }

    #[test]
    fn test_sort_filter_ui_manager_creation() {
        let manager = SortFilterUIManager::new();

        // 初期状態のテスト
        let sort_config = manager.current_sort_config();
        assert_eq!(sort_config.criteria, SortCriteria::Name);
        assert_eq!(sort_config.direction, SortDirection::Ascending);

        let filter_criteria = manager.current_filter_criteria();
        assert!(!filter_criteria.show_hidden);
        assert!(filter_criteria.name_filter.is_none());
    }

    #[test]
    fn test_set_sort_criteria() {
        let manager = SortFilterUIManager::new();

        // 初期状態は Name, Ascending
        let initial_config = manager.current_sort_config();
        assert_eq!(initial_config.criteria, SortCriteria::Name);
        assert_eq!(initial_config.direction, SortDirection::Ascending);

        // Sizeに変更
        manager.set_sort_criteria(SortCriteria::Size);
        let new_config = manager.current_sort_config();
        assert_eq!(new_config.criteria, SortCriteria::Size);
        assert_eq!(new_config.direction, SortDirection::Ascending);

        // 同じ条件を設定すると方向が変わる
        manager.set_sort_criteria(SortCriteria::Size);
        let toggled_config = manager.current_sort_config();
        assert_eq!(toggled_config.criteria, SortCriteria::Size);
        assert_eq!(toggled_config.direction, SortDirection::Descending);
    }

    #[test]
    fn test_toggle_show_hidden() {
        let manager = SortFilterUIManager::new();

        // 初期状態は隠しファイル非表示
        assert!(!manager.current_filter_criteria().show_hidden);

        // 切り替え
        manager.toggle_show_hidden();
        assert!(manager.current_filter_criteria().show_hidden);

        // 再度切り替え
        manager.toggle_show_hidden();
        assert!(!manager.current_filter_criteria().show_hidden);
    }

    #[test]
    fn test_update_name_filter() {
        let manager = SortFilterUIManager::new();

        // 初期状態はフィルタなし
        assert!(manager.current_filter_criteria().name_filter.is_none());

        // フィルタを設定
        manager.update_name_filter("test".to_string());
        assert_eq!(
            manager.current_filter_criteria().name_filter,
            Some("test".to_string())
        );

        // 空文字列でクリア
        manager.update_name_filter("".to_string());
        assert!(manager.current_filter_criteria().name_filter.is_none());
    }

    #[test]
    fn test_clear_filters() {
        let manager = SortFilterUIManager::new();

        // フィルタを設定
        manager.update_name_filter("test".to_string());
        manager.update_extension_filter("txt".to_string());
        manager.toggle_show_hidden();

        // フィルタが設定されていることを確認
        assert!(manager.has_active_filters());

        // フィルタをクリア
        manager.clear_filters();

        // フィルタがクリアされていることを確認
        let criteria = manager.current_filter_criteria();
        assert!(criteria.name_filter.is_none());
        assert!(criteria.extension_filter.is_none());
        assert!(!criteria.show_hidden);
        assert!(!manager.has_active_filters());
    }
}
