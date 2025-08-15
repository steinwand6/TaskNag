# Requirements Definition - Notification Browser Actions

## Functional Requirements

### R1: URL設定機能
- **R1.1**: タスク作成・編集時にブラウザアクション用URLを設定可能
- **R1.2**: URLの有効/無効切り替えが可能
- **R1.3**: 複数URL設定対応（最大5個まで）
- **R1.4**: URL名称（ラベル）を設定可能
- **R1.5**: URLプレビュー機能（ファビコン・タイトル表示）

### R2: 通知連動機能
- **R2.1**: 期日ベース通知時にURLを自動で開く
- **R2.2**: 定期通知時にURLを自動で開く
- **R2.3**: 通知レベルに応じたアクション実行の制御
- **R2.4**: 手動通知無効化時はURLアクションも無効化

### R3: ブラウザ制御機能
- **R3.1**: システムデフォルトブラウザでURLを開く
- **R3.2**: 新しいタブで開く（既存セッション維持）
- **R3.3**: 複数URL設定時は連続して開く（500ms間隔）
- **R3.4**: ブラウザ起動失敗時の適切なエラーハンドリング

### R4: セキュリティ機能
- **R4.1**: HTTP/HTTPSプロトコルのみ許可
- **R4.2**: ローカルファイル（file://）の禁止
- **R4.3**: 危険なスキーム（javascript:, data:）の禁止
- **R4.4**: URL長制限（2048文字以内）
- **R4.5**: ドメイン検証（存在確認は行わない）

### R5: ユーザーインターフェース
- **R5.1**: タスクフォームにブラウザアクション設定セクション追加
- **R5.2**: URL入力フィールドとバリデーション表示
- **R5.3**: テストボタン（URLを即座に開いてテスト）
- **R5.4**: URL履歴・よく使うURLの候補表示
- **R5.5**: アクションプレビュー（通知時の動作説明）

## Non-Functional Requirements

### N1: パフォーマンス
- **N1.1**: URL起動処理は通知表示を阻害しない（非同期実行）
- **N1.2**: ブラウザ起動時間は3秒以内でタイムアウト
- **N1.3**: URL設定UIの応答時間は200ms以内
- **N1.4**: 大量タスク（1000+）でもパフォーマンス劣化なし

### N2: 可用性
- **N2.1**: ブラウザ未インストール時も通知は正常動作
- **N2.2**: ネットワーク未接続時も起動処理は実行
- **N2.3**: URL起動失敗時はログ記録のみで継続動作
- **N2.4**: システム負荷が高い時も基本機能は維持

### N3: ユーザビリティ
- **N3.1**: URL設定は5秒以内で完了可能
- **N3.2**: エラーメッセージは分かりやすい日本語表示
- **N3.3**: 設定変更は即座に反映
- **N3.4**: ヘルプ・ガイダンス機能の提供

### N4: セキュリティ
- **N4.1**: 入力されたURLはプレーンテキストでデータベース保存
- **N4.2**: XSS攻撃耐性（入力サニタイゼーション）
- **N4.3**: ログにはURL情報を記録（監査証跡）
- **N4.4**: 権限エスカレーション防止

## Interface Requirements

### I1: データベース拡張
```sql
-- TaskNotificationSettingsテーブル拡張案
ALTER TABLE tasks ADD COLUMN browser_actions TEXT DEFAULT NULL; -- JSON配列
```

### I2: 型定義拡張
```typescript
interface BrowserAction {
  id: string;
  label: string;
  url: string;
  enabled: boolean;
  order: number;
}

interface TaskNotificationSettings {
  // 既存フィールド
  notificationType: 'none' | 'due_date_based' | 'recurring';
  daysBefore?: number;
  notificationTime?: string;
  daysOfWeek?: number[];
  level: 1 | 2 | 3;
  
  // 新規フィールド
  browserActions?: BrowserAction[];
  enableBrowserActions?: boolean;
}
```

### I3: API拡張
- URL検証エンドポイント: `POST /api/validate-url`
- URL情報取得エンドポイント: `GET /api/url-info?url={url}`
- ブラウザ起動コマンド: `invoke('open_browser_url')`

## Business Rules

### B1: URL制限ルール
- 1タスクあたり最大5個のURL
- URL文字列長は2048文字以内
- 同一URL重複は許可（異なるラベルで区別）

### B2: 通知連動ルール
- 通知レベル1: ブラウザアクション無効
- 通知レベル2-3: ブラウザアクション有効
- タスク完了後はブラウザアクション停止

### B3: 実行順序ルール
1. デスクトップ通知表示
2. 音声通知再生
3. ブラウザアクション実行（非同期）
4. 通知ログ記録

## Acceptance Criteria

### AC1: 基本機能
- [ ] タスクにURL設定し、通知時に正しく開かれる
- [ ] 無効なURLは設定時に適切にエラー表示される
- [ ] 複数URL設定時はすべて順番に開かれる

### AC2: エラーハンドリング
- [ ] ブラウザ起動失敗時も通知は正常に表示される
- [ ] 不正なURLは保存を拒否される
- [ ] ネットワーク問題でもアプリは正常動作する

### AC3: ユーザビリティ
- [ ] URL設定UIは直感的で分かりやすい
- [ ] テスト機能でURLが即座に開かれる
- [ ] ヘルプテキストで機能が理解できる

## Dependencies

### Internal Dependencies
- 既存通知システム（notification-system-redesign）
- タスク管理システム（task-manager-desktop）
- Tauriシェル機能

### External Dependencies
- システムデフォルトブラウザ
- ネットワーク接続（URLアクセス時）
- OS提供のURL起動機能

## Risk Analysis

### High Risk
- **セキュリティ脆弱性**: 悪意のあるURL実行
- **パフォーマンス影響**: 大量URL処理時の負荷

### Medium Risk
- **ブラウザ互換性**: 特定ブラウザでの動作問題
- **ユーザー混乱**: 予期しないページ表示

### Low Risk
- **設定複雑化**: 多機能による使いにくさ
- **データ増加**: URL情報によるDB容量増加

---
*Created: 2025-08-15*
*Status: 📋 Requirements Definition Complete*