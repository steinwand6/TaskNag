# Requirements Document - TaskNag

## 1. Functional Requirements

> **TaskNag** は「口うるさくて世話焼き」をコンセプトとした、プロアクティブなタスク管理アプリケーションです。ユーザーが忘れがちなタスクを積極的にリマインドし、生産性向上をサポートします。

### 1.1 Task Management

#### FR-001: Task Creation
- **Description**: ユーザーは新しいタスクを作成できる
- **Priority**: High
- **Acceptance Criteria**:
  - タスクタイトル（必須、最大200文字）を入力できる
  - タスク説明（任意、最大2000文字）を入力できる
  - 作成時のデフォルトステータスは"inbox"
  - 作成日時が自動的に記録される

#### FR-002: Task Status Management
- **Description**: タスクのステータスを4段階で管理できる
- **Priority**: High
- **States**:
  - `inbox`: 新規・未分類タスク
  - `todo`: 実行予定タスク
  - `in_progress`: 実行中タスク
  - `done`: 完了タスク
- **Acceptance Criteria**:
  - ドラッグ&ドロップでステータス変更可能
  - ショートカットキーでステータス変更可能
  - ステータス変更時にタイムスタンプを記録

#### FR-003: Custom Notification Settings
- **Description**: タスクごとに個別の通知設定が可能
- **Priority**: High
- **Notification Types**:
  - None: 通知なし（デフォルト）
  - Due Date Based: 期日N日前から期日まで連続通知
  - Recurring: 曜日・時刻指定での定期リマインド
- **Acceptance Criteria**:
  - 期日ベース通知では通知開始日数と時刻を設定可能
  - 定期通知では曜日（複数選択可）と時刻（時・分）を設定可能
  - 期限なしタスクでも定期通知は有効
  - タスク作成時・詳細画面で設定可能

#### FR-004: Task Editing
- **Description**: 既存タスクを編集できる
- **Priority**: High
- **Acceptance Criteria**:
  - タイトル、説明、通知設定、期限を変更可能
  - 更新日時が自動記録される
  - 編集中の自動保存機能

#### FR-005: Task Deletion
- **Description**: タスクを削除できる
- **Priority**: Medium
- **Acceptance Criteria**:
  - 削除確認ダイアログ表示
  - ソフトデリート（ゴミ箱機能）
  - 完全削除オプション

#### FR-006: Due Date Management
- **Description**: タスクに期限を設定できる
- **Priority**: High
- **Acceptance Criteria**:
  - カレンダーUIで日付選択
  - 時刻も設定可能
  - 期限切れタスクの視覚的強調

#### FR-007: Sub-Tasks (子タスク)
- **Description**: タスクを階層構造で管理できる
- **Priority**: High
- **Acceptance Criteria**:
  - 親タスクに対して複数の子タスクを作成可能
  - 子タスクは独立したステータスを持つ
  - 親タスクの進捗率を子タスクの完了状況から自動計算
  - 最大3階層まで（親→子→孫）
  - 子タスクの折りたたみ/展開表示
  - 親タスク完了時の子タスク一括完了オプション

#### FR-008: Recurring Tasks (定期タスク)
- **Description**: 定期的に繰り返されるタスクを設定できる
- **Priority**: High
- **Recurrence Patterns**:
  - 毎日（特定時刻）
  - 毎週（特定曜日・時刻）
  - 毎月（特定日・時刻）
  - 毎年（特定日付・時刻）
  - カスタム間隔（N日ごと、N週間ごと等）
- **Acceptance Criteria**:
  - 定期タスクのテンプレート作成
  - 次回実行日時の自動計算・表示
  - タスク完了時に次回タスクを自動生成
  - 定期タスクの一時停止/再開
  - 特定回数での終了設定
  - 期限付き定期タスク（終了日の設定）

#### FR-009: Tag System
- **Description**: タスクにタグを付けて分類できる
- **Priority**: Medium
- **Acceptance Criteria**:
  - 複数タグの付与が可能
  - タグの作成・編集・削除
  - タグによるフィルタリング
  - タグの色分け機能

### 1.2 Notification System

#### FR-010: Custom Notification Settings (Revised)
- **Description**: タスクごとに詳細な通知設定が可能
- **Priority**: High
- **Setting Types**:
  - **None**: 通知なし（デフォルト）
  - **Due Date Based**: 期日N日前から期日まで毎日通知
  - **Recurring**: 指定曜日・時刻での定期通知
- **Notification Levels**:
  - **Level 1**: システム通知のみ
  - **Level 2**: システム通知 + 音声通知
  - **Level 3**: アプリケーション最大化 + 通知
- **Acceptance Criteria**:
  - 期日ベース：開始日数（1-30日前）と通知時刻を設定
  - 定期通知：曜日（複数選択）と時刻（時・分）を設定
  - 通知レベルをタスクごとに個別設定可能
  - 期限なしタスクでも定期通知は機能
  - タスク作成時と詳細画面で設定可能
  - 通知頻度は1分間隔でチェック

#### FR-011: Due Date Notifications (Updated)
- **Description**: 期日ベース通知の詳細仕様
- **Priority**: High
- **Acceptance Criteria**:
  - 設定した日数前から期日当日まで毎日通知
  - 指定時刻に正確に通知
  - 同日重複通知の防止機能
  - 完了済みタスクは通知停止

#### FR-012: Recurring Notifications (New)
- **Description**: 定期通知の詳細仕様
- **Priority**: High
- **Acceptance Criteria**:
  - 曜日単位での通知設定（月-日の複数選択可）
  - 時・分での精密な時刻設定
  - 期限なしタスクでも機能
  - 月次通知も将来的に対応可能な設計

### 1.3 Search and Filter

#### FR-013: Search Functionality
- **Description**: タスクを検索できる
- **Priority**: High
- **Acceptance Criteria**:
  - タイトル、説明での全文検索
  - リアルタイム検索結果表示
  - 検索履歴機能

#### FR-014: Filter Options
- **Description**: 複数条件でタスクをフィルタリングできる
- **Priority**: High
- **Filter Criteria**:
  - ステータス
  - 通知設定タイプ
  - タグ
  - 期限（今日、今週、期限切れ）
- **Acceptance Criteria**:
  - 複数フィルタの組み合わせ
  - フィルタのプリセット保存

### 1.4 System Integration

#### FR-015: System Tray
- **Description**: システムトレイに常駐する
- **Priority**: High
- **Acceptance Criteria**:
  - 最小化時にシステムトレイに格納
  - トレイアイコンから主要機能へのアクセス
  - 未完了タスク数のバッジ表示

#### FR-016: Hotkeys
- **Description**: グローバルホットキーをサポート
- **Priority**: Medium
- **Default Hotkeys**:
  - `Ctrl+Shift+T`: 新規タスク作成
  - `Ctrl+Shift+Space`: アプリ表示/非表示
  - `Ctrl+Shift+Q`: クイック追加
- **Acceptance Criteria**:
  - カスタマイズ可能なホットキー
  - ホットキーの競合検出

#### FR-017: Auto-Start
- **Description**: Windows起動時に自動起動
- **Priority**: Low
- **Acceptance Criteria**:
  - 設定で有効/無効切り替え可能
  - 最小化状態での起動オプション

### 1.5 Data Management

#### FR-018: Data Backup
- **Description**: データのバックアップ機能
- **Priority**: Medium
- **Acceptance Criteria**:
  - 手動バックアップ
  - 自動バックアップ（日次）
  - バックアップからの復元

#### FR-019: Data Export
- **Description**: データをエクスポートできる
- **Priority**: Low
- **Formats**:
  - JSON
  - CSV
  - Markdown
- **Acceptance Criteria**:
  - 選択的エクスポート
  - 全データエクスポート

## 2. Non-Functional Requirements

### 2.1 Performance

#### NFR-001: Application Startup
- **Requirement**: アプリケーション起動時間 < 1秒
- **Measurement**: コールドスタートから使用可能になるまで

#### NFR-002: Memory Usage
- **Requirement**: メモリ使用量 < 50MB（アイドル時）
- **Measurement**: タスク100件登録時のアイドル状態

#### NFR-003: Database Performance
- **Requirement**: データベースクエリ応答 < 10ms
- **Measurement**: 1000件のタスクでの検索・フィルタリング

#### NFR-004: UI Responsiveness
- **Requirement**: UI操作の応答時間 < 100ms
- **Measurement**: クリックからビジュアルフィードバックまで

### 2.2 Reliability

#### NFR-005: Data Integrity
- **Requirement**: データ損失ゼロ
- **Measurement**: 異常終了時のデータ保全率100%

#### NFR-006: Crash Recovery
- **Requirement**: クラッシュ後の自動復旧
- **Measurement**: 前回の状態を完全復元

### 2.3 Usability

#### NFR-007: Learning Curve
- **Requirement**: 5分以内に基本操作習得
- **Measurement**: 新規ユーザーテスト

#### NFR-008: Keyboard Navigation
- **Requirement**: マウスなしで全機能操作可能
- **Measurement**: キーボードのみでの操作テスト

### 2.4 Security

#### NFR-009: Data Encryption
- **Requirement**: ローカルデータベースの暗号化オプション
- **Measurement**: AES-256暗号化

#### NFR-010: Secure Storage
- **Requirement**: 設定・認証情報の安全な保存
- **Measurement**: Windows Credential Manager使用

### 2.5 Compatibility

#### NFR-011: OS Support
- **Requirement**: Windows 10 (1909以降) / Windows 11
- **Measurement**: 各バージョンでの動作確認

#### NFR-012: Display Support
- **Requirement**: HiDPI/マルチモニター対応
- **Measurement**: 各種解像度での表示確認

## 3. Constraints

### 3.1 Technical Constraints
- **Programming Language**: Rust (Backend), TypeScript (Frontend)
- **Framework**: Tauri 2.0+
- **Database**: SQLite 3.x
- **Binary Size**: < 20MB
- **Minimum RAM**: 2GB
- **Minimum Storage**: 100MB

### 3.2 Business Constraints
- **Development Timeline**: 3ヶ月でMVPリリース
- **Maintenance**: 単一開発者でメンテナンス可能
- **Licensing**: オープンソース互換ライセンス

### 3.3 Regulatory Constraints
- **Data Privacy**: ローカルデータのみ（クラウド同期なし）
- **Accessibility**: Windows標準アクセシビリティ準拠

## 4. Acceptance Criteria

### 4.1 MVP Acceptance Criteria
- [ ] タスクのCRUD操作が完全に機能する
- [ ] 4つのステータス間でタスク移動が可能
- [ ] システムトレイ常駐が動作する
- [ ] 基本的な通知機能が動作する
- [ ] SQLiteデータベースが正常に動作する
- [ ] アプリケーション起動が1秒以内
- [ ] メモリ使用量が50MB以下

### 4.2 Phase 2 Acceptance Criteria
- [ ] 3段階の通知レベルが機能する
- [ ] 音声通知が動作する
- [ ] 期限管理と通知が連動する
- [ ] ホットキーが動作する

### 4.3 Phase 3 Acceptance Criteria
- [ ] 検索・フィルタリングが高速動作
- [ ] タグシステムが完全実装
- [ ] データバックアップが動作
- [ ] キーボードナビゲーション完備

## 5. Future Requirements (Phase 4)

### 5.1 LLM Integration
- **Natural Language Input**: 自然言語でのタスク入力
- **Smart Suggestions**: AIによるタスク提案
- **Auto-Categorization**: タスクの自動分類
- **Priority Estimation**: 優先度の自動判定
- **Due Date Prediction**: 期限の自動推定

### 5.2 Collaboration Features
- **Task Sharing**: タスクの共有機能
- **Team Workspace**: チームワークスペース
- **Sync Options**: 選択的クラウド同期

## Approval Status
- [ ] Requirements Review Completed
- [ ] Stakeholder Approval
- [ ] Technical Feasibility Confirmed
- [ ] Ready for Design Phase

---
*Last Updated: 2025-01-13*
*Version: 1.0.0*