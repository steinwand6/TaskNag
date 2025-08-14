# TaskCard.tsx - タスクカードコンポーネント解説

## 📋 概要

`TaskCard.tsx`はTaskNagアプリケーションの中核となるUIコンポーネントで、個別のタスクを視覚的に表示するカードインターフェースを提供します。ドラッグ&ドロップ、複数の操作手段、通知設定の可視化など、豊富なインタラクション機能を備えています。

---

## 🏗️ ファイル構造

### インポート構成
```typescript
// React Core
import React from 'react';
import { useNavigate } from 'react-router-dom';

// 型定義
import { Task, TaskStatus } from '../types/Task';

// 状態管理
import { useTaskStore } from '../stores/taskStore';

// 関連コンポーネント
import { EditTaskModal } from './EditTaskModal';
```

### インターフェース定義
```typescript
interface TaskCardProps {
  task: Task;                           // 表示するタスクデータ
  onDragStart?: (taskId: string) => void; // ドラッグ開始イベント
  onDragEnd?: () => void;               // ドラッグ終了イベント
}
```

---

## 🎯 主要機能

### 1. 通知設定の可視化
```typescript
const getNotificationDisplay = (task: Task) => {
  // 通知なしの場合
  if (!task.notificationSettings || task.notificationSettings.notificationType === 'none') {
    return null;
  }
  
  const { notificationSettings } = task;
  
  // 期日ベース通知
  if (notificationSettings.notificationType === 'due_date_based' && task.dueDate) {
    return (
      <span className="text-xs text-blue-600">
        🔔 期日{notificationSettings.daysBefore}日前
      </span>
    );
  }
  
  // 定期通知
  else if (notificationSettings.notificationType === 'recurring') {
    const dayNames = ['日', '月', '火', '水', '木', '金', '土'];
    const days = notificationSettings.daysOfWeek?.map(d => dayNames[d]).join('') || '';
    return (
      <span className="text-xs text-green-600">
        🔔 {days}
      </span>
    );
  }
  
  return null;
};
```

**通知表示の特徴:**
- **期日ベース**: 🔔 期日X日前 (青色テキスト)
- **定期通知**: 🔔 月火水木金 (緑色テキスト)
- **通知なし**: 通知なし (灰色テキスト)

### 2. 視覚的優先度表示
```typescript
const getBorderColor = () => {
  // 通知設定なしの場合
  if (!task.notificationSettings || task.notificationSettings.notificationType === 'none') {
    return 'border-l-gray-300';
  }
  
  // 通知レベルに応じた色分け
  switch (task.notificationSettings.level) {
    case 3: return 'border-l-red-500';    // 高優先度 (赤)
    case 2: return 'border-l-yellow-500'; // 中優先度 (黄)
    case 1: return 'border-l-blue-500';   // 低優先度 (青)
    default: return 'border-l-gray-300';  // デフォルト (灰)
  }
};
```

**視覚的階層:**
- **レベル3 (高)**: 赤い左ボーダー
- **レベル2 (中)**: 黄色い左ボーダー
- **レベル1 (低)**: 青い左ボーダー
- **なし**: 灰色の左ボーダー

### 3. 日付フォーマッティング
```typescript
const formatDate = (date?: Date) => {
  if (!date) return '';
  return new Intl.DateTimeFormat('ja-JP', {
    month: 'short',
    day: 'numeric',
  }).format(date);
};
```

**日付表示仕様:**
- **ロケール**: 日本語 (ja-JP)
- **フォーマット**: "1月15日" 形式
- **アイコン**: 📅 カレンダー絵文字付き

---

## 🖱️ インタラクション設計

### 1. 複数の操作手段
```typescript
// ダブルクリック → 編集モーダル
const handleDoubleClick = () => {
  setEditTask(task);
};

// 右クリック → 詳細ページへナビゲート
const handleRightClick = (e: React.MouseEvent) => {
  e.preventDefault();
  navigate(`/task/${task.id}`);
};

// Ctrl+クリック → 詳細ページへナビゲート
const handleCtrlClick = (e: React.MouseEvent) => {
  if (e.ctrlKey || e.metaKey) {
    e.preventDefault();
    navigate(`/task/${task.id}`);
  }
};
```

**操作手段マッピング:**
- **ダブルクリック**: インライン編集 (EditTaskModal)
- **右クリック**: 詳細ビューへ遷移
- **Ctrl+クリック**: 詳細ビューへ遷移
- **ドラッグ**: ステータス変更

### 2. 高度なドラッグ&ドロップ実装
```typescript
const handleMouseDown = (e: React.MouseEvent) => {
  if (e.button !== 0) return; // 左クリックのみ
  
  const startPos = { x: e.clientX, y: e.clientY };
  let dragStarted = false;
  
  const handleMouseMove = (e: MouseEvent) => {
    const deltaX = Math.abs(e.clientX - startPos.x);
    const deltaY = Math.abs(e.clientY - startPos.y);
    
    // 3px以上の移動でドラッグ開始
    if ((deltaX > 3 || deltaY > 3) && !dragStarted) {
      dragStarted = true;
      setIsDragging(true);
      onDragStart?.(task.id);
    }
  };
  
  const handleMouseUp = (e: MouseEvent) => {
    if (dragStarted) {
      // ドロップ位置の要素を特定
      const elementUnderMouse = document.elementFromPoint(e.clientX, e.clientY);
      
      // data-status属性を持つ親要素を探索
      let columnElement = elementUnderMouse;
      let status = null;
      
      while (columnElement && columnElement !== document.body) {
        if (columnElement.hasAttribute && columnElement.hasAttribute('data-status')) {
          status = columnElement.getAttribute('data-status') as TaskStatus;
          break;
        }
        columnElement = columnElement.parentElement;
      }
      
      // ステータス変更を実行
      if (status) {
        moveTask(task.id, status);
      }
      
      setIsDragging(false);
      onDragEnd?.();
    }
    
    // イベントリスナーをクリーンアップ
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };
  
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);
};
```

**ドラッグ&ドロップの特徴:**
- **3pxの移動閾値**: 意図しないドラッグを防止
- **動的ドロップ判定**: マウス位置から適切なカラムを特定
- **視覚的フィードバック**: ドラッグ中の透明度変更
- **DOM探索**: data-status属性によるターゲット特定

---

## 🎨 UI構造とスタイリング

### レスポンシブカードデザイン
```typescript
<div 
  className={`bg-white p-3 rounded-lg border-l-4 ${getBorderColor()} shadow-sm hover:shadow-md transition-shadow cursor-move select-none ${
    isDragging ? 'opacity-50 scale-105' : ''
  }`}
  // ... イベントハンドラー
>
```

**スタイリング仕様:**
- **ベース**: 白背景、角丸、左ボーダー
- **インタラクション**: ホバー時シャドウ増加
- **ドラッグ状態**: 透明度50%、5%拡大
- **カーソル**: move (移動可能を示唆)

### カード内レイアウト
```typescript
{/* ヘッダー部分 */}
<div className="flex justify-between items-start mb-2">
  <h4 className="font-medium text-gray-900 text-sm leading-tight">
    {task.title}
  </h4>
  <button onClick={handleDelete} className="text-gray-400 hover:text-red-500">
    ×
  </button>
</div>

{/* 説明文 */}
{task.description && (
  <p className="text-gray-600 text-xs mb-2 line-clamp-2">
    {task.description}
  </p>
)}

{/* フッター部分 */}
<div className="flex justify-between items-center text-xs text-gray-500">
  {getNotificationDisplay(task) || <span className="text-gray-400">通知なし</span>}
  {task.dueDate && (
    <span className="text-orange-600">
      📅 {formatDate(task.dueDate)}
    </span>
  )}
</div>
```

**レイアウト構成:**
1. **ヘッダー**: タイトル + 削除ボタン
2. **本文**: 説明文 (2行制限、line-clamp)
3. **フッター**: 通知設定 + 期日

---

## 🔄 状態管理

### ローカル状態
```typescript
const [editTask, setEditTask] = React.useState<Task | null>(null);
const [isDragging, setIsDragging] = React.useState(false);
```

**状態の責任範囲:**
- **`editTask`**: 編集モーダルの開閉制御
- **`isDragging`**: ドラッグ操作中の視覚効果

### グローバル状態連携
```typescript
const { deleteTask, moveTask } = useTaskStore();
const navigate = useNavigate();
```

**外部依存:**
- **`deleteTask`**: タスク削除アクション
- **`moveTask`**: ステータス変更アクション
- **`navigate`**: ルーティング制御

---

## 🎛️ 条件分岐とレンダリング

### 動的コンテンツ表示
```typescript
{/* 説明文の条件付き表示 */}
{task.description && (
  <p className="text-gray-600 text-xs mb-2 line-clamp-2">
    {task.description}
  </p>
)}

{/* 期日の条件付き表示 */}
{task.dueDate && (
  <span className="text-orange-600">
    📅 {formatDate(task.dueDate)}
  </span>
)}
```

### モーダルの制御
```typescript
<EditTaskModal
  isOpen={editTask !== null}
  onClose={() => setEditTask(null)}
  task={editTask}
/>
```

**表示制御ロジック:**
- **editTask が null 以外**: モーダル表示
- **editTask が null**: モーダル非表示

---

## 🎯 パフォーマンス最適化

### 1. イベントハンドラーの最適化
```typescript
// 現在: 毎回新しい関数作成
onClick={(e) => {
  e.stopPropagation();
  deleteTask(task.id);
}}

// 最適化案: useCallback使用
const handleDelete = React.useCallback((e: React.MouseEvent) => {
  e.stopPropagation();
  deleteTask(task.id);
}, [deleteTask, task.id]);
```

### 2. 条件付きレンダリングの改善
```typescript
// 現在: 毎回関数実行
{getNotificationDisplay(task) || <span className="text-gray-400">通知なし</span>}

// 最適化案: useMemo使用
const notificationDisplay = React.useMemo(() => 
  getNotificationDisplay(task), 
  [task.notificationSettings, task.dueDate]
);
```

### 3. クラス名の動的生成最適化
```typescript
// 最適化案: useMemo使用
const cardClasses = React.useMemo(() => 
  `bg-white p-3 rounded-lg border-l-4 ${getBorderColor()} shadow-sm hover:shadow-md transition-shadow cursor-move select-none ${
    isDragging ? 'opacity-50 scale-105' : ''
  }`,
  [task.notificationSettings, isDragging]
);
```

---

## 🧪 テスト観点

### 1. 単体テスト項目
- [ ] タスクデータの正しい表示
- [ ] 通知設定の可視化
- [ ] 期日フォーマットの確認
- [ ] ボーダー色の条件分岐

### 2. インタラクションテスト
- [ ] ダブルクリックでの編集モーダル開閉
- [ ] 右クリックでのナビゲーション
- [ ] Ctrl+クリックでのナビゲーション
- [ ] 削除ボタンの動作

### 3. ドラッグ&ドロップテスト
- [ ] ドラッグ開始の閾値確認
- [ ] ドロップ位置の正確な判定
- [ ] 視覚的フィードバックの動作
- [ ] ステータス変更の正確性

### 4. エッジケーステスト
- [ ] 説明なしタスクの表示
- [ ] 期日なしタスクの表示
- [ ] 通知設定なしタスクの表示
- [ ] 長いタイトル・説明のクリッピング

---

## 🚀 将来の拡張予定

### 1. 機能拡張
- **プログレスバー**: タスク進捗の視覚表示
- **サブタスク表示**: 子タスク数のインジケーター
- **タグ表示**: カテゴリ・ラベルの可視化
- **アバター表示**: 担当者情報

### 2. インタラクション改善
- **キーボードナビゲーション**: Tab/Enter キー対応
- **マルチセレクト**: Ctrl+クリックでの複数選択
- **コンテキストメニュー**: 右クリックメニューの拡張
- **ホットキー対応**: 削除・編集のショートカット

### 3. 視覚的改善
- **アニメーション**: ステータス変更時のトランジション
- **テーマ対応**: ダーク/ライトモード
- **カスタムカラー**: ユーザー定義色
- **アクセシビリティ**: 高コントラストモード

---

## 📝 開発者向けノート

### パフォーマンス考慮事項
- **大量タスク**: 仮想化による描画最適化が必要
- **ドラッグ処理**: GPU アクセラレーション活用
- **メモリリーク**: イベントリスナーの適切なクリーンアップ

### アクセシビリティ
- **ARIA 属性**: ドラッグ可能要素のラベル付け
- **キーボード操作**: Tab インデックスの設定
- **スクリーンリーダー**: 重要情報の読み上げ対応

### 保守性
- **関数分離**: イベントハンドラーの外部化
- **型安全性**: TaskCardProps の厳密な定義
- **テストカバレッジ**: 全インタラクションの網羅