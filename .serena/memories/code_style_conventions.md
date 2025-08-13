# TaskNag コードスタイル・規約

## TypeScript/React規約

### ファイル命名規則
- Reactコンポーネント: `PascalCase.tsx` (例: `App.tsx`)
- その他のTS/JSファイル: `camelCase.ts` 
- 設定ファイル: `kebab-case` (例: `tailwind.config.js`)

### TypeScript設定
- **Strictモード**: 有効
- **未使用変数/パラメータ**: エラー扱い
- **fall-throughケース**: エラー扱い
- **JSX**: `react-jsx`形式

### パス解決
```typescript
// エイリアス設定済み
import Component from '@/components/Component';
import { useStore } from '@/stores/taskStore';
import { TaskType } from '@/types/Task';
```

### ESLint設定
- **TypeScript ESLint**: 有効
- **React Hooks**: ルール適用
- **React Refresh**: 開発時適用

## Rust規約

### プロジェクト構造
```rust
// lib.rs - メインライブラリエントリポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { ... }

// main.rs - バイナリエントリポイント  
fn main() {
    app_lib::run();
}
```

### Cargo.toml設定
- **Edition**: 2021
- **Rust version**: 1.77.2+
- **Crate types**: staticlib, cdylib, rlib

## CSS/Tailwind規約

### カスタムカラーパレット
```javascript
colors: {
  'task-bg': '#f8fafc',
  'status-inbox': '#64748b',
  'status-todo': '#3b82f6', 
  'status-progress': '#8b5cf6',
  'status-done': '#10b981',
  'priority-{low|medium|high|critical}': '...'
}
```

### アニメーション
- **fade-in**: 0.2s ease-in-out
- **slide-up**: 0.3s ease-out

### クラス命名
- Tailwindユーティリティクラス優先
- カスタムクラス: `btn-primary`, `btn-secondary`

## ファイル組織
- **フロントエンド**: `src/` 配下に機能別フォルダ
- **バックエンド**: `src-tauri/src/` 配下 
- **設定**: プロジェクトルート
- **Kiro開発管理**: `.kiro/` (Git除外)

## コミット規約
- 日本語コミットメッセージ可
- 機能別ブランチ: `feature/amazing-feature`
- MIT ライセンス適用