# taskStore.ts - Zustand状態管理ストア解説

## 📋 概要

`taskStore.ts`はTaskNagアプリケーションのクライアント側状態管理を担当するZustandストアです。タスクデータの一元管理、バックエンドとの同期、エラーハンドリング、そしてリアクティブなUI更新を提供します。階層タスクの管理とシステムトレイとの統合も担当しています。

---

## 🏗️ ファイル構造

### インポート構成
```typescript
import { create } from 'zustand';
import { TaskStore, CreateTaskRequest, UpdateTaskRequest } from '../types/Task';
import { TaskService } from '../services/taskService';
import { LogService } from '../services/logService';
```

**依存関係:**
- **Zustand**: 軽量状態管理ライブラリ
- **TaskStore**: TypeScript インターフェース定義
- **TaskService**: バックエンドAPI通信層
- **LogService**: ログ記録・エラー追跡

### ストア初期化
```typescript
export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],           // タスクデータ配列
  isLoading: false,    // ローディング状態
  error: null,         // エラー状態
  
  // ... アクション定義
}));
```

**Zustandパターン:**
- **`set`**: 状態更新関数
- **`get`**: 現在状態取得関数
- **型安全性**: TaskStore インターフェースによる制約

---

## 🔄 非同期アクション

### 1. タスク読み込み (loadTasks)
```typescript
loadTasks: async () => {
  set({ isLoading: true, error: null });
  try {
    const tasks = await TaskService.getRootTasks();
    // Convert date strings to Date objects
    const parsedTasks = tasks.map(task => ({
      ...task,
      createdAt: new Date(task.createdAt),
      updatedAt: new Date(task.updatedAt),
      completedAt: task.completedAt ? new Date(task.completedAt) : undefined,
      dueDate: task.dueDate ? new Date(task.dueDate) : undefined,
    }));
    set({ tasks: parsedTasks, isLoading: false });
    // Update system tray title
    TaskService.updateTrayTitle().catch(console.error);
  } catch (error) {
    LogService.error('TaskStore.loadTasks error', error);
    set({ error: error as Error, isLoading: false });
  }
},
```

**特徴的な実装:**
- **ルートタスクのみ**: 親子関係による階層管理
- **日時変換**: RFC3339文字列 → Dateオブジェクト
- **システムトレイ更新**: 非同期でのタスク数反映
- **エラーログ**: LogService による詳細記録

**日時変換パターン:**
```typescript
// Backend (RFC3339) → Frontend (Date)
createdAt: new Date(task.createdAt),
completedAt: task.completedAt ? new Date(task.completedAt) : undefined,
```

### 2. タスク作成 (addTask)
```typescript
addTask: async (taskData) => {
  set({ isLoading: true, error: null });
  try {
    const createRequest: CreateTaskRequest = {
      title: taskData.title,
      description: taskData.description,
      status: taskData.status,
      priority: taskData.priority,
      parentId: taskData.parentId,
      dueDate: taskData.dueDate,
      notificationSettings: taskData.notificationSettings,
    };

    const newTask = await TaskService.createTask(createRequest);
    const parsedTask = {
      ...newTask,
      createdAt: new Date(newTask.createdAt),
      updatedAt: new Date(newTask.updatedAt),
      completedAt: newTask.completedAt ? new Date(newTask.completedAt) : undefined,
      dueDate: newTask.dueDate ? new Date(newTask.dueDate) : undefined,
    };

    // Only add to state if it's a root task (no parent)
    if (!taskData.parentId) {
      set(state => ({
        tasks: [...state.tasks, parsedTask],
        isLoading: false,
      }));
    } else {
      set({ isLoading: false });
    }
    
    // Update system tray title
    TaskService.updateTrayTitle().catch(console.error);
  } catch (error) {
    console.error('TaskStore.addTask error:', error);
    set({ error: error as Error, isLoading: false });
  }
},
```

**階層管理の設計:**
- **ルートタスクのみストアに追加**: `!taskData.parentId`
- **子タスクは非追加**: 親タスクを通じてアクセス
- **状態管理の最適化**: 不要なデータ保持を回避

**データ変換フロー:**
```
Frontend Input → CreateTaskRequest → Backend → Task Response → Parsed Task → Store
```

### 3. タスク更新 (updateTask)
```typescript
updateTask: async (id, updateData) => {
  set({ isLoading: true, error: null });
  try {
    const updateRequest: UpdateTaskRequest = {
      title: updateData.title,
      description: updateData.description,
      status: updateData.status,
      priority: updateData.priority,
      parentId: updateData.parentId,
      dueDate: updateData.dueDate,
      notificationSettings: updateData.notificationSettings,
    };

    const updatedTask = await TaskService.updateTask(id, updateRequest);
    const parsedTask = {
      ...updatedTask,
      createdAt: new Date(updatedTask.createdAt),
      updatedAt: new Date(updatedTask.updatedAt),
      completedAt: updatedTask.completedAt ? new Date(updatedTask.completedAt) : undefined,
      dueDate: updatedTask.dueDate ? new Date(updatedTask.dueDate) : undefined,
    };

    set(state => ({
      tasks: state.tasks.map(task => task.id === id ? parsedTask : task),
      isLoading: false,
    }));
    
    // Update system tray title
    TaskService.updateTrayTitle().catch(console.error);
  } catch (error) {
    console.error('TaskStore.updateTask error:', error);
    set({ error: error as Error, isLoading: false });
  }
},
```

**Immutable更新パターン:**
```typescript
// 配列内要素の部分更新
tasks: state.tasks.map(task => task.id === id ? parsedTask : task)
```

### 4. タスク削除 (deleteTask)
```typescript
deleteTask: async (id) => {
  set({ isLoading: true, error: null });
  try {
    await TaskService.deleteTask(id);
    set(state => ({
      tasks: state.tasks.filter(task => task.id !== id),
      isLoading: false,
    }));
    
    // Update system tray title
    TaskService.updateTrayTitle().catch(console.error);
  } catch (error) {
    console.error('TaskStore.deleteTask error:', error);
    set({ error: error as Error, isLoading: false });
  }
},
```

**削除処理の特徴:**
- **楽観的削除**: バックエンド削除後の状態更新
- **フィルタリング**: 指定ID以外のタスクを保持
- **エラー時の状態保護**: 削除失敗時は元の状態を維持

### 5. タスク移動 (moveTask)
```typescript
moveTask: async (taskId, newStatus) => {
  set({ isLoading: true, error: null });
  try {
    await TaskService.moveTask(taskId, newStatus);
    
    // Find the task and update its status
    const state = get();
    const taskToUpdate = state.tasks.find(task => task.id === taskId);
    if (taskToUpdate) {
      const updatedTask = { ...taskToUpdate, status: newStatus };
      set({
        tasks: state.tasks.map(task => task.id === taskId ? updatedTask : task),
        isLoading: false,
      });
    } else {
      set({ isLoading: false });
    }
    
    // Update system tray title
    TaskService.updateTrayTitle().catch(console.error);
  } catch (error) {
    console.error('TaskStore.moveTask error:', error);
    set({ error: error as Error, isLoading: false });
  }
},
```

**ドラッグ&ドロップ対応:**
- **ステータスのみ更新**: 効率的な部分更新
- **存在確認**: タスクが見つからない場合の安全な処理
- **`get()` 関数**: 現在状態の取得と操作

---

## 🔍 セレクター関数

### ステータス別タスク取得
```typescript
// Get tasks by status
getTasksByStatus: (status) => {
  return get().tasks.filter(task => task.status === status);
},
```

**セレクターの特徴:**
- **リアルタイム計算**: 呼び出し時点での最新状態
- **フィルタリング**: 指定ステータスのタスクのみ抽出
- **依存関係なし**: Zustand内蔵の `get()` 使用

**使用例:**
```typescript
// コンポーネント内での使用
const todoTasks = useTaskStore(state => state.getTasksByStatus('todo'));
const inProgressTasks = useTaskStore(state => state.getTasksByStatus('in_progress'));
```

---

## 🎯 システム統合

### システムトレイ更新
```typescript
// Update system tray title
TaskService.updateTrayTitle().catch(console.error);
```

**統合ポイント:**
- **全CRUD操作後**: タスク数の変更を即座に反映
- **非同期実行**: UI操作をブロックしない
- **エラー無視**: トレイ更新失敗でもアプリ機能は継続

### ログサービス統合
```typescript
LogService.error('TaskStore.loadTasks error', error);
```

**ログ戦略:**
- **詳細エラー**: TaskStore内でのエラー追跡
- **コンソールログ**: 開発時のデバッグ情報
- **ログサービス**: 運用時の監視・分析

---

## 🔄 状態管理パターン

### 楽観的UI更新
```typescript
// 即座にローカル状態を更新、後でサーバーと同期
set(state => ({
  tasks: [...state.tasks, newTask],
  isLoading: false,
}));
```

### エラー境界設計
```typescript
try {
  // 非同期操作
} catch (error) {
  console.error('Operation error:', error);
  set({ error: error as Error, isLoading: false });
}
```

**エラー処理方針:**
- **エラー状態の保存**: UI でのエラー表示用
- **ローディング解除**: エラー時もUI操作を可能に
- **詳細ログ**: 問題診断のための情報記録

### 状態の一貫性保証
```typescript
// 状態更新の原子性
set({ isLoading: true, error: null });  // 操作開始
set({ tasks: newTasks, isLoading: false });  // 成功時
set({ error: error as Error, isLoading: false });  // 失敗時
```

---

## 🎨 データ変換パターン

### Backend ↔ Frontend 変換
```typescript
// バックエンドからの受信データ変換
const parsedTasks = tasks.map(task => ({
  ...task,
  createdAt: new Date(task.createdAt),        // string → Date
  updatedAt: new Date(task.updatedAt),        // string → Date
  completedAt: task.completedAt ? new Date(task.completedAt) : undefined,
  dueDate: task.dueDate ? new Date(task.dueDate) : undefined,
}));

// フロントエンドからの送信データ変換
const createRequest: CreateTaskRequest = {
  title: taskData.title,
  description: taskData.description,
  status: taskData.status,                    // enum → string
  priority: taskData.priority,                // enum → string
  parentId: taskData.parentId,
  dueDate: taskData.dueDate,                  // Date → ISO string
  notificationSettings: taskData.notificationSettings,
};
```

**変換の必要性:**
- **型システム**: TypeScript型とRust型の橋渡し
- **日時処理**: RFC3339とJavaScript Dateの相互変換
- **Null安全性**: Option型とundefinedの適切な変換

---

## 🧪 テスト観点

### 単体テスト項目
- [ ] 各アクション関数の正常動作
- [ ] エラーハンドリングの適切性
- [ ] 状態更新の一貫性
- [ ] セレクター関数の正確性

### 統合テスト項目
- [ ] TaskServiceとの連携
- [ ] システムトレイ更新の確認
- [ ] ログサービスとの統合
- [ ] 日時変換の正確性

### エッジケース
- [ ] 空のタスクリスト
- [ ] ネットワークエラー時の状態
- [ ] 不正なタスクIDでの操作
- [ ] 階層タスクの親子関係

---

## 🚀 パフォーマンス最適化

### 1. メモ化パターン
```typescript
// セレクター関数のメモ化案
import { useMemo } from 'react';

// コンポーネント内で
const todoTasks = useMemo(() => 
  tasks.filter(task => task.status === 'todo'), 
  [tasks]
);
```

### 2. 部分状態購読
```typescript
// 必要な状態のみ購読
const isLoading = useTaskStore(state => state.isLoading);
const error = useTaskStore(state => state.error);

// 複数状態の効率的な購読
const { tasks, isLoading } = useTaskStore(state => ({
  tasks: state.tasks,
  isLoading: state.isLoading,
}));
```

### 3. 非同期処理の最適化
```typescript
// システムトレイ更新の最適化案
let trayUpdateScheduled = false;

const scheduleTrayUpdate = () => {
  if (!trayUpdateScheduled) {
    trayUpdateScheduled = true;
    setTimeout(() => {
      TaskService.updateTrayTitle().catch(console.error);
      trayUpdateScheduled = false;
    }, 100); // 100ms デバウンス
  }
};
```

---

## 🔧 拡張性設計

### 新機能追加パターン
```typescript
// 新しいアクションの追加
export const useTaskStore = create<TaskStore>((set, get) => ({
  // ... 既存の状態とアクション

  // 新しいアクション: バッチ操作
  batchUpdateTasks: async (updates: Array<{id: string, data: Partial<UpdateTaskRequest>}>) => {
    set({ isLoading: true, error: null });
    try {
      const updatedTasks = await TaskService.batchUpdate(updates);
      // 状態更新ロジック
      set({ isLoading: false });
    } catch (error) {
      set({ error: error as Error, isLoading: false });
    }
  },

  // 新しいセレクター: 期日ベース
  getTasksByDueDate: (daysFromNow: number) => {
    const targetDate = new Date();
    targetDate.setDate(targetDate.getDate() + daysFromNow);
    return get().tasks.filter(task => 
      task.dueDate && task.dueDate <= targetDate
    );
  },
}));
```

### 状態構造の拡張
```typescript
// 新しい状態の追加
interface ExtendedTaskStore extends TaskStore {
  filters: {
    status: TaskStatus[];
    priority: TaskPriority[];
    search: string;
  };
  sorting: {
    field: 'createdAt' | 'dueDate' | 'priority';
    direction: 'asc' | 'desc';
  };
  pagination: {
    page: number;
    pageSize: number;
    total: number;
  };
}
```

---

## 📝 開発者向けノート

### Zustand最適化のベストプラクティス
- **小さなストア**: 機能別の分割を検討
- **イミュータブル更新**: spread operatorの活用
- **TypeScript**: 厳密な型定義による安全性

### デバッグ支援
```typescript
// Zustandのdevtools統合
import { devtools } from 'zustand/middleware';

export const useTaskStore = create<TaskStore>()(
  devtools(
    (set, get) => ({
      // ストア定義
    }),
    {
      name: 'task-store', // DevToolsでの識別名
    }
  )
);
```

### パフォーマンス監視
```typescript
// アクション実行時間の測定
const startTime = performance.now();
await TaskService.createTask(createRequest);
const endTime = performance.now();
LogService.info(`Task creation took ${endTime - startTime} milliseconds`);
```