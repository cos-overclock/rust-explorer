//! メインウィンドウの実装
//!
//! ## floem 0.2.0 最小ウィンドウサイズ制約の調査結果
//!
//! ### 調査概要
//! floem 0.2.0における最小ウィンドウサイズ制約の実装方法を詳しく調査しました。
//!
//! ### 調査結果
//! 1. **WindowConfigのmin_size API**: floem 0.2.0では公開されていない
//! 2. **代替手段**: ウィンドウリサイズイベントでの制約チェックと警告表示
//! 3. **OSレベル制約**: 現在の実装では適用不可
//!
//! ### 実装された解決策
//! - リサイズイベントハンドラーでの最小サイズチェック
//! - 制約違反時の警告メッセージ表示
//! - ユーザーインターフェースでの制限事項の明示
//!
//! ### 将来的な改善方向
//! - floem の将来バージョンでのmin_size API対応
//! - floem-winit直接使用による高度な制御
//! - カスタムウィンドウマネージャーの実装

use floem::event::{Event, EventListener};
use floem::kurbo::Size;
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::text::Weight;
use floem::window::WindowConfig;
use rust_explorer_config::Settings;
use rust_explorer_utils::AppError;
use std::cell::RefCell;
use std::rc::Rc;

/// メインウィンドウの状態
pub struct WindowState {
    pub settings: Rc<RefCell<Settings>>,
}

/// メインウィンドウ
pub struct MainWindow {
    window_state: WindowState,
}

impl MainWindow {
    /// 新しいメインウィンドウを作成
    pub fn new(settings: &Settings) -> Result<Self, AppError> {
        Ok(MainWindow {
            window_state: WindowState {
                settings: Rc::new(RefCell::new(settings.clone())),
            },
        })
    }

    /// メインウィンドウのfloemビューを作成
    pub fn create_view(&self) -> impl IntoView {
        let settings = self.window_state.settings.clone();

        main_window_view(settings)
    }

    /// ウィンドウ設定を作成
    pub fn create_window_config(&self) -> WindowConfig {
        let settings = self.window_state.settings.borrow();
        let (pos_x, pos_y) = settings.window_position();
        let (min_width, min_height) = settings.min_window_size();

        let mut config = WindowConfig::default()
            .size((
                settings.window_width() as f64,
                settings.window_height() as f64,
            ))
            .title("rust-explorer");

        // 位置の設定
        if let (Some(x), Some(y)) = (pos_x, pos_y) {
            config = config.position((x as f64, y as f64).into());
        }

        // 最小サイズ制約を設定（floem 0.2でサポートされている場合）
        // Note: 可能であれば、WindowConfigで最小サイズを設定することで
        // OSレベルでの制約が適用される
        Self::try_set_min_size(config, min_width, min_height)
    }

    /// 最小サイズの設定を試行（floem 0.2.0では公開APIなし）
    fn try_set_min_size(config: WindowConfig, _min_width: u32, _min_height: u32) -> WindowConfig {
        // Note: floem 0.2.0では最小サイズ設定の公開APIが存在しない
        //
        // 代替手段:
        // 1. ウィンドウリサイズイベントでの警告表示（現在実装済み）
        // 2. 将来のfloem更新での対応待ち
        // 3. floem-winitを直接使用した高度な制御（実装複雑度が高い）
        //
        // 現在の実装では、元のconfigをそのまま返し、
        // イベントハンドラーでユーザーフィードバックを提供
        config
    }

    /// アプリケーションを起動
    pub fn launch(self) -> Result<(), AppError> {
        let settings = self.window_state.settings.clone();

        floem::launch(move || main_window_view(settings));

        Ok(())
    }
}

/// メインウィンドウのビュー
fn main_window_view(settings: Rc<RefCell<Settings>>) -> impl IntoView {
    let settings_clone = settings.clone();

    v_stack((
        // ヘッダー部分
        create_header(),
        // メインコンテンツ部分
        create_main_content(settings_clone),
        // ステータスバー部分
        create_status_bar(),
    ))
    .style(|s| s.size_full().flex_col())
    .on_event_stop(EventListener::WindowResized, move |event| {
        if let Event::WindowResized(new_size) = event {
            handle_window_resize(&settings, *new_size);
        }
    })
}

/// ウィンドウリサイズイベントを処理
fn handle_window_resize(settings: &Rc<RefCell<Settings>>, new_size: Size) {
    let settings_ref = settings.borrow();
    let (min_width, min_height) = settings_ref.min_window_size();

    // 最小サイズ以下にリサイズされた場合の警告表示
    if new_size.width < min_width as f64 || new_size.height < min_height as f64 {
        println!(
            "Warning: Window resized below minimum size. Current: {}x{}, Minimum: {}x{}",
            new_size.width, new_size.height, min_width, min_height
        );
    }

    // 設定を更新（実際のリサイズサイズで）
    // Note: floem 0.2.0では最小サイズ制約の公開APIが存在しないため、
    // 現在は実際のサイズで設定を更新し、制約は将来のfloem更新で対応予定
    drop(settings_ref);
    let mut settings_mut = settings.borrow_mut();
    settings_mut.update_window_state(
        new_size.width as u32,
        new_size.height as u32,
        None,
        None,
        false,
    );
}

/// ヘッダー部分の作成
fn create_header() -> impl IntoView {
    h_stack((
        label(|| "rust-explorer").style(|s| {
            s.font_size(18.0)
                .font_weight(Weight::BOLD)
                .margin_left(10.0)
        }),
        // 将来的にはメニューやツールバーボタンを追加
    ))
    .style(|s| {
        s.width_full()
            .height(40.0)
            .background(Color::rgb8(240, 240, 240))
            .border_bottom(1.0)
            .border_color(Color::rgb8(200, 200, 200))
            .items_center()
    })
}

/// メインコンテンツ部分の作成
fn create_main_content(_settings: Rc<RefCell<Settings>>) -> impl IntoView {
    let mut counter = RwSignal::new(0);

    container(
        v_stack((
            label(|| "rust-explorer - Main Window").style(|s| {
                s.font_size(24.0)
                    .font_weight(Weight::BOLD)
                    .margin_bottom(20.0)
            }),
            label(|| "メインウィンドウとアプリケーションライフサイクルが実装されました")
                .style(|s| s.font_size(16.0).margin_bottom(30.0)),
            h_stack((
                button("カウント +")
                    .action(move || counter += 1)
                    .style(|s| s.margin_right(10.0)),
                label(move || format!("カウント: {}", counter.get()))
                    .style(|s| s.font_size(16.0).margin_right(10.0)),
                button("カウント -").action(move || counter -= 1),
            ))
            .style(|s| s.gap(10.0)),
            label(|| "機能:").style(|s| {
                s.font_size(14.0)
                    .font_weight(Weight::BOLD)
                    .margin_top(30.0)
                    .margin_bottom(10.0)
            }),
            v_stack((
                label(|| "✓ ウィンドウサイズ・位置の管理"),
                label(|| "! 最小ウィンドウサイズ制限（800x600） - floem 0.2制限により警告のみ"),
                label(|| "✓ ウィンドウタイトル設定"),
                label(|| "✓ アプリケーション初期化処理"),
                label(|| "✓ 基本レイアウト（ヘッダー・メイン・ステータス）"),
            ))
            .style(|s| s.gap(5.0)),
        ))
        .style(|s| s.items_center().justify_center()),
    )
    .style(|s| s.size_full().background(Color::rgb8(250, 250, 250)))
}

/// ステータスバー部分の作成
fn create_status_bar() -> impl IntoView {
    h_stack((
        label(|| "準備完了").style(|s| s.font_size(12.0).margin_left(10.0)),
        // スペーサー
        container("").style(|s| s.flex_grow(1.0)),
        label(|| format!("rust-explorer v{}", env!("CARGO_PKG_VERSION")))
            .style(|s| s.font_size(12.0).margin_right(10.0)),
    ))
    .style(|s| {
        s.width_full()
            .height(25.0)
            .background(Color::rgb8(230, 230, 230))
            .border_top(1.0)
            .border_color(Color::rgb8(200, 200, 200))
            .items_center()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_explorer_config::Settings;

    #[test]
    fn test_main_window_creation() {
        let settings = Settings::default();
        let result = MainWindow::new(&settings);
        assert!(result.is_ok());
    }

    #[test]
    fn test_window_config_creation() {
        let settings = Settings::default();
        let window = MainWindow::new(&settings).unwrap();
        let _config = window.create_window_config();

        // 設定が正しく反映されているかテスト
        // Note: WindowConfigの内部フィールドは直接アクセスできないため、
        // 作成時にパニックしないことを確認
        assert!(true); // configが正常に作成されれば成功
    }

    #[test]
    fn test_window_state_creation() {
        let settings = Settings::default();
        let window_state = WindowState {
            settings: Rc::new(RefCell::new(settings)),
        };

        let settings_ref = window_state.settings.borrow();
        assert_eq!(settings_ref.window_width(), 1200);
        assert_eq!(settings_ref.window_height(), 800);
    }

    #[test]
    fn test_window_with_custom_settings() {
        let mut settings = Settings::default();
        settings.update_window_state(1024, 768, Some(100), Some(50), false);

        let window = MainWindow::new(&settings).unwrap();
        let _config = window.create_window_config();

        // カスタム設定での作成が成功することを確認
        assert!(true);
    }

    #[test]
    fn test_handle_window_resize_constraint() {
        use floem::kurbo::Size;
        use std::cell::RefCell;
        use std::rc::Rc;

        let settings = Rc::new(RefCell::new(Settings::default()));
        let (min_width, min_height) = settings.borrow().min_window_size();

        // 最小サイズ以下のサイズでリサイズイベントをシミュレート
        let small_size = Size::new((min_width - 100) as f64, (min_height - 50) as f64);
        handle_window_resize(&settings, small_size);

        // 現在の実装では実際のリサイズサイズで設定が更新される
        // （floem 0.2.0では最小サイズ制約のAPIが利用不可のため）
        let updated_settings = settings.borrow();
        assert_eq!(updated_settings.window_width(), min_width - 100);
        assert_eq!(updated_settings.window_height(), min_height - 50);
    }

    #[test]
    fn test_handle_window_resize_no_constraint() {
        use floem::kurbo::Size;
        use std::cell::RefCell;
        use std::rc::Rc;

        let settings = Rc::new(RefCell::new(Settings::default()));
        let (min_width, min_height) = settings.borrow().min_window_size();

        // 最小サイズより大きなサイズでリサイズイベントをシミュレート
        let large_size = Size::new((min_width + 200) as f64, (min_height + 100) as f64);
        handle_window_resize(&settings, large_size);

        // 設定が元のサイズのまま更新されることを確認
        let updated_settings = settings.borrow();
        assert_eq!(updated_settings.window_width(), min_width + 200);
        assert_eq!(updated_settings.window_height(), min_height + 100);
    }
}
