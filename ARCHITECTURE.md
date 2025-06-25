# プロジェクト構造アーキテクチャ

このドキュメントは、rust-explorerプロジェクトの構造と設計思想について説明します。

## ディレクトリ構造

```
src/
├── main.rs           # アプリケーションエントリポイント
└── lib.rs            # ライブラリクレートの公開API（再エクスポート）

crates/               # 再利用可能なクレート群
├── utils/            # ユーティリティクレート
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── error.rs  # エラー定義
├── config/           # 設定管理クレート
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── settings.rs # アプリケーション設定
├── core/             # コア機能クレート
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── filesystem.rs # ファイルシステム操作
│       └── event.rs  # イベント管理システム
└── ui/               # ユーザーインターフェースクレート
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── app.rs    # アプリケーションメインクラス
        ├── window.rs # メインウィンドウ管理
        └── components/ # 再利用可能なUIコンポーネント
            └── mod.rs
```

## クレート設計

### rust-explorer-utils クレート

- **責務**: 共通ユーティリティとエラー定義
- **依存関係**: 外部クレートのみ（循環依存なし）
- **主要コンポーネント**:
  - `AppError`: 統一されたエラー型定義（thiserror 2.0使用）
  - `AppResult`: 結果型のエイリアス

### rust-explorer-config クレート

- **責務**: 設定の管理と永続化
- **依存関係**: rust-explorer-utilsのみに依存
- **主要コンポーネント**:
  - `Settings`: アプリケーション設定の定義と操作

### rust-explorer-core クレート

- **責務**: アプリケーションのコア機能とビジネスロジック
- **依存関係**: rust-explorer-utilsのみに依存（UI非依存）
- **主要コンポーネント**:
  - `FileSystemManager`: ファイルシステム操作の抽象化
  - `EventManager`: アプリケーション内イベント管理
  - `Event`: イベント型定義

### rust-explorer-ui クレート

- **責務**: ユーザーインターフェイスの実装
- **依存関係**: すべての下位クレートに依存
- **主要コンポーネント**:
  - `App`: アプリケーションの主要制御クラス
  - `MainWindow`: メインウィンドウの管理
  - `components`: 再利用可能なUIコンポーネント群

## 依存関係設計

```
rust-explorer-ui → rust-explorer-core + rust-explorer-config + rust-explorer-utils
rust-explorer-core → rust-explorer-utils
rust-explorer-config → rust-explorer-utils
rust-explorer-utils → (外部クレート)
```

この設計により：

- **再利用性**: 各クレートは独立して他のプロジェクトで使用可能
- **循環依存を回避**: 明確なレイヤー構造
- **テスト可能な構造**: 各クレートが独立してテスト可能
- **モジュール間の責務を明確に分離**: 単一責任原則の徹底

## 拡張ポイント

1. **UIコンポーネント**: `crates/ui/src/components/`配下に新しいコンポーネントを追加
2. **コア機能**: `crates/core/src/`配下に新しいビジネスロジックモジュールを追加
3. **設定項目**: `crates/config/src/settings.rs`に新しい設定項目を追加
4. **エラー型**: `crates/utils/src/error.rs`に新しいエラー型を追加
5. **新しいクレート**: 必要に応じて`crates/`配下に新しい機能クレートを追加

## 設計原則

- **単一責任原則**: 各モジュールは明確に定義された単一の責務を持つ
- **依存関係逆転**: 上位モジュールは下位モジュールに依存し、逆はない
- **テスト可能性**: 各モジュールは独立してテスト可能
- **拡張性**: 新機能の追加が既存コードに影響を与えにくい構造
