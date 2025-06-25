# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

rust-explorerは、Rust + floemを使用したマルチプラットフォーム対応のモダンなタブ型ファイラーアプリケーションです。現在は初期開発段階で、floemの基本的なカウンターサンプルから始まっています。

## 技術スタック

- **言語**: Rust（Edition 2024）
- **GUI フレームワーク**: floem 0.2
- **目標**: Windows/macOS/Linuxクロスプラットフォーム対応

## 開発コマンド

### ビルドとチェック

```bash
# 基本的なコンパイルチェック
cargo check

# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# アプリケーション実行
cargo run
```

### テストとフォーマット

```bash
# テスト実行
cargo test

# コード自動フォーマット
cargo fmt

# Lintチェック
cargo clippy
```

## アーキテクチャ

### 現在の構造

- `src/main.rs`: floemを使用したシンプルなカウンターアプリケーション
- まだ基本的なサンプルコードの段階

### 計画されている機能（README.mdより）

- タブ操作とペイン分割
- ファイル/フォルダー操作（コピー、移動、削除等）
- ドラッグ&ドロップ操作
- インデックス型高速検索
- アドオン機能（Rustベース）
- カスタマイズ可能なキーバインド
- テーマ切り替え（ダーク/ライト）
- 全設定のJSON保存

### 設計方針

- モダンなUI（Files appを参考）
- 高速な起動と動作
- ポータブル運用対応（レジストリ不使用）
- OSS公開予定

## 開発時の注意点

- floem 0.2のAPIを使用
- クロスプラットフォーム対応を考慮した実装
- 設定ファイルはJSON形式で管理
- アドオンシステムはRustベースで設計
- 外部crateは可能な限り新しいバージョンを使用する
