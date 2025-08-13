# タスク完了時のチェックリスト

## コード品質確認

### 1. 型チェック・リンティング
```bash
# TypeScript型チェック（エラーゼロ確認）
npx tsc --noEmit

# ESLint実行（警告・エラーゼロ確認）
npx eslint src --ext .ts,.tsx

# Rust clippy実行（警告確認）
cargo clippy --manifest-path=src-tauri/Cargo.toml
```

### 2. ビルドテスト
```bash
# フロントエンドビルド成功確認
npm run build

# Tauriビルド成功確認（時間がかかる場合はオプション）
cargo tauri build
```

## 動作確認

### 3. 開発サーバーテスト
```bash
# 開発モード起動・動作確認
cargo tauri dev

# 主要機能が正常動作するか確認
# - アプリ起動
# - UI表示
# - 基本操作
```

### 4. ホットリロード確認
- ファイル変更時の自動リロード
- コンソールエラーなし

## コミット前確認

### 5. Git ステータス確認
```bash
# 変更ファイル確認
git status

# 差分確認
git diff

# 意図しないファイルが含まれていないか確認
```

### 6. コミットメッセージ
- 変更内容を明確に記述
- 日本語または英語で一貫性保持
- 形式: `機能追加: 新機能の説明` または `Fix: バグ修正内容`

## 特記事項

### Tauri固有の確認点
- **重要**: Rustコンパイルエラーがないか
- **重要**: Tauri設定ファイル(`tauri.conf.json`)の構文エラーなし
- **推奨**: ログレベル適切に設定

### React/TypeScript固有
- **重要**: 型エラーゼロ
- **重要**: ESLintルール違反なし
- **推奨**: 未使用変数・インポートなし

### Tailwind CSS
- **推奨**: カスタムクラス名が規約に準拠
- **推奨**: レスポンシブ対応（必要に応じて）

### MCP統合
- **参考**: Serena MCP使用時はシンボリック検索優先
- **参考**: cratedocs MCP使用でRustドキュメント参照可能

## リリース前最終確認
1. README.md更新（機能追加時）
2. バージョン番号更新（package.json、Cargo.toml）
3. CHANGELOG.md更新（大きな変更時）