//! モダンなテーマシステム
//!
//! Files CommunityとLapceにインスパイアされたモダンなデザインテーマを提供

use floem::peniko::Color;

/// テーマの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// ライトテーマ
    Light,
    /// ダークテーマ
    Dark,
    /// システム設定に従う
    System,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::System
    }
}

/// カラーパレット
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // 基本色
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,

    // 背景色
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,

    // テキスト色
    pub on_background: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub on_primary: Color,

    // ボーダー色
    pub border: Color,
    pub border_variant: Color,
    pub border_focus: Color,

    // インタラクション色
    pub hover: Color,
    pub pressed: Color,
    pub selected: Color,
    pub selected_hover: Color,

    // シャドウ色
    pub shadow: Color,
    pub elevation_1: Color,
    pub elevation_2: Color,
    pub elevation_3: Color,
}

impl ColorPalette {
    /// ライトテーマのカラーパレット
    pub fn light() -> Self {
        Self {
            // 基本色 - Modern Blue系
            primary: Color::rgb8(0, 120, 212),  // Modern Blue
            secondary: Color::rgb8(96, 94, 92), // Neutral Gray
            accent: Color::rgb8(255, 140, 0),   // Orange Accent
            success: Color::rgb8(16, 124, 16),  // Green
            warning: Color::rgb8(255, 193, 7),  // Yellow
            error: Color::rgb8(196, 43, 28),    // Red

            // 背景色 - Clean & Minimal
            background: Color::rgb8(249, 249, 249), // Light Gray Background
            surface: Color::rgb8(255, 255, 255),    // Pure White
            surface_variant: Color::rgb8(246, 246, 246), // Subtle Gray
            surface_container: Color::rgb8(243, 243, 243), // Container Gray
            surface_container_high: Color::rgb8(237, 237, 237), // Elevated Gray

            // テキスト色
            on_background: Color::rgb8(32, 31, 30), // Dark Gray
            on_surface: Color::rgb8(50, 49, 48),    // Text Gray
            on_surface_variant: Color::rgb8(96, 94, 92), // Secondary Text
            on_primary: Color::rgb8(255, 255, 255), // White on Primary

            // ボーダー色
            border: Color::rgb8(225, 223, 221), // Light Border
            border_variant: Color::rgb8(237, 235, 233), // Subtle Border
            border_focus: Color::rgb8(0, 120, 212), // Blue Focus Border

            // インタラクション色
            hover: Color::rgb8(243, 242, 241),       // Light Hover
            pressed: Color::rgb8(237, 235, 233),     // Light Pressed
            selected: Color::rgba8(0, 120, 212, 26), // Blue Selection (10% opacity)
            selected_hover: Color::rgba8(0, 120, 212, 38), // Blue Selection Hover (15%)

            // シャドウ色
            shadow: Color::rgba8(0, 0, 0, 20),      // Subtle Shadow
            elevation_1: Color::rgba8(0, 0, 0, 5),  // Very Light Shadow
            elevation_2: Color::rgba8(0, 0, 0, 10), // Light Shadow
            elevation_3: Color::rgba8(0, 0, 0, 15), // Medium Shadow
        }
    }

    /// ダークテーマのカラーパレット
    pub fn dark() -> Self {
        Self {
            // 基本色 - Modern Blue系 (ダーク調整)
            primary: Color::rgb8(99, 162, 255), // Lighter Blue for Dark
            secondary: Color::rgb8(161, 159, 157), // Light Gray
            accent: Color::rgb8(255, 185, 0),   // Bright Orange
            success: Color::rgb8(107, 203, 119), // Light Green
            warning: Color::rgb8(255, 213, 79), // Light Yellow
            error: Color::rgb8(255, 99, 71),    // Light Red

            // 背景色 - Modern Dark
            background: Color::rgb8(16, 16, 16), // Very Dark Background
            surface: Color::rgb8(24, 24, 24),    // Dark Surface
            surface_variant: Color::rgb8(32, 32, 32), // Variant Surface
            surface_container: Color::rgb8(40, 40, 40), // Container Surface
            surface_container_high: Color::rgb8(48, 48, 48), // Elevated Surface

            // テキスト色
            on_background: Color::rgb8(240, 240, 240), // Light Text
            on_surface: Color::rgb8(220, 220, 220),    // Surface Text
            on_surface_variant: Color::rgb8(180, 180, 180), // Secondary Text
            on_primary: Color::rgb8(16, 16, 16),       // Dark on Primary

            // ボーダー色
            border: Color::rgb8(60, 60, 60),         // Dark Border
            border_variant: Color::rgb8(48, 48, 48), // Subtle Dark Border
            border_focus: Color::rgb8(99, 162, 255), // Blue Focus Border

            // インタラクション色
            hover: Color::rgb8(40, 40, 40),           // Dark Hover
            pressed: Color::rgb8(32, 32, 32),         // Dark Pressed
            selected: Color::rgba8(99, 162, 255, 26), // Blue Selection (10% opacity)
            selected_hover: Color::rgba8(99, 162, 255, 38), // Blue Selection Hover (15%)

            // シャドウ色
            shadow: Color::rgba8(0, 0, 0, 50), // Stronger Shadow for Dark
            elevation_1: Color::rgba8(255, 255, 255, 3), // Light Elevation
            elevation_2: Color::rgba8(255, 255, 255, 6), // Medium Elevation
            elevation_3: Color::rgba8(255, 255, 255, 9), // High Elevation
        }
    }
}

/// タイポグラフィー設定
#[derive(Debug, Clone)]
pub struct Typography {
    // フォントサイズ
    pub display_large: f32,
    pub display_medium: f32,
    pub display_small: f32,
    pub headline_large: f32,
    pub headline_medium: f32,
    pub headline_small: f32,
    pub title_large: f32,
    pub title_medium: f32,
    pub title_small: f32,
    pub body_large: f32,
    pub body_medium: f32,
    pub body_small: f32,
    pub label_large: f32,
    pub label_medium: f32,
    pub label_small: f32,

    // 行間
    pub line_height_tight: f32,
    pub line_height_normal: f32,
    pub line_height_relaxed: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            // フォントサイズ（Material Design 3準拠）
            display_large: 57.0,
            display_medium: 45.0,
            display_small: 36.0,
            headline_large: 32.0,
            headline_medium: 28.0,
            headline_small: 24.0,
            title_large: 22.0,
            title_medium: 16.0,
            title_small: 14.0,
            body_large: 16.0,
            body_medium: 14.0,
            body_small: 12.0,
            label_large: 14.0,
            label_medium: 12.0,
            label_small: 11.0,

            // 行間
            line_height_tight: 1.2,
            line_height_normal: 1.4,
            line_height_relaxed: 1.6,
        }
    }
}

/// スペーシング設定
#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: f32,  // 4px
    pub sm: f32,  // 8px
    pub md: f32,  // 16px
    pub lg: f32,  // 24px
    pub xl: f32,  // 32px
    pub xxl: f32, // 48px
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

/// ボーダーラジアス設定
#[derive(Debug, Clone)]
pub struct BorderRadius {
    pub none: f32,
    pub xs: f32,   // 2px
    pub sm: f32,   // 4px
    pub md: f32,   // 8px
    pub lg: f32,   // 12px
    pub xl: f32,   // 16px
    pub full: f32, // 9999px (pill shape)
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            none: 0.0,
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        }
    }
}

/// アニメーション設定
#[derive(Debug, Clone)]
pub struct Animation {
    pub duration_fast: f32,   // 150ms
    pub duration_normal: f32, // 250ms
    pub duration_slow: f32,   // 350ms
    pub easing_standard: String,
    pub easing_emphasized: String,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            duration_fast: 0.15,
            duration_normal: 0.25,
            duration_slow: 0.35,
            easing_standard: "cubic-bezier(0.2, 0.0, 0, 1.0)".to_string(),
            easing_emphasized: "cubic-bezier(0.3, 0.0, 0.8, 0.15)".to_string(),
        }
    }
}

/// メインテーマ構造体
#[derive(Debug, Clone)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub border_radius: BorderRadius,
    pub animation: Animation,
}

impl Theme {
    /// ライトテーマを作成
    pub fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            colors: ColorPalette::light(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            animation: Animation::default(),
        }
    }

    /// ダークテーマを作成
    pub fn dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            colors: ColorPalette::dark(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            animation: Animation::default(),
        }
    }

    /// システムテーマを作成（現在はライトテーマを返す）
    pub fn system() -> Self {
        // TODO: システムの設定を検出して適切なテーマを返す
        Self::light()
    }

    /// テーマバリアントに基づいてテーマを作成
    pub fn new(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Light => Self::light(),
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::System => Self::system(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// グローバルテーマ管理
use std::sync::{Arc, RwLock};

static GLOBAL_THEME: std::sync::OnceLock<Arc<RwLock<Theme>>> = std::sync::OnceLock::new();

/// グローバルテーマを取得
pub fn get_theme() -> Arc<RwLock<Theme>> {
    GLOBAL_THEME
        .get_or_init(|| Arc::new(RwLock::new(Theme::default())))
        .clone()
}

/// グローバルテーマを設定
pub fn set_theme(theme: Theme) {
    if let Ok(mut global_theme) = get_theme().write() {
        *global_theme = theme;
    }
}

/// テーマバリアントを切り替え
pub fn switch_theme(variant: ThemeVariant) {
    set_theme(Theme::new(variant));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let light_theme = Theme::light();
        assert_eq!(light_theme.variant, ThemeVariant::Light);

        let dark_theme = Theme::dark();
        assert_eq!(dark_theme.variant, ThemeVariant::Dark);
    }

    #[test]
    fn test_color_palette() {
        let light_colors = ColorPalette::light();
        let dark_colors = ColorPalette::dark();

        // ライトテーマは明るい背景
        assert_eq!(light_colors.background, Color::rgb8(249, 249, 249));

        // ダークテーマは暗い背景
        assert_eq!(dark_colors.background, Color::rgb8(16, 16, 16));
    }

    #[test]
    fn test_typography() {
        let typography = Typography::default();

        assert_eq!(typography.body_medium, 14.0);
        assert_eq!(typography.headline_large, 32.0);
    }

    #[test]
    fn test_spacing() {
        let spacing = Spacing::default();

        assert_eq!(spacing.xs, 4.0);
        assert_eq!(spacing.md, 16.0);
        assert_eq!(spacing.xl, 32.0);
    }

    #[test]
    fn test_global_theme() {
        let theme = get_theme();
        assert!(theme.read().is_ok());

        switch_theme(ThemeVariant::Dark);
        let current_theme = theme.read().unwrap();
        assert_eq!(current_theme.variant, ThemeVariant::Dark);
    }
}
