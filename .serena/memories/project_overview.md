# TaskNag プロジェクト概要

## プロジェクトの目的
TaskNagは「口うるさくて世話焼きなWindowsデスクトップタスク管理アプリケーション」です。
ユーザーが忘れがちなタスクを積極的にリマインドする「おせっかい」なタスク管理アプリです。

## 主要機能
- 🗣️ プロアクティブな通知（3段階の通知レベル）
- 🏠 システムトレイ常駐
- 🌳 階層タスク管理（親子関係）
- 🔄 定期タスク（繰り返しタスクの自動生成）
- 📊 カンバンボード（Inbox → Todo → In Progress → Done）
- 🤖 将来的にLLM統合予定

## プロジェクト構造
```
TaskNag/
├── src/                  # Reactフロントエンドコード
├── src-tauri/           # Rustバックエンドコード
│   ├── src/             # Rustソースコード
│   ├── Cargo.toml       # Rust依存関係
│   └── tauri.conf.json  # Tauriアプリ設定
├── .kiro/               # Kiro開発スペック（Git非追跡）
├── .claude/             # Claude設定ファイル
├── package.json         # Node.js依存関係
├── tailwind.config.js   # Tailwind CSS設定
├── tsconfig.json        # TypeScript設定
└── README.md           # プロジェクト詳細

```

## 開発フェーズ
現在Phase 1（MVP）を実行中：
- [x] プロジェクトセットアップ  
- [ ] 基本的なタスクCRUD機能
- [ ] システムトレイ統合
- [ ] カンバンボードUI

## ターゲットプラットフォーム
Windows 10/11 デスクトップアプリケーション