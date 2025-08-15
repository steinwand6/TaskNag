# Implementation Tasks - Notification Browser Actions

## Phase 1: Foundation & Backend (Priority: High)

### Task 1.1: データベーススキーマ拡張
- [ ] tasksテーブルにbrowser_actionsカラム追加（JSON型）
- [ ] マイグレーションファイル作成（20240816_browser_actions.sql）
- [ ] 既存データの互換性確保
- [ ] スキーマバリデーション追加

**Prerequisites**: 通知システム完成
**Estimated Time**: 2時間
**Files**: `src-tauri/migrations/`, `src-tauri/src/models/task.rs`

### Task 1.2: Rustデータモデル定義
- [ ] BrowserAction構造体定義
- [ ] BrowserActionSettings構造体定義
- [ ] BrowserActionError列挙型定義
- [ ] JSON シリアライズ/デシリアライズ実装

**Prerequisites**: Task 1.1
**Estimated Time**: 1時間
**Files**: `src-tauri/src/models/browser_action.rs`

### Task 1.3: URL検証サービス実装
- [ ] URLValidator構造体作成
- [ ] プロトコル検証（http/https）
- [ ] セキュリティフィルタ実装
- [ ] 危険スキーム検出（javascript:, data:, file:）
- [ ] ホスト名バリデーション

**Prerequisites**: Task 1.2
**Estimated Time**: 3時間
**Files**: `src-tauri/src/services/url_validator.rs`

### Task 1.4: ブラウザアクションサービス実装
- [ ] BrowserActionService構造体作成
- [ ] execute_actions非同期メソッド実装
- [ ] システムブラウザ起動機能
- [ ] エラーハンドリングとログ記録
- [ ] タイムアウト処理（3秒）

**Prerequisites**: Task 1.3
**Estimated Time**: 4時間
**Files**: `src-tauri/src/services/browser_action_service.rs`

## Phase 2: システム統合 (Priority: High)

### Task 2.1: 通知システム統合
- [ ] notification_service.rsにbrowser action実行追加
- [ ] 通知レベル判定ロジック統合
- [ ] エラー時の縮退処理実装
- [ ] 実行ログと監査証跡

**Prerequisites**: Task 1.4, 通知システム完成
**Estimated Time**: 3時間
**Files**: `src-tauri/src/services/notification_service.rs`

### Task 2.2: Tauriコマンド実装
- [ ] validate_url_command作成
- [ ] test_browser_action_command作成
- [ ] get_url_preview_command作成
- [ ] main.rsにコマンド登録

**Prerequisites**: Task 2.1
**Estimated Time**: 2時間
**Files**: `src-tauri/src/commands/browser_commands.rs`, `src-tauri/src/lib.rs`

### Task 2.3: データアクセス層更新
- [ ] TaskService.rs browser actions CRUD実装
- [ ] 型安全なJSON処理
- [ ] データベースエラーハンドリング
- [ ] トランザクション処理

**Prerequisites**: Task 2.2
**Estimated Time**: 2時間
**Files**: `src-tauri/src/services/task_service.rs`

## Phase 3: Frontend実装 (Priority: Medium)

### Task 3.1: TypeScript型定義拡張
- [ ] BrowserAction interface定義
- [ ] BrowserActionSettings interface定義
- [ ] TaskNotificationSettings拡張
- [ ] URLValidationResult型定義

**Prerequisites**: Task 2.3
**Estimated Time**: 1時間
**Files**: `src/types/Task.ts`, `src/types/BrowserAction.ts`

### Task 3.2: URLInputコンポーネント作成
- [ ] URL入力フィールド
- [ ] リアルタイムバリデーション
- [ ] プレビュー機能（ファビコン・タイトル）
- [ ] テストボタン
- [ ] エラー表示

**Prerequisites**: Task 3.1
**Estimated Time**: 4時間
**Files**: `src/components/URLInput.tsx`

### Task 3.3: URLActionConfigコンポーネント作成
- [ ] 複数URL管理UI
- [ ] ドラッグ&ドロップ並び替え
- [ ] 個別有効/無効トグル
- [ ] 追加/削除ボタン
- [ ] 最大5個制限

**Prerequisites**: Task 3.2
**Estimated Time**: 6時間
**Files**: `src/components/URLActionConfig.tsx`

### Task 3.4: タスクフォーム統合
- [ ] 既存TaskFormにブラウザアクション設定追加
- [ ] 通知設定と連動UI
- [ ] 条件付き表示ロジック
- [ ] フォームバリデーション

**Prerequisites**: Task 3.3
**Estimated Time**: 3時間
**Files**: `src/components/TaskForm.tsx`

## Phase 4: UX改善 (Priority: Low)

### Task 4.1: URL候補・履歴機能
- [ ] よく使うURL候補表示
- [ ] URL履歴保存（localStorageまたはDB）
- [ ] カテゴリ別URL候補
- [ ] 最近使用したURL表示

**Prerequisites**: Task 3.4
**Estimated Time**: 3時間
**Files**: `src/services/urlHistoryService.ts`, localStorage utilies

### Task 4.2: 設定画面とヘルプ
- [ ] ブラウザアクション設定ページ
- [ ] 機能説明とガイド
- [ ] セキュリティ注意事項
- [ ] 使用例とベストプラクティス

**Prerequisites**: Task 4.1
**Estimated Time**: 2時間
**Files**: `src/components/Settings/BrowserActionSettings.tsx`

### Task 4.3: 通知プレビュー機能
- [ ] 設定内容のプレビュー表示
- [ ] アクション実行シミュレーション
- [ ] 通知タイミング説明
- [ ] 期待される動作の可視化

**Prerequisites**: Task 4.2
**Estimated Time**: 2時間
**Files**: `src/components/NotificationPreview.tsx`

## Phase 5: テスト・品質保証 (Priority: Medium)

### Task 5.1: Backend単体テスト
- [ ] URLValidator単体テスト
- [ ] BrowserActionService単体テスト
- [ ] エラーケーステスト
- [ ] セキュリティテスト

**Prerequisites**: Phase 2完了
**Estimated Time**: 4時間
**Files**: `src-tauri/tests/browser_action_tests.rs`

### Task 5.2: Frontend単体テスト
- [ ] URLInputコンポーネントテスト
- [ ] URLActionConfigコンポーネントテスト
- [ ] バリデーションロジックテスト
- [ ] UIインタラクションテスト

**Prerequisites**: Phase 3完了
**Estimated Time**: 3時間
**Files**: `src/tests/components/`

### Task 5.3: 統合テストとE2Eテスト
- [ ] 通知時ブラウザ起動テスト
- [ ] 複数URL連続実行テスト
- [ ] エラー時縮退動作テスト
- [ ] パフォーマンステスト

**Prerequisites**: Phase 4完了
**Estimated Time**: 3時間
**Files**: `tests/integration/`

### Task 5.4: セキュリティテスト
- [ ] 悪意のあるURL検出テスト
- [ ] XSS攻撃耐性テスト
- [ ] プロトコル制限テスト
- [ ] URL長制限テスト

**Prerequisites**: Task 5.3
**Estimated Time**: 2時間
**Files**: `tests/security/`

## Quality Assurance

### Code Quality Checks
- [ ] Rust clippy warnings解決
- [ ] TypeScript strict mode適合
- [ ] ESLint規則準拠
- [ ] コードカバレッジ80%以上

### Documentation
- [ ] API仕様書更新
- [ ] コンポーネント使用例
- [ ] セキュリティガイドライン
- [ ] ユーザーマニュアル更新

### Performance Requirements
- [ ] URL検証応答時間 < 200ms
- [ ] ブラウザ起動時間 < 3秒
- [ ] UI応答性維持
- [ ] メモリ使用量監視

## Dependencies & Risks

### Internal Dependencies
- ✅ 通知システム（notification-system-redesign）
- ✅ タスク管理システム基盤
- ✅ Tauriシェル機能

### External Dependencies
- システムデフォルトブラウザ
- ネットワーク接続（URL検証時）
- OS提供のURL起動機能

### High Risk Items
- セキュリティ脆弱性（悪意URL実行）
- ブラウザ互換性問題
- パフォーマンス影響（大量URL処理）

### Risk Mitigation
- セキュリティテスト強化
- エラー時縮退処理実装
- 非同期処理による UI ブロック防止

## Success Criteria
- [ ] URL設定済みタスクが通知時に正しくブラウザを開く
- [ ] セキュリティ制限が適切に機能する
- [ ] 既存通知システムとの統合が完全
- [ ] エラー時でも通知は正常動作する
- [ ] UI/UX が直感的で使いやすい

---
*Created: 2025-08-15*
*Status: 📋 Implementation Roadmap Complete*
*Total Estimated Time: 50時間*
*Priority: High (Phase 1-2), Medium (Phase 3, 5), Low (Phase 4)*