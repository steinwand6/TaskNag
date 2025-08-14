# Ollama LocalLLM エージェント - 実装タスク

## Phase 1: 基盤実装 (1-2週間)

### Task 1.1: Ollama HTTP APIクライアント実装
- [ ] `src-tauri/src/services/ollama_client.rs` 作成
- [ ] `reqwest` crateの依存関係追加 
- [ ] 基本API呼び出し機能 (`/api/generate`, `/api/tags`)
- [ ] エラーハンドリングとタイムアウト設定
- [ ] 接続テスト機能

### Task 1.2: AgentService基盤構築  
- [ ] `src-tauri/src/services/agent_service.rs` 作成
- [ ] OllamaClientとの統合
- [ ] 基本的なプロンプト管理
- [ ] JSONレスポンス解析機能
- [ ] エージェント設定管理

### Task 1.3: データモデル定義
- [ ] `src-tauri/src/models/agent.rs` 作成
- [ ] Conversation, Message, AgentSuggestion構造体
- [ ] データベースマイグレーション (conversations, suggestions テーブル)
- [ ] 型安全なAPI応答構造体

### Task 1.4: 基本UI実装
- [ ] `src/components/AgentChat.tsx` 作成
- [ ] `src/components/AgentSettings.tsx` 作成
- [ ] エージェント設定画面
- [ ] 最小限チャット界面

## Phase 2: コアエージェント機能 (3-4週間)

### Task 2.1: タスク分析エージェント
- [ ] タスク記述改善プロンプト
- [ ] タグ・カテゴリ自動提案
- [ ] 複雑度とスコープ評価
- [ ] サブタスク生成機能

### Task 2.2: プロジェクト計画エージェント  
- [ ] プロジェクト分解アルゴリズム
- [ ] 依存関係推論機能
- [ ] 時系列スケジューリング
- [ ] リソース見積もり

### Task 2.3: 自然言語タスク作成
- [ ] 自然言語→構造化タスクデータ変換
- [ ] 既存タスクフォームとの統合
- [ ] プレビューと承認フロー
- [ ] バッチタスク作成機能

### Task 2.4: Smart Suggestions統合
- [ ] 既存タスクカードにエージェント提案ボタン
- [ ] インライン改善提案表示
- [ ] ワンクリック適用機能
- [ ] 提案履歴管理

## Phase 3: 高度機能 (5-6週間)

### Task 3.1: 進捗分析エージェント
- [ ] 現在進捗状況の詳細分析
- [ ] ボトルネック自動検出
- [ ] 次アクション提案アルゴリズム
- [ ] デッドライン警告システム

### Task 3.2: 学習・適応機能
- [ ] ユーザー行動パターン学習
- [ ] 提案精度向上アルゴリズム
- [ ] パーソナライゼーション機能
- [ ] フィードバック収集システム

### Task 3.3: ストリーミング応答
- [ ] WebSocket統合 (フロントエンド)
- [ ] リアルタイムストリーミング表示
- [ ] 部分応答のプログレッシブ表示
- [ ] 応答キャンセル機能

### Task 3.4: 高度分析ダッシュボード
- [ ] `/analytics` ページ作成
- [ ] AI生成進捗レポート
- [ ] 生産性インサイト表示
- [ ] 改善提案リスト

## 実装詳細

### タスク1.1: OllamaClient実装詳細
```rust
// src-tauri/Cargo.toml に追加
[dependencies]
reqwest = { version = "0.11", features = ["json", "stream"] }
tokio-stream = "0.1"

// src-tauri/src/services/ollama_client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    base_url: String,
    client: Client,
    default_model: String,
    timeout_seconds: u64,
}

#[derive(Serialize)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<GenerateOptions>,
}

#[derive(Deserialize)]
pub struct GenerateResponse {
    response: String,
    done: bool,
    context: Option<Vec<i32>>,
}
```

### タスク1.2: AgentService実装詳細
```rust
// src-tauri/src/services/agent_service.rs
use crate::services::ollama_client::OllamaClient;
use crate::models::Task;

pub struct AgentService {
    ollama: OllamaClient,
    prompt_manager: PromptManager,
    db: Database,
}

impl AgentService {
    pub async fn analyze_task(&self, description: &str) -> Result<TaskAnalysis, AgentError> {
        let prompt = self.prompt_manager.build_task_analysis_prompt(description);
        let response = self.ollama.generate(&prompt, None).await?;
        let analysis: TaskAnalysis = serde_json::from_str(&response.response)?;
        Ok(analysis)
    }
}
```

### タスク2.3: UI統合実装詳細
```tsx
// src/components/SmartTaskCreator.tsx
export const SmartTaskCreator: React.FC = () => {
  const [input, setInput] = useState('');
  const [suggestions, setSuggestions] = useState<TaskAnalysis | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);

  const handleAnalyze = async () => {
    setIsAnalyzing(true);
    try {
      const analysis = await invoke('analyze_task', { taskDescription: input });
      setSuggestions(analysis);
    } catch (error) {
      console.error('Analysis failed:', error);
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleCreateTask = async () => {
    if (!suggestions) return;
    
    const taskData = {
      title: suggestions.improved_description,
      description: input,
      // ... other fields from analysis
    };
    
    await invoke('create_task', taskData);
  };
};
```

## 設定ファイル

### AgentConfig
```json
// src-tauri/agent_config.json
{
  "ollama": {
    "base_url": "http://localhost:11434",
    "default_model": "llama3:latest",
    "timeout_seconds": 30,
    "retry_attempts": 3
  },
  "prompts": {
    "task_analysis_template": "templates/task_analysis.txt",
    "project_planning_template": "templates/project_planning.txt"
  },
  "features": {
    "auto_suggestions": true,
    "learning_enabled": true,
    "streaming_responses": true
  }
}
```

## 成功条件

### 定量的指標
- ollama API応答時間 < 5秒 (95%ile)
- タスク分析精度 > 80% (ユーザー採用率)
- システム安定性 > 99.5% (稼働率)

### 定性的指標  
- 自然言語でのタスク作成が直感的
- エージェント提案が実用的で有用
- UIが既存ワークフローを妨げない

## リスク対策

### 技術リスク
- **ollama接続失敗**: オフラインモードフォールバック
- **レスポンス品質低下**: プロンプト改善とモデル切り替え
- **パフォーマンス劣化**: 非同期処理とキャッシュ

### UXリスク
- **AI依存度過多**: 手動機能の完全保持
- **学習曲線**: チュートリアルとオンボーディング
- **信頼性懸念**: 透明性のある提案根拠表示

この段階的実装により、TaskNagは確実にAI-poweredタスク管理システムへと進化する。