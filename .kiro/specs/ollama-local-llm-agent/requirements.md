# Ollama LocalLLM タスク管理エージェント - 要件仕様

## 概要
TaskNagにollama基盤のローカルLLMエージェント機能を統合し、プライベートで知的なタスク管理アシスタントを実現する。

## UltraThink: 深層設計思考

### 1. コア価値提案
- **完全プライベート**: すべてのAI処理がローカルで実行、データ漏洩なし
- **知的アシスタント**: 自然言語でのタスク操作と提案
- **コンテキスト理解**: ユーザーの作業パターンとタスク履歴を学習
- **シームレス統合**: 既存のTaskNag UIと完全統合

### 2. 技術アーキテクチャ

#### 2.1 Ollama API統合
```
TaskNag Backend (Rust) ↔ HTTP API ↔ Ollama Server (localhost:11434)
                                  ↔ Local LLM Models (llama3, codellama, etc.)
```

#### 2.2 サービス設計
**新規サービス**: `AgentService`
- ollama HTTP APIクライアント
- プロンプトエンジニアリング
- レスポンス解析とタスクデータ変換
- 会話履歴管理

#### 2.3 データフロー
```
User Natural Language Input → AgentService → Ollama API → LLM Processing → 
Structured Response → Task Generation → TaskNag Database → UI Update
```

## 3. エージェント機能仕様

### 3.1 タスク分析エージェント
**機能**:
- タスク記述の自動改善提案
- 複雑度とスコープ評価
- カテゴリとタグの自動提案
- 優先度レベル推論

**API**:
```rust
async fn analyze_task(&self, description: &str) -> Result<TaskAnalysis, AgentError>
```

**出力例**:
```json
{
  "improved_description": "React学習プロジェクト：基礎から実践まで",
  "suggested_tags": ["学習", "プログラミング", "フロントエンド"],
  "estimated_duration": "2-3週間",
  "complexity": "medium",
  "subtasks": [
    "React基礎概念の学習",
    "Hooks APIの理解",
    "実践プロジェクト開発"
  ]
}
```

### 3.2 プロジェクト計画エージェント
**機能**:
- 大きなプロジェクトの自動分解
- タスク依存関係の推論
- 時系列スケジューリング
- リソース見積もり

**API**:
```rust
async fn plan_project(&self, project_desc: &str) -> Result<ProjectPlan, AgentError>
```

### 3.3 進捗分析エージェント
**機能**:
- 現在の進捗状況分析
- ボトルネックの特定
- 次のアクション提案
- デッドライン警告

**API**:
```rust
async fn analyze_progress(&self, tasks: &[Task]) -> Result<ProgressAnalysis, AgentError>
```

### 3.4 通知コンテンツ生成エージェント
**機能**:
- コンテキスト理解した通知文生成
- ユーザーパターン学習
- 適応的通知タイミング
- モチベーショナルメッセージ

## 4. UI/UX設計

### 4.1 エージェントチャット界面
**位置**: サイドパネルまたはモーダル
**機能**:
- 自然言語入力フィールド
- ストリーミングレスポンス表示
- 提案タスクのプレビューと承認
- 会話履歴の表示

### 4.2 Smart Suggestions
**位置**: 既存タスクカードに統合
**機能**:
- 「💡 エージェント提案」ボタン
- インライン改善提案
- 自動補完とタグ提案

### 4.3 Progress Dashboard
**新規画面**: `/analytics`
**機能**:
- AI生成の進捗レポート
- ボトルネック可視化
- 改善提案リスト

## 5. Ollama統合実装

### 5.1 HTTP APIクライアント
```rust
pub struct OllamaClient {
    base_url: String,  // "http://localhost:11434"
    default_model: String,  // "llama3:latest"
    client: reqwest::Client,
}

impl OllamaClient {
    async fn generate(&self, prompt: &str, model: Option<&str>) -> Result<String, OllamaError>
    async fn chat(&self, messages: &[ChatMessage], model: Option<&str>) -> Result<String, OllamaError>
    async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError>
}
```

### 5.2 プロンプトテンプレート
**タスク分析用**:
```
あなたは日本語タスク管理の専門家です。以下のタスク記述を分析し、改善提案をJSONで出力してください：

タスク: "{task_description}"

出力形式:
{
  "improved_description": "改善された記述",
  "suggested_tags": ["タグ1", "タグ2"],
  "estimated_duration": "見積もり期間",
  "subtasks": ["サブタスク1", "サブタスク2"]
}
```

### 5.3 レスポンス解析
```rust
#[derive(Deserialize)]
pub struct TaskAnalysis {
    pub improved_description: String,
    pub suggested_tags: Vec<String>,
    pub estimated_duration: String,
    pub subtasks: Vec<String>,
}
```

## 6. エラーハンドリングと信頼性

### 6.1 Ollama接続エラー
- オフライン検出とフォールバック
- タイムアウト設定（30秒）
- ユーザーフレンドリーエラーメッセージ

### 6.2 レスポンス品質制御
- JSON形式バリデーション
- 不適切な内容フィルタリング
- 長すぎるレスポンスの切り詰め

## 7. 設定とカスタマイズ

### 7.1 エージェント設定
```json
{
  "ollama": {
    "enabled": true,
    "base_url": "http://localhost:11434",
    "default_model": "llama3:latest",
    "timeout_seconds": 30,
    "max_response_length": 2000
  },
  "agent": {
    "auto_suggestions": true,
    "learning_enabled": true,
    "conversation_history_days": 30
  }
}
```

### 7.2 モデル管理
- 利用可能モデル一覧表示
- モデル切り替え機能
- モデルダウンロード状況確認

## 8. パフォーマンス考慮

### 8.1 レスポンス最適化
- ストリーミングレスポンス対応
- キャッシュ機能（同一質問の重複回避）
- バックグラウンド処理での非同期実行

### 8.2 リソース管理
- ollama接続プール
- メモリ使用量監視
- CPU使用率制限

## 9. 段階的実装計画

### Phase 1: 基盤実装
1. OllamaClient基本実装
2. 簡単なタスク分析機能
3. 設定画面の追加

### Phase 2: エージェント機能
1. チャット界面実装
2. タスク生成フロー
3. 提案システム統合

### Phase 3: 高度機能
1. 学習機能とパターン認識
2. 進捗分析とレポート
3. プロジェクト計画機能

## 10. 成功指標

### 10.1 技術指標
- ollama API応答時間 < 5秒
- エラー率 < 1%
- ユーザー満足度スコア > 4.0/5.0

### 10.2 利用指標
- エージェント提案採用率 > 60%
- 自然言語タスク作成率 > 30%
- ユーザー継続利用率 > 80%

## 11. セキュリティとプライバシー

### 11.1 データプライバシー
- 全処理ローカル実行
- 会話データの暗号化保存
- ユーザー制御可能なデータ削除

### 11.2 セキュリティ
- ollama API認証（必要に応じて）
- 入力サニタイゼーション
- 悪意あるプロンプト検出

この設計により、TaskNagは単純なタスク管理ツールから知的生産性アシスタントへと進化し、完全プライベートな環境でAIの恩恵を受けられる革新的ソリューションとなる。