//! パンくずナビゲーションコンポーネント

use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::{
    IntoView,
    views::{Decorators, h_stack, label},
};
use std::path::{Path, PathBuf};

/// パンくずナビゲーションの設定
#[derive(Debug, Clone)]
pub struct BreadcrumbConfig {
    /// 最大表示パス数（これを超えると省略される）
    pub max_visible_paths: usize,
    /// 区切り文字
    pub separator: String,
    /// ホームディレクトリのアイコン
    pub home_icon: String,
    /// ルートディレクトリのアイコン
    pub root_icon: String,
    /// フォルダアイコン
    pub folder_icon: String,
}

impl Default for BreadcrumbConfig {
    fn default() -> Self {
        Self {
            max_visible_paths: 6,
            separator: " / ".to_string(),
            home_icon: "🏠".to_string(),
            root_icon: "💻".to_string(),
            folder_icon: "📁".to_string(),
        }
    }
}

/// パンくずナビゲーションのパス要素
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    /// 表示名
    pub name: String,
    /// フルパス
    pub path: PathBuf,
    /// アイコン（オプション）
    pub icon: Option<String>,
    /// クリック可能かどうか
    pub clickable: bool,
}

/// パンくずナビゲーションコンポーネント
pub struct BreadcrumbNavigation {
    /// 現在のパス
    current_path: RwSignal<PathBuf>,
    /// 設定
    _config: BreadcrumbConfig,
    /// パス変更コールバック
    on_path_change: Option<Box<dyn Fn(PathBuf) + Send + Sync>>,
}

impl BreadcrumbNavigation {
    /// 新しいパンくずナビゲーションを作成
    pub fn new(initial_path: PathBuf, config: BreadcrumbConfig) -> Self {
        Self {
            current_path: RwSignal::new(initial_path),
            _config: config,
            on_path_change: None,
        }
    }

    /// デフォルト設定でパンくずナビゲーションを作成
    pub fn with_default(initial_path: PathBuf) -> Self {
        Self::new(initial_path, BreadcrumbConfig::default())
    }

    /// パス変更コールバックを設定
    pub fn on_path_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(PathBuf) + Send + Sync + 'static,
    {
        self.on_path_change = Some(Box::new(callback));
        self
    }

    /// パスを更新
    pub fn set_path(&self, path: PathBuf) {
        self.current_path.set(path);
    }

    /// 現在のパスを取得
    pub fn current_path(&self) -> PathBuf {
        self.current_path.get()
    }

    /// パンくずナビゲーションビューを作成
    pub fn view(&self) -> impl IntoView {
        let current_path = self.current_path;

        h_stack((label(move || {
            let path = current_path.get();
            create_breadcrumb_text(&path)
        })
        .style(|s| {
            s.font_size(14)
                .color(Color::rgb8(107, 114, 128))
                .padding_horiz(8)
                .padding_vert(4)
        }),))
        .style(|s| {
            s.items_center()
                .padding(8)
                .background(Color::rgb8(248, 249, 250))
                .border_radius(6)
                .border(1)
                .border_color(Color::rgb8(220, 222, 224))
        })
    }

    /// パスを変更（内部使用）
    #[allow(dead_code)]
    fn change_path(&self, new_path: PathBuf) {
        self.current_path.set(new_path.clone());
        if let Some(callback) = &self.on_path_change {
            callback(new_path);
        }
    }
}

/// パンくずナビゲーションのアイテムを作成
#[allow(dead_code)]
fn create_breadcrumb_items(path: &Path, config: &BreadcrumbConfig) -> Vec<BreadcrumbItem> {
    let mut items = Vec::new();
    let home_dir = dirs::home_dir();

    // パスを正規化
    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let components: Vec<_> = canonical_path.components().collect();

    if components.is_empty() {
        return items;
    }

    // ルートディレクトリの処理
    if let Some(first) = components.first() {
        let root_path = PathBuf::from(first.as_os_str());
        items.push(BreadcrumbItem {
            name: config.root_icon.clone(),
            path: root_path,
            icon: Some(config.root_icon.clone()),
            clickable: true,
        });
    }

    // 各ディレクトリコンポーネントを処理
    let mut current_path = PathBuf::new();
    for component in &components {
        current_path.push(component);

        if current_path.as_os_str() == components.first().unwrap().as_os_str() {
            continue; // ルートは既に追加済み
        }

        let name = if Some(&current_path) == home_dir.as_ref() {
            config.home_icon.clone()
        } else {
            component
                .as_os_str()
                .to_string_lossy()
                .trim_matches('/')
                .to_string()
        };

        let icon = if Some(&current_path) == home_dir.as_ref() {
            Some(config.home_icon.clone())
        } else {
            Some(config.folder_icon.clone())
        };

        items.push(BreadcrumbItem {
            name,
            path: current_path.clone(),
            icon,
            clickable: current_path != canonical_path, // 現在のパス以外はクリック可能
        });
    }

    // 長いパスの省略処理
    if items.len() > config.max_visible_paths {
        let mut result = Vec::new();
        result.push(items[0].clone()); // ルートは常に表示

        if items.len() > config.max_visible_paths + 1 {
            // 省略記号を追加
            result.push(BreadcrumbItem {
                name: "...".to_string(),
                path: PathBuf::new(),
                icon: None,
                clickable: false,
            });
        }

        // 末尾の要素を追加
        let start_index = items.len().saturating_sub(config.max_visible_paths - 2);
        result.extend(items[start_index..].iter().cloned());
        return result;
    }

    items
}

/// パンくずナビゲーションコンポーネントを作成（便利関数）
pub fn breadcrumb_navigation(
    initial_path: PathBuf,
    config: BreadcrumbConfig,
) -> BreadcrumbNavigation {
    BreadcrumbNavigation::new(initial_path, config)
}

/// デフォルト設定でパンくずナビゲーションを作成（便利関数）
pub fn default_breadcrumb_navigation(initial_path: PathBuf) -> BreadcrumbNavigation {
    BreadcrumbNavigation::with_default(initial_path)
}

/// シンプルなパンくずナビゲーションビューを作成
pub fn breadcrumb_view(current_path: RwSignal<PathBuf>) -> impl IntoView {
    h_stack((
        // パンくずリストコンテナ
        label(move || {
            let path = current_path.get();
            create_breadcrumb_text(&path)
        })
        .style(|s| {
            s.font_size(14)
                .color(Color::rgb8(107, 114, 128))
                .padding_horiz(8)
                .padding_vert(4)
        }),
    ))
    .style(|s| {
        s.items_center()
            .padding(8)
            .background(Color::rgb8(248, 249, 250))
            .border_radius(6)
            .border(1)
            .border_color(Color::rgb8(220, 222, 224))
    })
}

/// パンくずテキストを作成（シンプル版）
fn create_breadcrumb_text(path: &Path) -> String {
    let config = BreadcrumbConfig::default();
    let home_dir = dirs::home_dir();

    // パスを正規化
    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let components: Vec<_> = canonical_path.components().collect();

    if components.is_empty() {
        return config.root_icon;
    }

    let mut parts = Vec::new();

    // ルートアイコンを追加
    parts.push(config.root_icon.clone());

    // 各コンポーネントを処理
    let mut current_path = PathBuf::new();
    for component in &components {
        current_path.push(component);

        if current_path.as_os_str() == components.first().unwrap().as_os_str() {
            continue; // ルートは既に追加済み
        }

        let name = if Some(&current_path) == home_dir.as_ref() {
            config.home_icon.clone()
        } else {
            component
                .as_os_str()
                .to_string_lossy()
                .trim_matches('/')
                .to_string()
        };

        if !name.is_empty() {
            parts.push(name);
        }
    }

    // 長いパスの省略処理
    if parts.len() > config.max_visible_paths {
        let mut result = Vec::new();
        result.push(parts[0].clone()); // ルート
        result.push("...".to_string()); // 省略記号

        // 末尾の要素を追加
        let start_index = parts.len().saturating_sub(config.max_visible_paths - 2);
        result.extend(parts[start_index..].iter().cloned());
        parts = result;
    }

    parts.join(&config.separator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_config_default() {
        let config = BreadcrumbConfig::default();
        assert_eq!(config.max_visible_paths, 6);
        assert_eq!(config.separator, " / ");
        assert_eq!(config.home_icon, "🏠");
        assert_eq!(config.root_icon, "💻");
        assert_eq!(config.folder_icon, "📁");
    }

    #[test]
    fn test_breadcrumb_item_creation() {
        let item = BreadcrumbItem {
            name: "Documents".to_string(),
            path: PathBuf::from("/home/user/Documents"),
            icon: Some("📁".to_string()),
            clickable: true,
        };

        assert_eq!(item.name, "Documents");
        assert_eq!(item.path, PathBuf::from("/home/user/Documents"));
        assert_eq!(item.icon, Some("📁".to_string()));
        assert!(item.clickable);
    }

    #[test]
    fn test_create_breadcrumb_items_simple_path() {
        let config = BreadcrumbConfig::default();
        let path = Path::new("/home/user/Documents");
        let items = create_breadcrumb_items(path, &config);

        assert!(!items.is_empty());
        // ルートアイテムが含まれていることを確認
        assert!(items.iter().any(|item| item.name == config.root_icon));
    }

    #[test]
    fn test_create_breadcrumb_items_root_path() {
        let config = BreadcrumbConfig::default();
        let path = Path::new("/");
        let items = create_breadcrumb_items(path, &config);

        assert!(!items.is_empty());
        assert_eq!(items[0].name, config.root_icon);
    }

    #[test]
    fn test_breadcrumb_navigation_creation() {
        let path = PathBuf::from("/home/user");
        let nav = BreadcrumbNavigation::with_default(path.clone());
        assert_eq!(nav.current_path(), path);
    }

    #[test]
    fn test_breadcrumb_navigation_set_path() {
        let initial_path = PathBuf::from("/home/user");
        let new_path = PathBuf::from("/home/user/Documents");

        let nav = BreadcrumbNavigation::with_default(initial_path);
        nav.set_path(new_path.clone());

        assert_eq!(nav.current_path(), new_path);
    }

    #[test]
    fn test_breadcrumb_items_long_path_truncation() {
        let config = BreadcrumbConfig {
            max_visible_paths: 3,
            ..Default::default()
        };

        // 非常に長いパスを作成
        let path = Path::new("/very/long/path/with/many/components/that/should/be/truncated");
        let items = create_breadcrumb_items(path, &config);

        // 省略により指定した最大数以下になることを確認
        assert!(items.len() <= config.max_visible_paths + 1); // +1は省略記号のため

        // 省略記号が含まれていることを確認
        assert!(items.iter().any(|item| item.name == "..."));
    }

    #[test]
    fn test_breadcrumb_item_clickable_logic() {
        let config = BreadcrumbConfig::default();
        let path = Path::new("/home/user/Documents");
        let items = create_breadcrumb_items(path, &config);

        // 最後の要素（現在のパス）以外はクリック可能
        if items.len() > 1 {
            for item in &items[..items.len() - 1] {
                if item.name != "..." {
                    // 省略記号以外
                    assert!(
                        item.clickable,
                        "非最終要素はクリック可能である必要があります: {}",
                        item.name
                    );
                }
            }
        }
    }
}
