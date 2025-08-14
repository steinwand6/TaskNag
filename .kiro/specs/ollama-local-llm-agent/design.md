# Ollama LocalLLM エージェント - 設計仕様

## 1. システムアーキテクチャ

### 1.1 全体構成
```
┌─────────────────────────────────────────────────────────────┐
│                    TaskNag Frontend (React)                 │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌─────────────────┐  ┌───────────────┐ │
│ │   Chat Panel    │  │ Smart Suggestions│  │ Agent Settings│ │
│ │   - 自然言語入力  │  │ - インライン提案  │  │ - モデル選択   │ │
│ │   - ストリーミング│  │ - タスク改善案   │  │ - 設定管理     │ │
│ └─────────────────┘  └─────────────────┘  └───────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                    Tauri Backend (Rust)                    │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌─────────────────┐  ┌───────────────┐ │
│ │  AgentService   │  │  OllamaClient   │  │ PromptManager │ │
│ │  - エージェント   │  │  - HTTP API     │  │ - テンプレート │ │
│ │  - ロジック統制   │  │  - モデル管理    │  │ - プロンプト   │ │
│ └─────────────────┘  └─────────────────┘  └───────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                    Local Ollama Server                     │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌─────────────────┐  ┌───────────────┐ │
│ │    llama3       │  │   codellama     │  │    mistral    │ │
│ │  - 汎用エージェント│  │ - コード分析     │  │ - 高速応答     │ │
│ └─────────────────┘  └─────────────────┘  └───────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 データフロー
```
1. User Input (自然言語) → Frontend
2. Frontend → Tauri Command → AgentService
3. AgentService → PromptManager (テンプレート適用)
4. AgentService → OllamaClient → HTTP Request
5. Ollama Server → LLM Processing → HTTP Response
6. OllamaClient → AgentService (JSON解析)
7. AgentService → Task Creation/Update
8. Database Update → Frontend State Update
```

## 2. API設計

### 2.1 Tauri Commands
```rust
#[tauri::command]
async fn chat_with_agent(message: String, context: Option<String>) -> Result<AgentResponse, String>

#[tauri::command] 
async fn analyze_task(task_description: String) -> Result<TaskAnalysis, String>

#[tauri::command]
async fn plan_project(project_description: String) -> Result<ProjectPlan, String>

#[tauri::command]
async fn get_progress_insights(task_ids: Vec<String>) -> Result<ProgressInsights, String>

#[tauri::command]
async fn list_ollama_models() -> Result<Vec<ModelInfo>, String>

#[tauri::command]
async fn set_agent_model(model_name: String) -> Result<(), String>
```

### 2.2 Ollama HTTP API統合
```rust
// GET /api/tags - モデル一覧
pub async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError>

// POST /api/generate - 単発生成
pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, OllamaError>

// POST /api/chat - チャット形式
pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, OllamaError>

// WebSocket /api/generate (stream) - ストリーミング
pub async fn generate_stream(&self, request: GenerateRequest) -> Result<Stream, OllamaError>
```

## 3. プロンプトエンジニアリング

### 3.1 タスク分析プロンプト
```
SYSTEM: あなたは日本語タスク管理の専門エージェントです。ユーザーのタスクを分析し、構造化された改善提案を行います。

USER: 以下のタスクを分析してください：
タスク: "{task_description}"
既存タグ: {available_tags}

以下のJSON形式で回答してください：
{
  "improved_description": "明確で実行可能な記述",
  "suggested_tags": ["関連タグ配列"],
  "estimated_hours": 数値,
  "complexity": "low|medium|high",
  "subtasks": ["具体的サブタスク配列"],
  "dependencies": ["依存タスク配列"],
  "risks": ["潜在リスク配列"]
}
```

### 3.2 プロジェクト計画プロンプト
```
SYSTEM: あなたはプロジェクト管理の専門家です。大きなプロジェクトを実行可能なタスクに分解し、効率的なスケジュールを提案します。

USER: プロジェクト: "{project_description}"
期限: {deadline}
利用可能時間: {available_hours_per_week}時間/週

以下の構造でプロジェクト計画を作成してください：
{
  "phases": [
    {
      "name": "フェーズ名",
      "duration_weeks": 数値,
      "tasks": ["タスク配列"],
      "deliverables": ["成果物配列"]
    }
  ],
  "critical_path": ["クリティカルパス"],
  "resources_needed": ["必要リソース"],
  "risk_mitigation": ["リスク対策"]
}
```

## 4. データモデル

### 4.1 エージェント会話
```rust
#[derive(Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub context_task_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,  // User, Assistant, System
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
```

### 4.2 エージェント提案
```rust
#[derive(Serialize, Deserialize)]
pub struct AgentSuggestion {
    pub id: String,
    pub suggestion_type: SuggestionType,  // TaskImprovement, ProjectPlan, etc.
    pub content: serde_json::Value,
    pub confidence: f32,  // 0.0-1.0
    pub applied: bool,
    pub created_at: DateTime<Utc>,
}
```

## 5. 実装優先順位

### Phase 1: 基盤実装 (Week 1-2)
1. **OllamaClient基本実装**
   - HTTP API接続
   - 基本的なgenerate機能
   - エラーハンドリング

2. **AgentService基盤**
   - 単純なタスク分析機能
   - JSON レスポンス解析
   - データベース統合

3. **最小限UI**
   - エージェント設定画面
   - 基本チャット界面

### Phase 2: コア機能 (Week 3-4)
1. **Smart Task Creation**
   - 自然言語→タスク変換
   - タグとカテゴリ自動提案
   - サブタスク生成

2. **Project Planning**
   - プロジェクト分解機能
   - 依存関係推論
   - スケジューリング

3. **UI Integration**
   - 既存フォームとの統合
   - ワンクリック提案適用
   - プレビュー機能

### Phase 3: 高度機能 (Week 5-6)
1. **Learning & Adaptation**
   - ユーザーパターン学習
   - 提案品質向上
   - パーソナライゼーション

2. **Advanced Analytics**
   - 進捗分析レポート
   - 生産性インサイト
   - 改善提案ダッシュボード

## 6. パフォーマンス要件

### 6.1 レスポンス時間
- 簡単な分析: < 3秒
- 複雑な計画: < 10秒
- ストリーミング開始: < 1秒

### 6.2 リソース使用量
- RAM使用量追加: < 100MB
- CPU使用率: < 10% (アイドル時)
- ディスク使用量: < 50MB (会話履歴含む)

## 7. テスト戦略

### 7.1 単体テスト
- OllamaClient API通信
- プロンプトテンプレート検証
- JSON解析精度

### 7.2 統合テスト
- E2E エージェント機能
- UI統合動作確認
- パフォーマンステスト

### 7.3 ユーザビリティテスト
- 自然言語理解精度
- 提案品質評価
- ワークフロー効率性

## 8. 運用・保守

### 8.1 監視
- ollama接続状況監視
- エージェント応答品質追跡
- ユーザーフィードバック収集

### 8.2 更新・メンテナンス
- モデル更新ガイダンス
- プロンプト改善履歴
- 設定マイグレーション

この設計により、TaskNagはAI駆動の知的タスク管理システムとして、ユーザーの生産性を大幅に向上させる革新的ツールに進化する。