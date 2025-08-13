# Project Structure Steering

## Directory Structure
```
task-manager/
├── .kiro/                    # Kiro仕様管理
│   ├── steering/            # ステアリング文書
│   │   ├── product.md      # プロダクト方針
│   │   ├── tech.md         # 技術方針
│   │   └── structure.md    # 構造方針
│   └── specs/              # 機能仕様
│       └── [feature]/      # 機能別仕様
│           ├── requirements.md
│           ├── design.md
│           └── tasks.md
├── .claude/                 # Claude設定
│   └── commands/           # カスタムコマンド
│       └── kiro/          # Kiroコマンド
├── src/                    # ソースコード
│   ├── components/         # UIコンポーネント
│   ├── pages/             # ページコンポーネント
│   ├── services/          # ビジネスロジック
│   ├── hooks/             # カスタムフック
│   ├── utils/             # ユーティリティ関数
│   ├── types/             # TypeScript型定義
│   ├── styles/            # グローバルスタイル
│   └── config/            # 設定ファイル
├── tests/                  # テストコード
│   ├── unit/              # ユニットテスト
│   ├── integration/       # 統合テスト
│   └── e2e/               # E2Eテスト
├── public/                 # 静的ファイル
├── docs/                   # ドキュメント
└── scripts/               # ビルド・デプロイスクリプト
```

## File Naming Conventions
- Components: PascalCase (e.g., `TaskList.tsx`)
- Utilities: camelCase (e.g., `formatDate.ts`)
- Types: PascalCase with `.types.ts` (e.g., `Task.types.ts`)
- Tests: `[name].test.ts` or `[name].spec.ts`
- Styles: camelCase with `.module.css` (e.g., `taskList.module.css`)

## Code Organization Principles
- Single Responsibility: 各モジュールは単一の責任を持つ
- Separation of Concerns: UI、ロジック、データを分離
- DRY (Don't Repeat Yourself): 重複コードを避ける
- KISS (Keep It Simple, Stupid): シンプルに保つ

## Import Order
1. External libraries
2. Internal modules
3. Components
4. Types
5. Styles
6. Assets

## Component Structure
```typescript
// 1. Imports
import React from 'react';

// 2. Types
interface Props {
  // ...
}

// 3. Component
export const Component: React.FC<Props> = (props) => {
  // 3.1 Hooks
  // 3.2 State
  // 3.3 Effects
  // 3.4 Handlers
  // 3.5 Render
};

// 4. Exports
export default Component;
```

## Git Conventions
- Branch naming: `feature/[name]`, `fix/[name]`, `docs/[name]`
- Commit messages: Conventional Commits形式
  - `feat:` 新機能
  - `fix:` バグ修正
  - `docs:` ドキュメント
  - `style:` コードスタイル
  - `refactor:` リファクタリング
  - `test:` テスト
  - `chore:` その他

## Testing Standards
- Unit tests: 各関数・コンポーネントに対して
- Integration tests: 主要な機能フローに対して
- E2E tests: クリティカルなユーザージャーニーに対して
- Coverage goal: > 80%