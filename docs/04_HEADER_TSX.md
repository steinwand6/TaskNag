# Header.tsx - ヘッダーコンポーネント解説

## 📋 概要

`Header.tsx`はTaskNagアプリケーションの最上部に配置されるヘッダーコンポーネントです。アプリケーションのブランディング、主要操作ボタン、表示切り替え機能を提供する、ユーザーインターフェースの中核となる要素です。

---

## 🏗️ ファイル構造

### インポート構成
```typescript
import React from 'react';
```

**特徴:**
- **軽量設計**: React以外の外部依存なし
- **純粋コンポーネント**: プロパティのみに依存
- **関数型コンポーネント**: 状態を持たない設計

### インターフェース定義
```typescript
interface HeaderProps {
  isLoading: boolean;        // ローディング状態
  onNewTask: () => void;     // 新規タスク作成イベント
  onRefresh: () => void;     // データ再読み込みイベント
  showDone: boolean;         // 完了タスク表示状態
  onToggleDone: () => void;  // 完了タスク表示切り替え
}
```

**プロパティ設計原則:**
- **単一責任**: 各プロパティは1つの機能を制御
- **明確な命名**: 機能が直感的に理解可能
- **型安全性**: TypeScriptによる厳密な型定義

---

## 🎯 主要機能

### 1. ブランディング・アイデンティティ
```typescript
<div className="flex items-center">
  <h1 className="text-2xl font-bold text-gray-900">
    TaskNag 🗣️
  </h1>
  <p className="ml-3 text-sm text-gray-500">
    口うるさいタスク管理
  </p>
</div>
```

**ブランディング要素:**
- **アプリ名**: TaskNag + 絵文字でキャラクター表現
- **キャッチフレーズ**: "口うるさいタスク管理"
- **視覚的階層**: サイズと色で情報優先度を表現

### 2. 完了タスク表示切り替え
```typescript
<button 
  onClick={onToggleDone}
  className={`px-3 py-2 text-sm rounded-md transition-colors ${
    showDone 
      ? 'bg-green-100 text-green-800 border border-green-300' 
      : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
  }`}
  disabled={isLoading}
>
  {showDone ? '✅ DONE表示中' : '✅ DONE'}
</button>
```

**状態による変化:**
- **非表示時**: 灰色背景、"✅ DONE"
- **表示時**: 緑色背景、"✅ DONE表示中"
- **ローディング時**: ボタン無効化

### 3. 新規タスク作成
```typescript
<button 
  onClick={onNewTask}
  className="btn-primary"
  disabled={isLoading}
>
  + 新規タスク
</button>
```

**設計特徴:**
- **プライマリアクション**: 最も重要な操作として強調
- **CSS クラス**: `btn-primary` で一貫したスタイリング
- **アイコン**: "+" で直感的な追加操作を表現

### 4. データ更新機能
```typescript
<button 
  className="btn-secondary"
  onClick={onRefresh}
  disabled={isLoading}
>
  {isLoading ? '⏳' : '🔄'} 更新
</button>
```

**動的アイコン表示:**
- **通常時**: 🔄 (回転矢印) + "更新"
- **ローディング時**: ⏳ (砂時計) + "更新"
- **無効化**: ローディング中はクリック不可

---

## 🎨 UI・レイアウト設計

### レスポンシブレイアウト
```typescript
<header className="bg-white shadow-sm border-b border-gray-200">
  <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div className="flex justify-between items-center py-4">
      {/* 左側: ブランディング */}
      <div className="flex items-center">...</div>
      
      {/* 右側: 操作ボタン */}
      <div className="flex items-center space-x-4">...</div>
    </div>
  </div>
</header>
```

**レスポンシブ仕様:**
- **最大幅**: 7xl (1280px) での中央揃え
- **パディング**: 
  - モバイル: 4 (16px)
  - タブレット: 6 (24px)  
  - デスクトップ: 8 (32px)
- **レイアウト**: Flexbox による左右配置

### 視覚的階層
```scss
// 擬似CSS表現
.header {
  background: white;
  shadow: subtle;           // 軽いドロップシャドウ
  border-bottom: gray-200;  // 境界線
}

.title {
  font-size: 2xl;           // 24px
  font-weight: bold;
  color: gray-900;          // 濃い灰色
}

.subtitle {
  font-size: sm;            // 14px
  color: gray-500;          // 中間灰色
  margin-left: 12px;
}
```

**色彩設計:**
- **背景**: 白色 (清潔感)
- **タイトル**: 濃い灰色 (可読性)
- **サブタイトル**: 中間灰色 (階層表現)
- **境界**: 薄い灰色 (セクション分離)

---

## 🔄 状態連動・インタラクション

### プロパティ駆動レンダリング
```typescript
// 完了タスク表示ボタンの状態制御
className={`... ${
  showDone 
    ? 'bg-green-100 text-green-800 border border-green-300'  // アクティブ
    : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'  // 非アクティブ
}`}

// ボタンテキストの動的変更
{showDone ? '✅ DONE表示中' : '✅ DONE'}
```

### ローディング状態の統合制御
```typescript
// 全ボタンの一括無効化
disabled={isLoading}

// 更新ボタンのアイコン変更
{isLoading ? '⏳' : '🔄'} 更新
```

**無効化対象:**
- 完了タスク表示切り替えボタン
- 新規タスク作成ボタン
- データ更新ボタン

---

## 🎛️ アクセシビリティ設計

### キーボードナビゲーション
```typescript
// 暗黙的なTabIndex順序
<button>DONE表示切り替え</button>    // Tab 1
<button>新規タスク</button>         // Tab 2  
<button>更新</button>              // Tab 3
```

### セマンティックHTML
```html
<header>                    <!-- ランドマーク要素 -->
  <h1>TaskNag 🗣️</h1>      <!-- 主見出し -->
  <p>口うるさいタスク管理</p>  <!-- 説明文 -->
  <button>...</button>      <!-- 操作ボタン -->
</header>
```

**改善提案:**
```typescript
// ARIA属性の追加案
<button 
  onClick={onToggleDone}
  aria-label={`完了タスクを${showDone ? '非表示' : '表示'}にする`}
  aria-pressed={showDone}
>
  {showDone ? '✅ DONE表示中' : '✅ DONE'}
</button>
```

---

## 🎯 パフォーマンス最適化

### 1. メモ化の検討
```typescript
// 現在: プロパティ変更時に毎回再レンダリング
export const Header: React.FC<HeaderProps> = ({ ... }) => {

// 最適化案: React.memo使用
export const Header = React.memo<HeaderProps>(({ 
  isLoading, onNewTask, onRefresh, showDone, onToggleDone 
}) => {
  // コンポーネント本体
}, (prevProps, nextProps) => {
  // カスタム比較関数（オプション）
  return prevProps.isLoading === nextProps.isLoading &&
         prevProps.showDone === nextProps.showDone;
});
```

### 2. クラス名の最適化
```typescript
// 現在: 毎回文字列結合
className={`px-3 py-2 text-sm rounded-md transition-colors ${
  showDone 
    ? 'bg-green-100 text-green-800 border border-green-300' 
    : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
}`}

// 最適化案: useMemo使用
const toggleButtonClasses = React.useMemo(() => 
  `px-3 py-2 text-sm rounded-md transition-colors ${
    showDone 
      ? 'bg-green-100 text-green-800 border border-green-300' 
      : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-gray-200'
  }`,
  [showDone]
);
```

---

## 🧪 テスト観点

### 1. レンダリングテスト
- [ ] ブランディング要素の正しい表示
- [ ] 全ボタンの初期状態確認
- [ ] レスポンシブレイアウトの動作

### 2. 状態変化テスト
- [ ] `showDone` プロパティによるボタン表示変更
- [ ] `isLoading` プロパティによるボタン無効化
- [ ] 更新ボタンのアイコン変化

### 3. インタラクションテスト
- [ ] 各ボタンクリック時のイベント発火
- [ ] ローディング中のボタン押下防止
- [ ] キーボードナビゲーションの動作

### 4. アクセシビリティテスト
- [ ] スクリーンリーダーでの読み上げ
- [ ] キーボードのみでの操作
- [ ] 高コントラストモードでの視認性

---

## 🚀 将来の拡張予定

### 1. 機能拡張
- **検索機能**: タスク検索用の入力フィールド
- **フィルター**: 優先度・期日による絞り込み
- **ソート**: 並び順切り替えボタン
- **設定**: ユーザー設定へのアクセス

### 2. 通知統合
- **タスクカウンター**: リアルタイムタスク数表示
- **通知インジケーター**: 期日が近いタスクのアラート
- **プログレス表示**: 全体進捗のバー表示

### 3. 国際化 (i18n)
```typescript
// 多言語対応案
interface HeaderTexts {
  title: string;
  subtitle: string;
  newTask: string;
  refresh: string;
  showDone: string;
  hideDone: string;
}

const texts: Record<string, HeaderTexts> = {
  'ja': {
    title: 'TaskNag 🗣️',
    subtitle: '口うるさいタスク管理',
    newTask: '+ 新規タスク',
    refresh: '更新',
    showDone: '✅ DONE',
    hideDone: '✅ DONE表示中'
  },
  'en': {
    title: 'TaskNag 🗣️',
    subtitle: 'Nagging Task Manager',
    newTask: '+ New Task',
    refresh: 'Refresh',
    showDone: '✅ DONE',
    hideDone: '✅ SHOWING DONE'
  }
};
```

---

## 📝 開発者向けノート

### CSS クラス依存関係
```scss
// グローバルCSS定義が必要
.btn-primary {
  @apply bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600 disabled:opacity-50;
}

.btn-secondary {
  @apply bg-gray-200 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-300 disabled:opacity-50;
}
```

### プロパティ設計パターン
- **状態プロパティ**: `isLoading`, `showDone`
- **イベントハンドラー**: `onNewTask`, `onRefresh`, `onToggleDone`
- **命名規則**: `on` + 動作 + `イベント名`

### 保守性向上のポイント
- **プロパティの型安全性**: 厳密なインターフェース定義
- **コンポーネント分離**: 単一責任の原則
- **スタイルクラスの統一**: 再利用可能なCSSクラス体系