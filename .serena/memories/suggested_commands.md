# TaskNag 開発コマンド集

## 必須セットアップコマンド

### 初回環境構築
```bash
# リポジトリクローン
git clone https://github.com/steinwand6/TaskNag.git
cd TaskNag

# Tauri CLI インストール（グローバル）
cargo install tauri-cli

# 依存関係インストール
npm install
```

## 開発コマンド

### 開発サーバー起動
```bash
# Tauri開発モード（推奨）
cargo tauri dev

# フロントエンドのみ開発サーバー
npm run dev
```

### ビルド
```bash
# 開発ビルド（高速）
npm run build

# 本番Tauriビルド
cargo tauri build

# MSIインストーラー生成
cargo tauri build --bundles msi
```

## 開発支援コマンド

### 型チェック・リンティング
```bash
# TypeScript型チェック
npx tsc --noEmit

# ESLint実行
npx eslint src --ext .ts,.tsx

# ESLint修正
npx eslint src --ext .ts,.tsx --fix
```

### Git操作
```bash
# 基本的なGitコマンド
git status
git add .
git commit -m "機能追加: ..."
git push origin feature/branch-name

# ブランチ作成・切り替え
git checkout -b feature/new-feature
git checkout main
```

## Windowsシステムコマンド

### ディレクトリ・ファイル操作
```cmd
# ディレクトリ一覧
dir
ls  # Git Bash使用時

# ファイル検索
findstr /s "検索文字列" *.ts
grep -r "検索文字列" src/  # Git Bash使用時

# ファイル内容表示
type ファイル名.txt
cat ファイル名.txt  # Git Bash使用時
```

### プロセス管理
```cmd
# ポート使用状況確認
netstat -ano | findstr :5173

# タスク終了
taskkill /PID プロセスID /F
```

## デバッグ・テスト

### Tauriデバッグ
```bash
# ログ確認付き開発モード
RUST_LOG=debug cargo tauri dev

# Rustデバッグビルド
cargo build --manifest-path=src-tauri/Cargo.toml
```

### フロントエンドデバッグ
```bash
# Vite詳細ログ
npm run dev -- --debug

# ビルド分析
npm run build -- --mode development
```

## 重要なファイルパス
- **Tauri設定**: `src-tauri/tauri.conf.json`
- **Cargo依存関係**: `src-tauri/Cargo.toml`
- **npm依存関係**: `package.json`
- **TypeScript設定**: `tsconfig.json`
- **Tailwind設定**: `tailwind.config.js`
- **Claude設定**: `.claude/settings.local.json`