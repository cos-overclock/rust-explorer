# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

rust-explorerは、Rust + floemを使用したマルチプラットフォーム対応のモダンなタブ型ファイラーアプリケーションです。ワークスペース構造で4つの独立したクレートに分割され、再利用可能な設計となっています。

Files app（Windows 11）を参考にした直感的なUIと、高速なファイル操作、タブ・ペイン分割機能を提供する個人開発のOSSプロジェクトです。

## 技術スタック

- **言語**: Rust（Edition 2024）
- **GUI フレームワーク**: floem 0.2
- **エラーハンドリング**: thiserror 2.0
- **設定管理**: serde + serde_json
- **ログ**: tracing + tracing-subscriber
- **目標**: Windows/macOS/Linuxクロスプラットフォーム対応

## ワークスペース構造

このプロジェクトは4つの独立したクレートで構成されています：

- **rust-explorer-utils**: 共通エラーハンドリング（thiserror 2.0使用）
- **rust-explorer-config**: アプリケーション設定管理とstate永続化（JSON形式）
- **rust-explorer-core**: ビジネスロジック（filesystem API、state管理、event system）
- **rust-explorer-ui**: ユーザーインターフェイス（floem使用、reactive state integration）

依存関係: `ui → core + config + utils`, `core → utils`, `config → utils`

## 実装済み機能

### コア機能（rust-explorer-core）

- **FileSystemApi**: 非同期ファイル操作（読み取り、作成、削除、移動、コピー）
- **StateManager**: アプリケーション状態管理（タブ、ウィンドウ、UI状態）
- **EventManager**: イベント駆動アーキテクチャ

### 設定・永続化（rust-explorer-config）

- **Settings**: JSON設定ファイル管理
- **StatePersistenceManager**: 状態の自動保存・復元（バックアップ機能付き）

### UI統合（rust-explorer-ui）

- **ReactiveStateManager**: floem RwSignalを使ったリアクティブ状態管理
- **ResponsiveLayoutManager**: レスポンシブレイアウトシステム
- **MainWindow**: メインアプリケーションウィンドウ
- **基本コンポーネント**: Header, MainContent, StatusBar, ErrorDialog

## 必須開発コマンド

### ビルドとチェック

```bash
# ワークスペース全体のコンパイルチェック
cargo check

# 個別クレートのチェック
cargo check -p rust-explorer-utils
cargo check -p rust-explorer-config
cargo check -p rust-explorer-core
cargo check -p rust-explorer-ui

# アプリケーション実行
cargo run
```

### 必須：テストとフォーマット

```bash
# 全テスト実行（必須）
cargo test

# 個別クレートのテスト
cargo test -p rust-explorer-utils
cargo test -p rust-explorer-config
cargo test -p rust-explorer-core
cargo test -p rust-explorer-ui

# 単一テスト実行例
cargo test -p rust-explorer-utils test_app_error

# 必須：コードフォーマット（完了前に必ず実行）
cargo fmt --all

# 必須：Lintチェック
cargo clippy --all-targets --all-features -- -D warnings

# ドキュメント生成
cargo doc --workspace --no-deps
```

## 実装ルール

### 必須要件

1. **全機能にテスト作成**: 新機能実装時は対応するテストを必ず作成
2. **フォーマット適用**: 実装完了時は必ず`cargo fmt --all`を実行
3. **Clippy通過**: 警告なしで`cargo clippy`が通ること
4. **エラーハンドリング**: thiserror 2.0を使用した統一エラー処理

### テストルール

- 各クレートは独立してテスト可能
- ユニットテストは対象モジュール内に`#[cfg(test)]`で記述
- 統合テストは`tests/`ディレクトリに配置
- モック使用時はトレイトベースの設計を活用

### コード品質

- 外部crateは新しいバージョンを使用
- 循環依存を回避する明確なレイヤー構造
- 各クレートは単一責任原則に従う
- パブリックAPIにはドキュメントコメント必須

## 設計方針

- **再利用性**: 各クレートは独立して他プロジェクトで使用可能
- **テスト容易性**: 依存注入とトレイトベースの設計
- **クロスプラットフォーム**: OS固有機能を除く共通実装
- **モダンUI**: Files appを参考にした直感的デザイン
- **JSON設定**: レジストリ不使用のポータブル運用

## アーキテクチャパターン

### 状態管理アーキテクチャ

- **StateManager**: Arc<RwLock<AppState>>を使ったスレッドセーフな状態管理
- **ReactiveStateManager**: floem RwSignalとの橋渡し、UIリアクティブ更新
- **StatePersistenceManager**: JSON形式での自動保存・復元（バックアップ5世代管理）
- **イベント駆動**: StateChangeEventによる状態変更の通知システム

### エラーハンドリングパターン

- **統一型**: `rust-explorer-utils::AppError`（thiserror 2.0使用）
- **Result型**: すべての操作で適切なエラー処理
- **ログ統合**: tracingでstructured logging

### 非同期処理パターン

- **ファイルシステム**: tokio::fs使用の非同期I/O
- **キャッシュ**: LRUキャッシュによる高速化
- **UI**: floem reactive systemとの連携

### テストパターン

- **ユニットテスト**: 各モジュール内で`#[cfg(test)]`
- **統合テスト**: `tests/`ディレクトリ
- **モック**: トレイトベース設計による依存注入
- **テスト数**: 現在40+テストケース実装済み

## 開発プロセス

### Issue駆動開発

- GitHubのIssue管理でタスク追跡
- 機能単位でブランチ作成（`issue-N-feature-name`）
- PR作成時は詳細な説明とテスト結果を記載

### 品質管理フロー

1. 実装 → 2. テスト作成 → 3. `cargo test` → 4. `cargo fmt --all` → 5. `cargo clippy` → 6. PR作成

### 最近の開発状況

- Issue #8-13: 基盤システム実装完了
- Issue #20: ファイル・フォルダー表示画面（次の実装対象）

## 主要な実装パターン

1. **エラー処理**: `rust-explorer-utils::AppError`で統一
2. **設定管理**: `rust-explorer-config`でJSON永続化とstate management
3. **ビジネスロジック**: `rust-explorer-core`でUI非依存、async/await多用
4. **UI統合**: `rust-explorer-ui`でfloem reactive systemと連携
5. **状態フロー**: StateManager → ReactiveStateManager → floem RwSignal → UI更新
