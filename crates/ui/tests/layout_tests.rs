//! レイアウトシステムの統合テスト

use floem::kurbo::Size;
use rust_explorer_ui::components::{ContentType, HeaderConfig, MainContentConfig, StatusBarConfig};
use rust_explorer_ui::layout::{LayoutConfig, ResponsiveLayoutManager, ScreenSizeCategory};

#[test]
fn test_complete_layout_system() {
    // レイアウト設定の作成
    let layout_config = LayoutConfig::default();
    let layout_manager = ResponsiveLayoutManager::new(layout_config);

    // 異なる画面サイズでのテスト
    let test_sizes = vec![
        (Size::new(500.0, 400.0), ScreenSizeCategory::XSmall),
        (Size::new(800.0, 600.0), ScreenSizeCategory::Small),
        (Size::new(1200.0, 800.0), ScreenSizeCategory::Medium),
        (Size::new(1600.0, 1000.0), ScreenSizeCategory::Large),
    ];

    for (size, expected_category) in test_sizes {
        layout_manager.on_resize(size);
        assert_eq!(layout_manager.get_screen_size_category(), expected_category);

        let main_content_size = layout_manager.calculate_main_content_size();
        assert_eq!(main_content_size.width, size.width);
        assert_eq!(main_content_size.height, size.height - 40.0 - 25.0); // header + status bar
    }
}

#[test]
fn test_component_configuration_compatibility() {
    // ヘッダー設定テスト
    let header_config = HeaderConfig::default();
    assert_eq!(header_config.height, 40.0);
    assert_eq!(header_config.title, "rust-explorer");

    // メインコンテンツ設定テスト
    let main_config = MainContentConfig::default();
    assert_eq!(main_config.padding, 20.0);
    matches!(main_config.content_type, ContentType::Welcome);

    // ステータスバー設定テスト
    let status_config = StatusBarConfig::default();
    assert_eq!(status_config.height, 25.0);
    assert!(status_config.show_version);
}

#[test]
fn test_layout_calculations() {
    let config = LayoutConfig::default();
    let manager = ResponsiveLayoutManager::new(config);

    // 標準的なデスクトップサイズ
    manager.on_resize(Size::new(1920.0, 1080.0));
    let main_content_size = manager.calculate_main_content_size();

    assert_eq!(main_content_size.width, 1920.0);
    assert_eq!(main_content_size.height, 1015.0); // 1080 - 40 - 25

    // 最小サイズでのテスト
    manager.on_resize(Size::new(800.0, 600.0));
    let main_content_size = manager.calculate_main_content_size();

    assert_eq!(main_content_size.width, 800.0);
    assert_eq!(main_content_size.height, 535.0); // 600 - 40 - 25
}

#[test]
fn test_responsive_behavior() {
    let config = LayoutConfig::default();
    let layout_manager = std::rc::Rc::new(ResponsiveLayoutManager::new(config));
    let responsive_style = rust_explorer_ui::layout::ResponsiveStyle::new(layout_manager.clone());

    // XSmall画面でのテスト
    layout_manager.on_resize(Size::new(500.0, 400.0));
    assert_eq!(responsive_style.get_responsive_padding(), 10.0);
    assert_eq!(responsive_style.get_responsive_margin(), 5.0);

    // Large画面でのテスト
    layout_manager.on_resize(Size::new(1600.0, 1000.0));
    assert_eq!(responsive_style.get_responsive_padding(), 25.0);
    assert_eq!(responsive_style.get_responsive_margin(), 16.0);
}

#[test]
fn test_layout_constraints() {
    use rust_explorer_ui::layout::utils;

    let min_size = Size::new(800.0, 600.0);

    // 制約を満たすサイズ
    let valid_size = Size::new(1200.0, 800.0);
    assert!(utils::check_min_size_constraint(valid_size, min_size));

    // 制約を満たさないサイズ
    let invalid_size = Size::new(700.0, 500.0);
    assert!(!utils::check_min_size_constraint(invalid_size, min_size));

    // 警告メッセージの生成
    let warning = utils::create_size_constraint_warning(invalid_size, min_size);
    assert!(warning.contains("700x500"));
    assert!(warning.contains("800x600"));
}

#[test]
fn test_aspect_ratio_calculations() {
    use rust_explorer_ui::layout::utils;

    // 16:9 アスペクト比のテスト
    let size_16_9 = Size::new(1920.0, 1080.0);
    let ratio = utils::calculate_aspect_ratio(size_16_9);
    assert!((ratio - 16.0 / 9.0).abs() < 0.001);

    // アスペクト比調整のテスト
    let original_size = Size::new(1000.0, 800.0);
    let target_ratio = 16.0 / 9.0;
    let adjusted_size = utils::adjust_size_with_aspect_ratio(original_size, target_ratio);

    let new_ratio = utils::calculate_aspect_ratio(adjusted_size);
    assert!((new_ratio - target_ratio).abs() < 0.001);
}

#[test]
fn test_layout_config_updates() {
    let mut config = LayoutConfig::default();
    let manager = ResponsiveLayoutManager::new(config.clone());

    // 設定の更新
    config.header_height = 50.0;
    config.status_bar_height = 30.0;
    manager.update_config(config);

    // 更新された設定での計算
    manager.on_resize(Size::new(1200.0, 800.0));
    let main_content_size = manager.calculate_main_content_size();

    assert_eq!(main_content_size.height, 720.0); // 800 - 50 - 30
}
