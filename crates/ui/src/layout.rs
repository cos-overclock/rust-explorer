//! レイアウトシステム
//!
//! レスポンシブレイアウトと動的レイアウト調整機能を提供します。

use floem::kurbo::Size;
use std::cell::RefCell;
use std::rc::Rc;

/// レイアウト設定
#[derive(Clone, Debug)]
pub struct LayoutConfig {
    /// 最小ウィンドウサイズ
    pub min_size: Size,
    /// ヘッダーの高さ
    pub header_height: f32,
    /// ステータスバーの高さ
    pub status_bar_height: f32,
    /// レスポンシブブレークポイント
    pub responsive_breakpoints: ResponsiveBreakpoints,
}

/// レスポンシブブレークポイント
#[derive(Clone, Debug)]
pub struct ResponsiveBreakpoints {
    /// 小画面（タブレット）
    pub small: f64,
    /// 中画面（デスクトップ）
    pub medium: f64,
    /// 大画面（ワイドデスクトップ）
    pub large: f64,
}

impl Default for ResponsiveBreakpoints {
    fn default() -> Self {
        Self {
            small: 768.0,
            medium: 1024.0,
            large: 1440.0,
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            min_size: Size::new(800.0, 600.0),
            header_height: 40.0,
            status_bar_height: 25.0,
            responsive_breakpoints: ResponsiveBreakpoints::default(),
        }
    }
}

/// レスポンシブレイアウトマネージャー
pub struct ResponsiveLayoutManager {
    config: Rc<RefCell<LayoutConfig>>,
    current_size: Rc<RefCell<Size>>,
}

impl ResponsiveLayoutManager {
    /// 新しいレスポンシブレイアウトマネージャーを作成
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config: Rc::new(RefCell::new(config)),
            current_size: Rc::new(RefCell::new(Size::new(1200.0, 800.0))),
        }
    }

    /// ウィンドウサイズが変更された時の処理
    pub fn on_resize(&self, new_size: Size) {
        *self.current_size.borrow_mut() = new_size;
    }

    /// 現在の画面サイズカテゴリを取得
    pub fn get_screen_size_category(&self) -> ScreenSizeCategory {
        let size = *self.current_size.borrow();
        let breakpoints = &self.config.borrow().responsive_breakpoints;

        if size.width < breakpoints.small {
            ScreenSizeCategory::XSmall
        } else if size.width < breakpoints.medium {
            ScreenSizeCategory::Small
        } else if size.width < breakpoints.large {
            ScreenSizeCategory::Medium
        } else {
            ScreenSizeCategory::Large
        }
    }

    /// メインコンテンツエリアのサイズを計算
    pub fn calculate_main_content_size(&self) -> Size {
        let current_size = *self.current_size.borrow();
        let config = self.config.borrow();

        Size::new(
            current_size.width,
            current_size.height - config.header_height as f64 - config.status_bar_height as f64,
        )
    }

    /// 設定を更新
    pub fn update_config(&self, new_config: LayoutConfig) {
        *self.config.borrow_mut() = new_config;
    }

    /// 現在の設定を取得
    pub fn get_config(&self) -> LayoutConfig {
        self.config.borrow().clone()
    }
}

/// 画面サイズカテゴリ
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScreenSizeCategory {
    /// 極小画面（< 768px）
    XSmall,
    /// 小画面（768px - 1024px）
    Small,
    /// 中画面（1024px - 1440px）
    Medium,
    /// 大画面（>= 1440px）
    Large,
}

/// レスポンシブスタイルヘルパー
pub struct ResponsiveStyle {
    layout_manager: Rc<ResponsiveLayoutManager>,
}

impl ResponsiveStyle {
    /// 新しいレスポンシブスタイルヘルパーを作成
    pub fn new(layout_manager: Rc<ResponsiveLayoutManager>) -> Self {
        Self { layout_manager }
    }

    /// 画面サイズに応じたパディングを取得
    pub fn get_responsive_padding(&self) -> f32 {
        match self.layout_manager.get_screen_size_category() {
            ScreenSizeCategory::XSmall => 10.0,
            ScreenSizeCategory::Small => 15.0,
            ScreenSizeCategory::Medium => 20.0,
            ScreenSizeCategory::Large => 25.0,
        }
    }

    /// 画面サイズに応じたフォントサイズを取得
    pub fn get_responsive_font_size(&self, base_size: f32) -> f32 {
        match self.layout_manager.get_screen_size_category() {
            ScreenSizeCategory::XSmall => base_size * 0.9,
            ScreenSizeCategory::Small => base_size,
            ScreenSizeCategory::Medium => base_size * 1.1,
            ScreenSizeCategory::Large => base_size * 1.2,
        }
    }

    /// 画面サイズに応じたマージンを取得
    pub fn get_responsive_margin(&self) -> f32 {
        match self.layout_manager.get_screen_size_category() {
            ScreenSizeCategory::XSmall => 5.0,
            ScreenSizeCategory::Small => 8.0,
            ScreenSizeCategory::Medium => 12.0,
            ScreenSizeCategory::Large => 16.0,
        }
    }
}

/// レイアウトユーティリティ関数
pub mod utils {
    use super::*;

    /// 最小サイズ制約をチェック
    pub fn check_min_size_constraint(current_size: Size, min_size: Size) -> bool {
        current_size.width >= min_size.width && current_size.height >= min_size.height
    }

    /// サイズ制約違反の警告メッセージを生成
    pub fn create_size_constraint_warning(current_size: Size, min_size: Size) -> String {
        format!(
            "Window size is below minimum. Current: {:.0}x{:.0}, Minimum: {:.0}x{:.0}",
            current_size.width, current_size.height, min_size.width, min_size.height
        )
    }

    /// アスペクト比を計算
    pub fn calculate_aspect_ratio(size: Size) -> f64 {
        if size.height > 0.0 {
            size.width / size.height
        } else {
            1.0
        }
    }

    /// 指定されたアスペクト比でサイズを調整
    pub fn adjust_size_with_aspect_ratio(size: Size, target_aspect_ratio: f64) -> Size {
        let current_ratio = calculate_aspect_ratio(size);

        if current_ratio > target_aspect_ratio {
            // 幅を調整
            Size::new(size.height * target_aspect_ratio, size.height)
        } else {
            // 高さを調整
            Size::new(size.width, size.width / target_aspect_ratio)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.min_size, Size::new(800.0, 600.0));
        assert_eq!(config.header_height, 40.0);
        assert_eq!(config.status_bar_height, 25.0);
    }

    #[test]
    fn test_responsive_breakpoints_default() {
        let breakpoints = ResponsiveBreakpoints::default();
        assert_eq!(breakpoints.small, 768.0);
        assert_eq!(breakpoints.medium, 1024.0);
        assert_eq!(breakpoints.large, 1440.0);
    }

    #[test]
    fn test_responsive_layout_manager() {
        let config = LayoutConfig::default();
        let manager = ResponsiveLayoutManager::new(config);

        // 初期サイズテスト
        assert_eq!(
            manager.get_screen_size_category(),
            ScreenSizeCategory::Medium
        );

        // リサイズテスト
        manager.on_resize(Size::new(500.0, 400.0));
        assert_eq!(
            manager.get_screen_size_category(),
            ScreenSizeCategory::XSmall
        );

        manager.on_resize(Size::new(900.0, 600.0));
        assert_eq!(
            manager.get_screen_size_category(),
            ScreenSizeCategory::Small
        );

        manager.on_resize(Size::new(1200.0, 800.0));
        assert_eq!(
            manager.get_screen_size_category(),
            ScreenSizeCategory::Medium
        );

        manager.on_resize(Size::new(1600.0, 1000.0));
        assert_eq!(
            manager.get_screen_size_category(),
            ScreenSizeCategory::Large
        );
    }

    #[test]
    fn test_main_content_size_calculation() {
        let config = LayoutConfig::default();
        let manager = ResponsiveLayoutManager::new(config);
        manager.on_resize(Size::new(1200.0, 800.0));

        let main_content_size = manager.calculate_main_content_size();
        assert_eq!(main_content_size.width, 1200.0);
        assert_eq!(main_content_size.height, 735.0); // 800 - 40 - 25
    }

    #[test]
    fn test_responsive_style() {
        let config = LayoutConfig::default();
        let manager = Rc::new(ResponsiveLayoutManager::new(config));
        let responsive_style = ResponsiveStyle::new(manager.clone());

        // 中画面でのテスト
        manager.on_resize(Size::new(1200.0, 800.0));
        assert_eq!(responsive_style.get_responsive_padding(), 20.0);
        assert_eq!(responsive_style.get_responsive_font_size(16.0), 17.6);
        assert_eq!(responsive_style.get_responsive_margin(), 12.0);

        // 小画面でのテスト
        manager.on_resize(Size::new(500.0, 400.0));
        assert_eq!(responsive_style.get_responsive_padding(), 10.0);
        assert_eq!(responsive_style.get_responsive_font_size(16.0), 14.4);
        assert_eq!(responsive_style.get_responsive_margin(), 5.0);
    }

    #[test]
    fn test_utils_functions() {
        let current_size = Size::new(1000.0, 700.0);
        let min_size = Size::new(800.0, 600.0);

        assert!(utils::check_min_size_constraint(current_size, min_size));

        let small_size = Size::new(600.0, 400.0);
        assert!(!utils::check_min_size_constraint(small_size, min_size));

        let warning = utils::create_size_constraint_warning(small_size, min_size);
        assert!(warning.contains("600x400"));
        assert!(warning.contains("800x600"));

        let aspect_ratio = utils::calculate_aspect_ratio(Size::new(1600.0, 900.0));
        assert!((aspect_ratio - 16.0 / 9.0).abs() < 0.001);
    }
}
