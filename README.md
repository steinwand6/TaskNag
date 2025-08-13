# TaskNag 🗣️

> 口うるさくて世話焼きなWindowsデスクトップタスク管理アプリケーション

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Tauri](https://img.shields.io/badge/tauri-2.0+-green.svg)

## 概要

**TaskNag** は、あなたが忘れがちなタスクを積極的にリマインドしてくれる「おせっかい」なタスク管理アプリです。システムトレイに常駐し、期限が近づいたタスクや忘れられたタスクを口うるさくお知らせします。

### 🎯 主な特徴

- 🗣️ **プロアクティブな通知**: 3段階の通知レベル（システム通知、音声付き、アプリ最大化）
- 🏠 **システムトレイ常駐**: 邪魔にならずに常にアクセス可能
- 🌳 **階層タスク管理**: 親子関係のある複雑なタスクも整理
- 🔄 **定期タスク**: 毎日16時の会議など、繰り返しタスクの自動生成
- 📊 **カンバンボード**: Inbox → Todo → In Progress → Done の直感的な管理
- 🤖 **LLM統合**: 将来的に自然言語でのタスク入力とスマート提案

## 技術スタック

- **Backend**: Rust + Tauri
- **Frontend**: React + TypeScript + Vite
- **Database**: SQLite (ローカル埋め込み)
- **UI**: Tailwind CSS
- **State Management**: Zustand

## 開発フェーズ

### Phase 1: MVP (完了予定: 2週間)
- [x] プロジェクトセットアップ
- [ ] 基本的なタスクCRUD機能
- [ ] システムトレイ統合
- [ ] カンバンボードUI

### Phase 2: 通知システム (完了予定: +1週間)
- [ ] 3段階通知レベル実装
- [ ] 期限管理と通知
- [ ] 基本的な定期タスク

### Phase 3: UX強化 (完了予定: +1.5週間)
- [ ] 検索・フィルタ機能
- [ ] タグシステム
- [ ] 子タスク階層管理
- [ ] ホットキー対応

### Phase 4: LLM統合 (将来)
- [ ] 自然言語タスク入力
- [ ] スマート提案機能
- [ ] インタラクティブアシスタント

## 開発セットアップ

### 必要な環境
- Rust 1.70+
- Node.js 18+
- Windows 10/11

### インストール手順

```bash
# リポジトリをクローン
git clone https://github.com/steinwand6/TaskNag.git
cd TaskNag

# Tauri CLIのインストール
cargo install tauri-cli

# 依存関係のインストール
cargo install
npm install

# 開発サーバーの起動
cargo tauri dev
```

## ビルド

```bash
# 本番ビルド
cargo tauri build

# インストーラー生成
cargo tauri build --bundles msi
```

## 使用方法

1. **タスク作成**: [+] ボタンまたは `Ctrl+Shift+T` で新規タスク
2. **ステータス変更**: ドラッグ&ドロップで列間移動
3. **通知設定**: 各タスクで通知レベルを設定
4. **定期タスク**: 繰り返しパターンを設定して自動生成

## コントリビューション

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. Pull Requestを作成

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照

## 作者

- **steinwand6** - *Initial work* - [GitHub](https://github.com/steinwand6)

## 謝辞

- [Tauri](https://tauri.app/) - クロスプラットフォームアプリ開発
- [React](https://reactjs.org/) - UI フレームワーク
- [Rust](https://www.rust-lang.org/) - システムプログラミング言語

---

> 「まだこのREADME読んでないタスクがありますよ！」 - TaskNag より 😤