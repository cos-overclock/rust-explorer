//! エラーダイアログコンポーネント
//!
//! ユーザー向けエラーメッセージ表示システムを提供します。

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::Position;
use floem::text::Weight;
use rust_explorer_utils::{AppError, ErrorCategory, ErrorSeverity};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// エラーダイアログの表示情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDisplayInfo {
    /// エラーID（重複排除用）
    pub id: String,
    /// 表示タイトル
    pub title: String,
    /// ユーザー向けメッセージ
    pub message: String,
    /// 詳細情報（技術者向け）
    pub details: Option<String>,
    /// エラーの重要度
    pub severity: ErrorSeverity,
    /// エラーカテゴリ
    pub category: ErrorCategory,
    /// 表示時刻
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 自動で消すかどうか
    pub auto_dismiss: bool,
    /// 自動消去までの秒数
    pub auto_dismiss_seconds: u32,
    /// アクションボタン
    pub actions: Vec<ErrorAction>,
}

/// エラーアクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAction {
    /// アクションID
    pub id: String,
    /// ボタンテキスト
    pub label: String,
    /// アクションタイプ
    pub action_type: ErrorActionType,
}

/// エラーアクションタイプ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorActionType {
    /// 閉じる
    Dismiss,
    /// 再試行
    Retry,
    /// 詳細表示
    ShowDetails,
    /// ログを開く
    OpenLogs,
    /// 設定を開く
    OpenSettings,
    /// サポートに連絡
    ContactSupport,
    /// カスタムアクション
    Custom(String),
}

impl ErrorDisplayInfo {
    /// AppErrorから表示情報を作成
    pub fn from_app_error(error: &AppError) -> Self {
        let severity = error.severity();
        let category = error.category();
        let user_message = error.user_message();

        let (title, auto_dismiss, auto_dismiss_seconds) = match severity {
            ErrorSeverity::Fatal => ("重大なエラー".to_string(), false, 0),
            ErrorSeverity::Critical => ("重要なエラー".to_string(), false, 0),
            ErrorSeverity::Error => ("エラー".to_string(), false, 0),
            ErrorSeverity::Warning => ("警告".to_string(), true, 10),
            ErrorSeverity::Info => ("情報".to_string(), true, 5),
        };

        let actions = Self::create_default_actions(&severity, &category);

        Self {
            id: Self::generate_error_id(error),
            title,
            message: user_message,
            details: Some(error.to_string()),
            severity,
            category,
            timestamp: chrono::Utc::now(),
            auto_dismiss,
            auto_dismiss_seconds,
            actions,
        }
    }

    /// エラーIDを生成
    fn generate_error_id(error: &AppError) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        error.to_string().hash(&mut hasher);
        error.category().hash(&mut hasher);
        format!("error_{:x}", hasher.finish())
    }

    /// デフォルトアクションを作成
    fn create_default_actions(
        severity: &ErrorSeverity,
        category: &ErrorCategory,
    ) -> Vec<ErrorAction> {
        let mut actions = vec![ErrorAction {
            id: "dismiss".to_string(),
            label: "閉じる".to_string(),
            action_type: ErrorActionType::Dismiss,
        }];

        // 重要度に応じてアクションを追加
        match severity {
            ErrorSeverity::Fatal | ErrorSeverity::Critical => {
                actions.push(ErrorAction {
                    id: "details".to_string(),
                    label: "詳細".to_string(),
                    action_type: ErrorActionType::ShowDetails,
                });
                actions.push(ErrorAction {
                    id: "logs".to_string(),
                    label: "ログを確認".to_string(),
                    action_type: ErrorActionType::OpenLogs,
                });
            }
            ErrorSeverity::Error => {
                actions.push(ErrorAction {
                    id: "retry".to_string(),
                    label: "再試行".to_string(),
                    action_type: ErrorActionType::Retry,
                });
            }
            _ => {}
        }

        // カテゴリに応じてアクションを追加
        match category {
            ErrorCategory::Configuration => {
                actions.push(ErrorAction {
                    id: "settings".to_string(),
                    label: "設定を開く".to_string(),
                    action_type: ErrorActionType::OpenSettings,
                });
            }
            ErrorCategory::FileSystem => {
                actions.push(ErrorAction {
                    id: "retry".to_string(),
                    label: "再試行".to_string(),
                    action_type: ErrorActionType::Retry,
                });
            }
            _ => {}
        }

        actions
    }

    /// 重要度に応じた色を取得
    pub fn get_severity_color(&self) -> Color {
        match self.severity {
            ErrorSeverity::Fatal => Color::rgb8(220, 53, 69), // 赤
            ErrorSeverity::Critical => Color::rgb8(255, 107, 107), // 明るい赤
            ErrorSeverity::Error => Color::rgb8(255, 193, 7), // 黄色
            ErrorSeverity::Warning => Color::rgb8(255, 235, 59), // 明るい黄色
            ErrorSeverity::Info => Color::rgb8(33, 150, 243), // 青
        }
    }

    /// 重要度に応じたアイコンを取得
    pub fn get_severity_icon(&self) -> &'static str {
        match self.severity {
            ErrorSeverity::Fatal => "⛔",
            ErrorSeverity::Critical => "❌",
            ErrorSeverity::Error => "⚠️",
            ErrorSeverity::Warning => "⚠️",
            ErrorSeverity::Info => "ℹ️",
        }
    }
}

/// エラー表示マネージャー
pub struct ErrorDisplayManager {
    /// 表示中のエラー
    active_errors: Arc<Mutex<VecDeque<ErrorDisplayInfo>>>,
    /// 最大表示数
    max_displayed_errors: usize,
    /// エラー履歴
    error_history: Arc<Mutex<Vec<ErrorDisplayInfo>>>,
    /// 最大履歴数
    max_history_size: usize,
}

impl ErrorDisplayManager {
    /// 新しいエラー表示マネージャーを作成
    pub fn new() -> Self {
        Self {
            active_errors: Arc::new(Mutex::new(VecDeque::new())),
            max_displayed_errors: 3,
            error_history: Arc::new(Mutex::new(Vec::new())),
            max_history_size: 100,
        }
    }

    /// エラーを表示キューに追加
    pub fn display_error(&self, error: &AppError) {
        let display_info = ErrorDisplayInfo::from_app_error(error);

        if let Ok(mut active) = self.active_errors.lock() {
            // 重複チェック
            if !active.iter().any(|e| e.id == display_info.id) {
                // 最大表示数を超える場合は古いものを削除
                if active.len() >= self.max_displayed_errors {
                    active.pop_front();
                }
                active.push_back(display_info.clone());
            }
        }

        // 履歴に追加
        if let Ok(mut history) = self.error_history.lock() {
            if history.len() >= self.max_history_size {
                history.remove(0);
            }
            history.push(display_info);
        }
    }

    /// エラーを削除
    pub fn dismiss_error(&self, error_id: &str) {
        if let Ok(mut active) = self.active_errors.lock() {
            active.retain(|e| e.id != error_id);
        }
    }

    /// すべてのエラーを削除
    pub fn dismiss_all_errors(&self) {
        if let Ok(mut active) = self.active_errors.lock() {
            active.clear();
        }
    }

    /// アクティブなエラーを取得
    pub fn get_active_errors(&self) -> Vec<ErrorDisplayInfo> {
        self.active_errors
            .lock()
            .map(|active| active.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// エラー履歴を取得
    pub fn get_error_history(&self) -> Vec<ErrorDisplayInfo> {
        self.error_history
            .lock()
            .map(|history| history.clone())
            .unwrap_or_default()
    }

    /// 古いエラーを自動削除
    pub fn cleanup_expired_errors(&self) {
        let now = chrono::Utc::now();

        if let Ok(mut active) = self.active_errors.lock() {
            active.retain(|error| {
                if error.auto_dismiss {
                    let elapsed = now.signed_duration_since(error.timestamp);
                    elapsed.num_seconds() < error.auto_dismiss_seconds as i64
                } else {
                    true
                }
            });
        }
    }
}

impl Default for ErrorDisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

/// エラーダイアログコンポーネント
pub fn error_dialog_component(
    error_info: ErrorDisplayInfo,
    on_action: impl Fn(ErrorActionType) + 'static + Clone,
) -> impl IntoView {
    let show_details = RwSignal::new(false);
    let severity_color = error_info.get_severity_color();
    let severity_icon = error_info.get_severity_icon();

    v_stack((
        // ヘッダー部分
        h_stack((
            label(move || severity_icon.to_string())
                .style(move |s| s.font_size(20.0).margin_right(10.0)),
            label(move || error_info.title.clone()).style(move |s| {
                s.font_size(16.0)
                    .font_weight(Weight::BOLD)
                    .color(severity_color)
            }),
            container("").style(|s| s.flex_grow(1.0)),
            // タイムスタンプ
            label(move || error_info.timestamp.format("%H:%M:%S").to_string())
                .style(|s| s.font_size(12.0).color(Color::rgb8(128, 128, 128))),
        ))
        .style(|s| s.items_center().margin_bottom(10.0)),
        // メッセージ部分
        label(move || error_info.message.clone()).style(|s| {
            s.font_size(14.0)
                .margin_bottom(15.0)
                .color(Color::rgb8(70, 70, 70))
        }),
        // 詳細表示切り替え
        container(if error_info.details.is_some() {
            button(label(move || {
                if show_details.get() {
                    "詳細を隠す"
                } else {
                    "詳細を表示"
                }
            }))
            .on_click_stop(move |_| show_details.update(|show| *show = !*show))
            .style(|s| s.font_size(12.0).margin_bottom(10.0))
            .into_any()
        } else {
            container("").into_any()
        }),
        // 詳細情報
        container(if let Some(details) = error_info.details.clone() {
            if show_details.get() {
                label(move || details.clone())
                    .style(|s| {
                        s.font_size(12.0)
                            .color(Color::rgb8(100, 100, 100))
                            .margin_bottom(15.0)
                    })
                    .into_any()
            } else {
                container("").into_any()
            }
        } else {
            container("").into_any()
        }),
        // アクションボタン
        h_stack((error_info
            .actions
            .iter()
            .map(|action| {
                let action_type = action.action_type.clone();
                let on_action_clone = on_action.clone();
                let action_label = action.label.clone();

                button(label(move || action_label.clone()))
                    .on_click_stop(move |_| on_action_clone(action_type.clone()))
                    .style(|s| s.margin_right(10.0).padding(8.0))
                    .into_any()
            })
            .collect::<Vec<_>>(),)),
    ))
    .style(move |s| {
        s.background(Color::rgb8(255, 255, 255))
            .border(2.0)
            .border_color(severity_color)
            .border_radius(8.0)
            .padding(15.0)
            .margin_bottom(10.0)
            .min_width(300.0)
            .max_width(500.0)
    })
}

/// エラー表示エリアコンポーネント
pub fn error_display_area(
    manager: Arc<ErrorDisplayManager>,
    on_action: impl Fn(&str, ErrorActionType) + 'static + Clone,
) -> impl IntoView {
    let active_errors = manager.get_active_errors();

    if active_errors.is_empty() {
        container("").into_any()
    } else {
        v_stack((active_errors
            .into_iter()
            .map(|error_info| {
                let error_id = error_info.id.clone();
                let on_action_clone = on_action.clone();

                error_dialog_component(error_info, move |action_type| {
                    on_action_clone(&error_id, action_type.clone())
                })
                .into_any()
            })
            .collect::<Vec<_>>(),))
        .style(|s| {
            s.position(Position::Absolute)
                .inset_top(20.0)
                .inset_right(20.0)
                .z_index(1000)
                .max_height(400.0)
        })
        .into_any()
    }
}

/// グローバルエラー表示マネージャー
static GLOBAL_ERROR_MANAGER: std::sync::LazyLock<ErrorDisplayManager> =
    std::sync::LazyLock::new(ErrorDisplayManager::new);

/// グローバルエラーマネージャーにアクセス
pub fn global_error_manager() -> &'static ErrorDisplayManager {
    &GLOBAL_ERROR_MANAGER
}

/// エラーを表示（グローバル関数）
pub fn display_error_globally(error: &AppError) {
    global_error_manager().display_error(error);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_explorer_utils::{AppError, ErrorCategory, ErrorSeverity};

    #[test]
    fn test_error_display_info_creation() {
        let error = AppError::Config("Test configuration error".to_string());
        let display_info = ErrorDisplayInfo::from_app_error(&error);

        assert_eq!(display_info.severity, ErrorSeverity::Critical);
        assert_eq!(display_info.category, ErrorCategory::Configuration);
        assert!(!display_info.auto_dismiss);
        assert!(!display_info.actions.is_empty());
    }

    #[test]
    fn test_error_display_manager() {
        let manager = ErrorDisplayManager::new();

        // エラーを追加
        let error1 = AppError::Config("Error 1".to_string());
        let error2 = AppError::InvalidInput("Error 2".to_string());

        manager.display_error(&error1);
        manager.display_error(&error2);

        let active_errors = manager.get_active_errors();
        assert_eq!(active_errors.len(), 2);

        // エラーを削除
        manager.dismiss_error(&active_errors[0].id);
        let remaining_errors = manager.get_active_errors();
        assert_eq!(remaining_errors.len(), 1);

        // すべて削除
        manager.dismiss_all_errors();
        let final_errors = manager.get_active_errors();
        assert!(final_errors.is_empty());
    }

    #[test]
    fn test_auto_dismiss() {
        let mut display_info =
            ErrorDisplayInfo::from_app_error(&AppError::InvalidInput("Test warning".to_string()));

        // 過去のタイムスタンプに設定
        display_info.timestamp = chrono::Utc::now() - chrono::Duration::seconds(15);
        display_info.auto_dismiss = true;
        display_info.auto_dismiss_seconds = 10;

        let manager = ErrorDisplayManager::new();

        // 手動で期限切れエラーを追加
        if let Ok(mut active) = manager.active_errors.lock() {
            active.push_back(display_info);
        }

        // クリーンアップ実行
        manager.cleanup_expired_errors();

        // 期限切れエラーが削除されているか確認
        let active_errors = manager.get_active_errors();
        assert!(active_errors.is_empty());
    }

    #[test]
    fn test_severity_colors_and_icons() {
        let error = AppError::OutOfMemory;
        let display_info = ErrorDisplayInfo::from_app_error(&error);

        let color = display_info.get_severity_color();
        let icon = display_info.get_severity_icon();

        assert_eq!(color, Color::rgb8(220, 53, 69)); // Fatal error color
        assert_eq!(icon, "⛔");
    }

    #[test]
    fn test_error_action_creation() {
        let config_error = AppError::Config("Config error".to_string());
        let display_info = ErrorDisplayInfo::from_app_error(&config_error);

        // 設定エラーには設定を開くアクションがあるはず
        let has_settings_action = display_info
            .actions
            .iter()
            .any(|action| matches!(action.action_type, ErrorActionType::OpenSettings));
        assert!(has_settings_action);
    }
}
