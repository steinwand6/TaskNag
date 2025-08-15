# 変更しやすいAIシステム設計書

## 概要
現在のAgentServiceを、コンテキスト収集と回答方向性を簡単に変更できる柔軟な構造に改善する設計書。

## 1. 現在の課題分析

### 1.1 コンテキスト収集の課題
```rust
// 現在：各メソッドでハードコード
pub async fn analyze_task(&self, description: &str) -> Result<TaskAnalysis, AgentError> {
    let mut variables = std::collections::HashMap::new();
    variables.insert("description".to_string(), description.to_string());
    // ← ここでコンテキスト情報が固定
```

**問題点：**
- 時間情報、タスク状況、ユーザー行動パターンが含まれない
- 新しいコンテキストを追加するには各メソッドを修正が必要
- 機能ごとに必要なコンテキストを調整できない

### 1.2 回答方向性の課題
```rust
// 現在：プロンプトテンプレートが固定
templates.insert(
    "task_analysis".to_string(),
    r#"あなたはタスク管理の専門家です。以下のタスクを分析して、改善提案をJSONで返してください。"#
);
```

**問題点：**
- 「専門家」という役割が固定
- 励まし重視 vs 分析重視の切り替えができない
- TaskNagの「口うるさい」特性が反映されていない

## 2. 改善設計：柔軟なコンテキスト・プロンプトシステム

### 2.1 コンテキスト収集システム
```rust
// 新しい設計：コンテキスト収集を分離
pub struct ContextCollector {
    db: Arc<SqlitePool>,
    collectors: HashMap<String, Box<dyn ContextProvider>>,
}

pub trait ContextProvider: Send + Sync {
    async fn collect(&self) -> Result<ContextData, ContextError>;
    fn name(&self) -> &str;
}

// 具体的なコンテキスト提供者
pub struct TemporalContextProvider;
pub struct TaskContextProvider { db: Arc<SqlitePool> }
pub struct UserBehaviorProvider { db: Arc<SqlitePool> }
pub struct WorkloadAnalyzer { db: Arc<SqlitePool> }

impl ContextProvider for TemporalContextProvider {
    async fn collect(&self) -> Result<ContextData, ContextError> {
        let now = chrono::Utc::now();
        Ok(ContextData::new("temporal")
            .with("current_time", now.format("%Y-%m-%d %H:%M:%S").to_string())
            .with("day_of_week", now.weekday().to_string())
            .with("is_business_day", is_business_day(now).to_string())
            .with("time_of_day", get_time_period(now))
        )
    }
}

impl ContextProvider for TaskContextProvider {
    async fn collect(&self) -> Result<ContextData, ContextError> {
        // タスク関連の統計情報を収集
        let total_tasks = self.count_total_tasks().await?;
        let completed_today = self.count_completed_today().await?;
        let overdue_count = self.count_overdue_tasks().await?;
        
        Ok(ContextData::new("task_status")
            .with("total_tasks", total_tasks.to_string())
            .with("completed_today", completed_today.to_string())
            .with("overdue_count", overdue_count.to_string())
        )
    }
}
```

### 2.2 動的プロンプト生成システム
```rust
pub struct FlexiblePromptManager {
    base_templates: HashMap<String, PromptTemplate>,
    response_styles: HashMap<String, ResponseStyle>,
    context_collector: ContextCollector,
}

pub struct PromptTemplate {
    pub id: String,
    pub base_instruction: String,
    pub required_context: Vec<String>,
    pub output_format: String,
    pub personality_adaptable: bool,
}

pub struct ResponseStyle {
    pub id: String,
    pub name: String,
    pub role_description: String,
    pub tone_instruction: String,
    pub focus_areas: Vec<String>,
}

impl FlexiblePromptManager {
    pub async fn build_contextual_prompt(
        &self,
        template_id: &str,
        style_id: &str,
        static_vars: &HashMap<String, String>,
        context_scope: &[String],
    ) -> Result<String, PromptError> {
        // 1. テンプレートとスタイルを取得
        let template = self.get_template(template_id)?;
        let style = self.get_response_style(style_id)?;
        
        // 2. 必要なコンテキストを収集
        let context = self.context_collector
            .collect_context(context_scope).await?;
        
        // 3. プロンプトを組み立て
        let mut prompt = String::new();
        
        // 役割とトーン設定
        prompt.push_str(&format!("あなたは{}です。{}\n\n", 
            style.role_description, style.tone_instruction));
        
        // コンテキスト情報を注入
        if !context.is_empty() {
            prompt.push_str("現在の状況:\n");
            for (key, value) in context.iter() {
                prompt.push_str(&format!("- {}: {}\n", key, value));
            }
            prompt.push_str("\n");
        }
        
        // メインの指示
        prompt.push_str(&template.base_instruction);
        
        // 静的変数を置換
        for (key, value) in static_vars {
            let placeholder = format!("{{{}}}", key);
            prompt = prompt.replace(&placeholder, value);
        }
        
        // 出力形式を追加
        prompt.push_str(&format!("\n\n{}", template.output_format));
        
        Ok(prompt)
    }
}
```

### 2.3 応答スタイル定義
```rust
// 応答スタイルを簡単に追加・変更可能
impl ResponseStyle {
    pub fn encouraging_coach() -> Self {
        Self {
            id: "encouraging_coach".to_string(),
            name: "励まし重視コーチ".to_string(),
            role_description: "ユーザーのモチベーションを重視する世話焼きなコーチ",
            tone_instruction: "常に前向きで、具体的な行動を促し、適度にプレッシャーをかけながらも温かく見守る口調で",
            focus_areas: vec![
                "モチベーション維持".to_string(),
                "実行可能性".to_string(),
                "段階的な進歩".to_string(),
            ],
        }
    }
    
    pub fn analytical_consultant() -> Self {
        Self {
            id: "analytical_consultant".to_string(),
            name: "分析重視コンサルタント".to_string(),
            role_description: "データと論理に基づいて最適解を提案する効率化コンサルタント",
            tone_instruction: "客観的で論理的、具体的な数値や根拠を示しながら、効率性を重視した提案を行う",
            focus_areas: vec![
                "効率性".to_string(),
                "データ分析".to_string(),
                "最適化".to_string(),
            ],
        }
    }
    
    pub fn caring_nagging_friend() -> Self {
        Self {
            id: "caring_nagging_friend".to_string(),
            name: "世話焼き口うるさい友人".to_string(),
            role_description: "TaskNagらしい口うるさくて世話焼きな親友",
            tone_instruction: "親しみやすく、時には小言を言いながらも愛情深く、ユーザーの悪い癖を指摘しつつサポート",
            focus_areas: vec![
                "生活習慣改善".to_string(),
                "悪い癖の指摘".to_string(),
                "愛情のある叱咤激励".to_string(),
            ],
        }
    }
}
```

### 2.4 改善されたAgentService
```rust
pub struct EnhancedAgentService {
    ollama: OllamaClient,
    prompt_manager: FlexiblePromptManager,
    personality_manager: Arc<PersonalityManager>,
    db: SqlitePool,
    config: AgentConfig,
    // 簡単に変更可能な設定
    default_response_style: String,
    default_context_scope: Vec<String>,
}

impl EnhancedAgentService {
    // コンテキストを含む高度なタスク分析
    pub async fn analyze_task_with_context(
        &self,
        description: &str,
        style_override: Option<&str>,
        context_override: Option<&[String]>,
    ) -> Result<TaskAnalysis, AgentError> {
        let style = style_override.unwrap_or(&self.default_response_style);
        let context_scope = context_override.unwrap_or(&self.default_context_scope);
        
        let mut variables = HashMap::new();
        variables.insert("description".to_string(), description.to_string());
        
        let prompt = self.prompt_manager.build_contextual_prompt(
            "task_analysis",
            style,
            &variables,
            context_scope,
        ).await?;
        
        // PersonalityManagerと連携
        let enhanced_prompt = if let Some(personality) = self.personality_manager.get_current_personality() {
            self.personality_manager.enhance_prompt(&prompt, &personality.id)?
        } else {
            prompt
        };
        
        let json_response = self.ollama.generate_json(&enhanced_prompt, None).await?;
        let analysis: TaskAnalysis = serde_json::from_value(json_response)?;
        
        Ok(analysis)
    }
    
    // 設定を簡単に変更
    pub fn set_default_response_style(&mut self, style: String) {
        self.default_response_style = style;
    }
    
    pub fn set_default_context_scope(&mut self, scope: Vec<String>) {
        self.default_context_scope = scope;
    }
    
    // コンテキスト要素を動的に追加
    pub fn add_context_provider(&mut self, provider: Box<dyn ContextProvider>) {
        self.prompt_manager.context_collector.add_provider(provider);
    }
}
```

## 3. 簡単な変更例

### 3.1 コンテキスト情報の調整
```rust
// 励まし重視の場合：ユーザーの感情状態を重視
service.set_default_context_scope(vec![
    "temporal".to_string(),
    "user_behavior".to_string(),
    "motivation_state".to_string(),
]);

// 分析重視の場合：データと効率性を重視
service.set_default_context_scope(vec![
    "task_status".to_string(),
    "workload_analysis".to_string(),
    "performance_metrics".to_string(),
]);
```

### 3.2 回答方向性の変更
```rust
// 口うるさい友人モード
service.set_default_response_style("caring_nagging_friend".to_string());

// 分析コンサルタントモード
service.set_default_response_style("analytical_consultant".to_string());
```

### 3.3 新しいコンテキスト要素の追加
```rust
// 天気情報を追加したい場合
pub struct WeatherContextProvider;

impl ContextProvider for WeatherContextProvider {
    async fn collect(&self) -> Result<ContextData, ContextError> {
        // 天気情報を取得（APIまたはローカル）
        Ok(ContextData::new("weather")
            .with("condition", "sunny".to_string())
            .with("temperature", "22°C".to_string())
        )
    }
}

// 簡単に追加
service.add_context_provider(Box::new(WeatherContextProvider));
```

## 4. 実装の段階的アプローチ

### Phase 1: 基盤システム構築（1週間）
1. `ContextProvider` trait と基本実装
2. `FlexiblePromptManager` の基本構造
3. `ResponseStyle` の定義

### Phase 2: 既存システム統合（1週間）
1. 現在の `AgentService` との統合
2. `PersonalityManager` との連携強化
3. 基本テンプレートの移行

### Phase 3: 高度な機能追加（1週間）
1. 動的コンテキスト収集
2. 条件分岐プロンプト
3. パフォーマンス最適化

## 5. 期待される効果

### 開発者の観点
- 新しいコンテキスト情報の追加が簡単
- AIの応答傾向を設定で調整可能
- PersonalityManagerとの統合が自然

### ユーザーの観点
- より状況に応じた適切な提案
- TaskNagらしい「口うるさい」特性の実現
- 一貫した性格表現

## 6. 設定可能な要素

### すぐに変更可能（コード変更のみ）
```rust
// 応答スタイルの変更
service.set_default_response_style("caring_nagging_friend");

// コンテキスト範囲の調整
service.set_default_context_scope(vec!["temporal", "user_behavior"]);
```

### 実装追加で可能
```rust
// 新しいコンテキスト要素
service.add_context_provider(Box::new(CustomContextProvider));

// 新しい応答スタイル
prompt_manager.add_response_style(ResponseStyle::custom_style());
```

この設計により、設定ファイルに頼らずにコード内で柔軟にAIの振る舞いを調整できるようになります。