# App.tsx - メインアプリケーションコンポーネント解説

## 📋 概要

`App.tsx`はTaskNagアプリケーションのメインエントリーポイントとなるReactコンポーネントです。カンバンボード形式のタスク管理UIを提供し、アプリケーション全体の状態管理とレイアウトを担当します。

---

## 🏗️ ファイル構造

### インポート構成
```typescript
// React Core
import React from 'react';

// 型定義
import { TaskStatus } from './types/Task';

// 状態管理
import { useTaskStore } from './stores/taskStore';

// UIコンポーネント
import { NewTaskModal } from './components/NewTaskModal';
import { Header } from './components/Header';
import { KanbanColumn } from './components/KanbanColumn';
import { ErrorMessage } from './components/ErrorMessage';
import { LoadingIndicator } from './components/LoadingIndicator';

// 定数・設定
import { STATUS_CONFIG, VISIBLE_STATUSES } from './constants';

// カスタムフック
import { useModal, useDragAndDrop, useNotifications } from './hooks';

// サービス
import { LogService } from './services/logService';
```

---

## 🎯 主要機能

### 1. 状態管理
```typescript
// Zustand Store からの状態取得
const { getTasksByStatus, moveTask, loadTasks, isLoading, error } = useTaskStore();

// ローカル状態管理
const [showDone, setShowDone] = React.useState(false);
```

**責任範囲:**
- グローバルタスク状態の管理
- 完了タスク表示/非表示の切り替え
- ローディング・エラー状態の監視

### 2. カスタムフック統合
```typescript
// モーダル管理
const { isModalOpen, modalInitialStatus, openModal, closeModal } = useModal();

// ドラッグ&ドロップ機能
const dragAndDropHandlers = useDragAndDrop(moveTask);

// 通知機能
const { } = useNotifications();
```

**各フック詳細:**
- **`useModal`**: タスク作成/編集モーダルの開閉状態
- **`useDragAndDrop`**: カンバンボード間のタスク移動
- **`useNotifications`**: システム通知の管理

### 3. ライフサイクル管理
```typescript
React.useEffect(() => {
  LogService.info('アプリ', 'TaskNagアプリケーションが起動しました');
  loadTasks();
}, [loadTasks]);
```

**起動時処理:**
- アプリケーション起動ログの記録
- 既存タスクデータの読み込み
- 初期状態の確立

---

## 🎨 UI構造

### レスポンシブレイアウト
```typescript
// 動的グリッドレイアウト
const displayStatuses = showDone 
  ? [...VISIBLE_STATUSES, 'done' as TaskStatus]
  : VISIBLE_STATUSES;

// CSS Grid設定
<div className={`grid grid-cols-1 gap-6 ${showDone ? 'md:grid-cols-4' : 'md:grid-cols-3'}`}>
```

**レイアウト仕様:**
- **モバイル**: 1カラム (縦並び)
- **デスクトップ (通常)**: 3カラム (Todo, In Progress, Done非表示)
- **デスクトップ (Done表示)**: 4カラム (全ステータス表示)

### コンポーネント階層
```
App
├── Header (ヘッダー・操作パネル)
├── LoadingIndicator (ローディング表示)
├── KanbanColumn × N (ステータス別カラム)
│   └── TaskCard × N (個別タスクカード)
├── NewTaskModal (タスク作成モーダル)
└── ErrorMessage (エラー表示)
```

---

## 🔄 データフロー

### 1. タスクデータの取得と整形
```typescript
const getStatusData = (status: TaskStatus) => {
  const statusTasks = getTasksByStatus(status);
  
  return {
    ...STATUS_CONFIG[status],      // ステータス設定 (色、アイコン等)
    count: statusTasks.length,     // タスク数
    tasks: statusTasks,            // タスクデータ配列
  };
};
```

**処理フロー:**
1. ステータス別にタスクをフィルタリング
2. 設定情報とタスクデータをマージ
3. UIに必要な統計情報を付与

### 2. ドラッグ&ドロップイベント処理
```typescript
<KanbanColumn
  // ... 他のプロパティ
  isDragOver={dragAndDropHandlers.dragOverStatus === status}
  onMouseEnter={dragAndDropHandlers.handleColumnMouseEnter}
  onMouseLeave={dragAndDropHandlers.handleColumnMouseLeave}
  onClick={dragAndDropHandlers.handleColumnClick}
  onDragStart={dragAndDropHandlers.handleDragStart}
  onDragEnd={dragAndDropHandlers.handleDragEnd}
/>
```

**イベント処理:**
- **マウスイベント**: ホバー状態の管理
- **ドラッグイベント**: タスク移動の開始/終了
- **クリックイベント**: カラム選択状態

---

## 🎛️ 条件分岐とレンダリング

### 1. エラーハンドリング
```typescript
if (error) {
  return <ErrorMessage error={error} />;
}
```

**エラー表示優先度:**
- アプリケーションレベルのエラーが発生した場合
- 通常UIの代わりにエラーコンポーネントを表示
- ユーザーに分かりやすいエラーメッセージを提供

### 2. ローディング状態
```typescript
{isLoading && <LoadingIndicator />}
```

**ローディング表示:**
- 非同期処理中にスピナーを表示
- UIの応答性を維持
- ユーザー体験の向上

### 3. モーダル表示制御
```typescript
{isModalOpen && (
  <NewTaskModal
    isOpen={isModalOpen}
    onClose={closeModal}
    initialStatus={modalInitialStatus}
  />
)}
```

**モーダル管理:**
- 条件付きレンダリングによる表示制御
- プロパティ経由での状態連携
- 初期ステータスの動的設定

---

## 🎯 パフォーマンス最適化

### 1. メモ化可能な箇所
```typescript
// 現在: 毎回計算
const displayStatuses = showDone 
  ? [...VISIBLE_STATUSES, 'done' as TaskStatus]
  : VISIBLE_STATUSES;

// 最適化案: useMemo使用
const displayStatuses = React.useMemo(() => 
  showDone ? [...VISIBLE_STATUSES, 'done' as TaskStatus] : VISIBLE_STATUSES,
  [showDone]
);
```

### 2. コールバック最適化
```typescript
// 現在: 毎回新しい関数作成
onNewTask={() => openModal()}

// 最適化案: useCallback使用
const handleNewTask = React.useCallback(() => openModal(), [openModal]);
```

---

## 🔧 設定・定数

### ステータス表示制御
```typescript
// constants/index.ts から取得
STATUS_CONFIG: {
  todo: { name: 'Todo', color: 'blue', icon: '📝' },
  in_progress: { name: 'In Progress', color: 'yellow', icon: '⚡' },
  done: { name: 'Done', color: 'green', icon: '✅' }
}

VISIBLE_STATUSES: ['todo', 'in_progress'] // 'done'は条件付き表示
```

---

## 🧪 テスト観点

### 1. 単体テスト項目
- [ ] 初期レンダリングの確認
- [ ] `showDone`切り替え時のレイアウト変更
- [ ] エラー状態での`ErrorMessage`表示
- [ ] ローディング状態での`LoadingIndicator`表示

### 2. 統合テスト項目
- [ ] タスクデータ取得後の正常表示
- [ ] ドラッグ&ドロップ操作
- [ ] モーダルの開閉操作
- [ ] ヘッダー機能との連携

### 3. E2Eテスト項目
- [ ] アプリケーション起動から基本操作まで
- [ ] レスポンシブデザインの動作確認
- [ ] エラー回復機能

---

## 🚀 将来の拡張予定

### 1. 機能拡張
- **フィルター機能**: 優先度・期日による絞り込み
- **検索機能**: タスクタイトル・内容での検索
- **ソート機能**: 作成日・期日・優先度でのソート

### 2. パフォーマンス改善
- **仮想化**: 大量タスク表示時の最適化
- **遅延読み込み**: 必要時のデータ取得
- **キャッシュ機能**: 頻繁アクセスデータの保持

### 3. アクセシビリティ
- **キーボードナビゲーション**: Tab移動の改善
- **スクリーンリーダー対応**: ARIA属性の追加
- **コントラスト改善**: 視覚的識別性の向上

---

## 📝 開発者向けノート

### コーディング規約
- **TypeScript**: 厳密な型定義の遵守
- **React Hooks**: 適切な依存配列の設定
- **CSS Classes**: Tailwind CSS の utility-first 原則

### デバッグ情報
- **LogService**: アプリ起動時にログ出力
- **Error Boundary**: 予期しないエラーのキャッチ
- **開発者ツール**: React DevTools での状態監視

### 保守性向上
- **コンポーネント分割**: 単一責任の原則
- **カスタムフック**: ロジックの再利用
- **定数の外部化**: magic number の排除