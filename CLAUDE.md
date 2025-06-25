# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

rust-explorerは、Rust + floemを使用したマルチプラットフォーム対応のモダンなタブ型ファイラーアプリケーションです。ワークスペース構造で4つの独立したクレートに分割され、再利用可能な設計となっています。

## 技術スタック

- **言語**: Rust（Edition 2024）
- **GUI フレームワーク**: floem 0.2
- **エラーハンドリング**: thiserror 2.0
- **設定管理**: serde + serde_json
- **目標**: Windows/macOS/Linuxクロスプラットフォーム対応

## ワークスペース構造

このプロジェクトは4つの独立したクレートで構成されています：

- **rust-explorer-utils**: 共通エラーハンドリング（thiserror 2.0使用）
- **rust-explorer-config**: アプリケーション設定管理（JSON形式）
- **rust-explorer-core**: ビジネスロジック（UI非依存）
- **rust-explorer-ui**: ユーザーインターフェイス（floem使用）

依存関係: `ui → core + config + utils`, `core → utils`, `config → utils`

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

## 主要な実装パターン

1. エラー処理は`rust-explorer-utils::AppError`で統一
2. 設定は`rust-explorer-config::Settings`でJSON永続化
3. ビジネスロジックは`rust-explorer-core`でUI非依存実装
4. UIコンポーネントは`rust-explorer-ui`で再利用可能設計
