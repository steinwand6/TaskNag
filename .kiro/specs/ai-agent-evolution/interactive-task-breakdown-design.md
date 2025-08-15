# 対話型タスク分解フレームワーク設計書

## 概要
LLMの一方的な生成では限界があるタスク分解を、ユーザーとの対話を通じて改善していく仕組みの設計書。

## 1. 現在の問題分析

### 1.1 LLM一方的生成の限界
```rust
// 現在のAgentService::analyze_task
pub async fn analyze_task(&self, description: &str) -> Result<TaskAnalysis, AgentError> {
    // ユーザーの意図を一度で理解しようとする
    // 生成されたタスクが適切かユーザーが判断できない
    // 修正・改善のフィードバックループなし
}
```

**具体的な問題：**
- **曖昧な要求の解釈ミス**: 「ウェブサイトを作る」→ 技術スタック不明
- **粒度の不一致**: ユーザーが想定する作業レベルと乖離
- **優先順位の誤解**: 重要度の判断基準が異なる
- **コンテキスト不足**: 既存作業、制約条件が考慮されない

### 1.2 理想的な対話型アプローチ
```
ユーザー: "ウェブサイトを作りたい"
TaskNag: "どんなウェブサイトですか？個人ブログ？会社サイト？"
ユーザー: "個人のポートフォリオサイト"
TaskNag: "技術は何を使いますか？WordPress？それとも自分でコーディング？"
ユーザー: "HTMLとCSSで自分で作りたい"
TaskNag: "どのくらいの期間で完成させたいですか？"
ユーザー: "2週間くらい"
TaskNag: "わかりました！2週間でHTML/CSSポートフォリオサイトを作る計画を立てますね"
```

## 2. 対話型タスク分解フレームワーク設計

### 2.1 対話セッション管理
```rust
#[derive(Debug, Clone)]
pub struct TaskBreakdownSession {
    pub id: String,
    pub original_request: String,
    pub conversation: Vec<ConversationTurn>,
    pub extracted_requirements: TaskRequirements,
    pub current_state: BreakdownState,
    pub generated_subtasks: Vec<SubtaskCandidate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub role: ConversationRole,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub extracted_info: Option<ExtractedInfo>,
}

#[derive(Debug, Clone)]
pub enum ConversationRole {
    User,
    Assistant,
    System, // 内部処理メッセージ
}

#[derive(Debug, Clone)]
pub enum BreakdownState {
    RequirementsGathering,  // 要件聞き取り中
    RequirementConfirming,  // 要件確認中
    TaskGenerating,         // タスク生成中
    TaskRefining,           // タスク調整中
    FinalConfirmation,      // 最終確認
    Completed,              // 完了
}
```

### 2.2 要件抽出エンジン
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub project_type: Option<String>,      // "ウェブサイト作成"
    pub technology_stack: Vec<String>,     // ["HTML", "CSS"]
    pub timeline: Option<Duration>,        // 2週間
    pub complexity_level: Option<String>,  // "初心者", "中級", "上級"
    pub deliverables: Vec<String>,         // ["ホームページ", "プロフィールページ"]
    pub constraints: Vec<String>,          // ["予算なし", "平日夜のみ"]
    pub success_criteria: Vec<String>,     // ["レスポンシブ対応", "SEO対策"]
    pub dependencies: Vec<String>,         // ["ドメイン取得", "サーバー契約"]
    pub confidence_score: f32,             // 要件の確実度 (0.0-1.0)
}

pub struct RequirementsExtractor {
    patterns: HashMap<String, Vec<ExtractionPattern>>,
    context_analyzer: ContextAnalyzer,
}

#[derive(Debug, Clone)]
pub struct ExtractionPattern {
    pub pattern_type: RequirementType,
    pub keywords: Vec<String>,
    pub regex_patterns: Vec<String>,
    pub confidence_weight: f32,
}

#[derive(Debug, Clone)]
pub enum RequirementType {
    ProjectType,
    Technology,
    Timeline,
    Complexity,
    Deliverable,
    Constraint,
    SuccessCriteria,
}

impl RequirementsExtractor {
    pub fn extract_from_conversation(&self, conversation: &[ConversationTurn]) -> TaskRequirements {
        let mut requirements = TaskRequirements::default();
        
        for turn in conversation {
            if turn.role == ConversationRole::User {
                self.analyze_user_message(&turn.message, &mut requirements);
            }
        }
        
        requirements.confidence_score = self.calculate_confidence(&requirements);
        requirements
    }
    
    fn analyze_user_message(&self, message: &str, requirements: &mut TaskRequirements) {
        // 自然言語処理で要件を抽出
        // 技術スタック検出
        if let Some(tech) = self.extract_technology_mentions(message) {
            requirements.technology_stack.extend(tech);
        }
        
        // 期限検出
        if let Some(timeline) = self.extract_timeline(message) {
            requirements.timeline = Some(timeline);
        }
        
        // 成果物検出
        if let Some(deliverables) = self.extract_deliverables(message) {
            requirements.deliverables.extend(deliverables);
        }
    }
}
```

### 2.3 インテリジェント質問生成
```rust
pub struct QuestionGenerator {
    question_templates: HashMap<RequirementType, Vec<QuestionTemplate>>,
    personality_manager: Arc<PersonalityManager>,
}

#[derive(Debug, Clone)]
pub struct QuestionTemplate {
    pub template: String,
    pub priority: u8,           // 1-10 (10が最重要)
    pub required_context: Vec<String>,
    pub follow_up_patterns: Vec<String>,
}

impl QuestionGenerator {
    pub fn generate_next_question(
        &self, 
        requirements: &TaskRequirements,
        conversation_history: &[ConversationTurn]
    ) -> Option<String> {
        // 不足している要件を特定
        let missing_requirements = self.identify_missing_requirements(requirements);
        
        // 最も重要な不足要件を選択
        if let Some(missing_req) = self.prioritize_missing_requirements(&missing_requirements) {
            // 性格を考慮した質問を生成
            let base_question = self.get_question_template(&missing_req)?;
            self.personalize_question(base_question, requirements)
        } else {
            None // 全要件が揃った
        }
    }
    
    fn get_question_template(&self, req_type: &RequirementType) -> Option<String> {
        match req_type {
            RequirementType::Technology => Some(
                "技術的なことについて聞かせてください。何を使って作りたいですか？\n\
                例：WordPress、HTML/CSS、React、Pythonなど".to_string()
            ),
            RequirementType::Timeline => Some(
                "いつまでに完成させたいですか？\n\
                急ぎでしょうか、それともじっくり時間をかけて？".to_string()
            ),
            RequirementType::Complexity => Some(
                "どのくらい本格的に作りたいですか？\n\
                シンプルでいい？それともしっかりしたものを？".to_string()
            ),
            RequirementType::Deliverable => Some(
                "具体的にはどんなものを作りたいですか？\n\
                どんなページや機能が必要でしょう？".to_string()
            ),
            _ => None,
        }
    }
    
    fn personalize_question(&self, base_question: String, requirements: &TaskRequirements) -> Option<String> {
        if let Some(personality) = self.personality_manager.get_current_personality() {
            // 性格に応じて質問の口調を調整
            let enhanced_question = match personality.id.as_str() {
                "caring_childhood_friend" => {
                    format!("ねえねえ、{}気になるから教えて〜", base_question)
                },
                "polite_secretary" => {
                    format!("恐れ入りますが、{}お聞かせいただけますでしょうか。", base_question)
                },
                "enthusiastic_coach" => {
                    format!("よし！{}一緒に決めていこう！", base_question)
                },
                _ => base_question,
            };
            Some(enhanced_question)
        } else {
            Some(base_question)
        }
    }
}
```

### 2.4 適応的タスク生成
```rust
pub struct AdaptiveTaskGenerator {
    base_generator: TaskGenerator,
    feedback_analyzer: FeedbackAnalyzer,
    iteration_history: Vec<GenerationIteration>,
}

#[derive(Debug, Clone)]
pub struct GenerationIteration {
    pub iteration_number: u32,
    pub input_requirements: TaskRequirements,
    pub generated_tasks: Vec<SubtaskCandidate>,
    pub user_feedback: Option<UserFeedback>,
    pub adjustments_made: Vec<Adjustment>,
}

#[derive(Debug, Clone)]
pub struct UserFeedback {
    pub overall_satisfaction: f32,      // 0.0-1.0
    pub granularity_preference: GranularityFeedback,
    pub priority_corrections: Vec<PriorityCorrection>,
    pub missing_tasks: Vec<String>,
    pub unnecessary_tasks: Vec<usize>,   // インデックス
    pub timeline_adjustments: Option<Duration>,
    pub free_text_feedback: Option<String>,
}

#[derive(Debug, Clone)]
pub enum GranularityFeedback {
    TooDetailed,        // 細かすぎる
    TooHighLevel,       // 大雑把すぎる
    JustRight,          // ちょうど良い
    Mixed(Vec<usize>),  // 一部のタスクが細かすぎる
}

impl AdaptiveTaskGenerator {
    pub async fn generate_tasks_iteratively(
        &mut self,
        requirements: &TaskRequirements,
        previous_feedback: Option<&UserFeedback>,
    ) -> Result<Vec<SubtaskCandidate>, TaskGenerationError> {
        // 前回のフィードバックを分析
        let adjustments = if let Some(feedback) = previous_feedback {
            self.feedback_analyzer.analyze_feedback(feedback)?
        } else {
            vec![]
        };
        
        // 調整を適用してタスク生成
        let mut tasks = self.base_generator.generate_tasks(requirements).await?;
        
        // フィードバックに基づく調整を適用
        for adjustment in &adjustments {
            self.apply_adjustment(&mut tasks, adjustment)?;
        }
        
        // 生成履歴を記録
        self.iteration_history.push(GenerationIteration {
            iteration_number: self.iteration_history.len() as u32 + 1,
            input_requirements: requirements.clone(),
            generated_tasks: tasks.clone(),
            user_feedback: previous_feedback.cloned(),
            adjustments_made: adjustments,
        });
        
        Ok(tasks)
    }
    
    fn apply_adjustment(
        &self, 
        tasks: &mut Vec<SubtaskCandidate>, 
        adjustment: &Adjustment
    ) -> Result<(), TaskGenerationError> {
        match adjustment {
            Adjustment::IncreaseGranularity(task_indices) => {
                // 指定されたタスクをより細かく分解
                for &index in task_indices {
                    if let Some(task) = tasks.get_mut(index) {
                        let subtasks = self.break_down_further(task)?;
                        // 元のタスクを細分化されたタスクで置換
                        tasks.splice(index..=index, subtasks);
                    }
                }
            },
            Adjustment::DecreaseGranularity(task_indices) => {
                // 指定されたタスクをまとめる
                self.merge_tasks(tasks, task_indices)?;
            },
            Adjustment::AdjustPriorities(corrections) => {
                // 優先度を調整
                for correction in corrections {
                    if let Some(task) = tasks.get_mut(correction.task_index) {
                        task.priority = correction.new_priority;
                    }
                }
            },
            Adjustment::RemoveTasks(indices) => {
                // 不要なタスクを削除（逆順で削除）
                let mut sorted_indices = indices.clone();
                sorted_indices.sort_by(|a, b| b.cmp(a));
                for &index in &sorted_indices {
                    if index < tasks.len() {
                        tasks.remove(index);
                    }
                }
            },
            Adjustment::AddTasks(new_tasks) => {
                // 新しいタスクを追加
                tasks.extend(new_tasks.clone());
            }
        }
        Ok(())
    }
}
```

## 3. 対話フロー設計

### 3.1 メイン対話フロー
```
1. 初期要求受付
   ユーザー: "ウェブサイトを作りたい"
   ↓
2. 要件聞き取り（インテリジェント質問）
   TaskNag: "どんなウェブサイトですか？" → 回答
   TaskNag: "技術は何を使いますか？" → 回答
   TaskNag: "いつまでに？" → 回答
   ↓
3. 要件確認
   TaskNag: "HTML/CSSで個人ポートフォリオを2週間で作るということですね？"
   ユーザー: "はい"
   ↓
4. 初回タスク生成
   TaskNag: "タスクを考えてみますね..."（生成中）
   TaskNag: "こんな感じはどうでしょう？"（タスクリスト提示）
   ↓
5. フィードバック収集
   TaskNag: "どう思いますか？細かすぎる？大雑把すぎる？"
   ユーザー: "もう少し細かくしてほしい"
   ↓
6. タスク調整
   TaskNag: "わかりました、もう少し細かく分けますね"
   （調整されたタスクリスト提示）
   ↓
7. 最終確認
   TaskNag: "これで良さそうですか？"
   ユーザー: "はい"
   ↓
8. タスク保存・完了
```

### 3.2 フィードバック収集UI
```
┌─ タスク調整 ─────────────────────────────┐
│                                        │
│ 生成されたタスクはいかがですか？       │
│                                        │
│ 全体的な印象:                          │
│ ○ ちょうど良い                         │
│ ○ 細かすぎる                           │
│ ○ 大雑把すぎる                         │
│                                        │
│ 個別調整:                              │
│ ┌────────────────────────────────────┐ │
│ │ ☑ 1. HTML基本構造を作成            │ │
│ │ ☑ 2. CSSスタイルシート作成         │ │
│ │ ☐ 3. レスポンシブ対応              │ │
│ │ ☑ 4. プロフィール内容作成          │ │
│ │ ☐ 5. 作品ギャラリー作成            │ │
│ └────────────────────────────────────┘ │
│ ☑=残す ☐=不要 ←クリックで切り替え     │
│                                        │
│ 追加したいタスク:                      │
│ [  SEO対策をしたい                   ] │
│                                        │
│ その他の要望:                          │
│ [ もう少しデザインに時間をかけたい   ] │
│                                        │
│              [調整] [このまま保存]      │
└────────────────────────────────────────┘
```

## 4. 実装計画

### Phase 1: 基本対話システム（1週間）
1. **TaskBreakdownSession** 管理
2. **RequirementsExtractor** の基本機能
3. **シンプルな質問生成**

### Phase 2: インテリジェント機能（1週間）
1. **QuestionGenerator** の高度化
2. **AdaptiveTaskGenerator** 実装
3. **フィードバック分析**

### Phase 3: UI統合（3日）
1. **対話UI** の実装
2. **フィードバック収集画面**
3. **Tauri Commands** 統合

## 5. 期待される効果

### 精度向上
- **要件の明確化**: 曖昧な要求の解決
- **適切な粒度**: ユーザー好みに調整
- **実用的なタスク**: コンテキストを考慮

### ユーザー体験
- **理解している感**: AIが自分の意図を理解
- **協働感**: 一緒にタスクを作り上げる
- **学習効果**: 良いタスク分解を学べる

この対話型システムにより、LLMの一方的生成の限界を克服し、真にユーザーに役立つタスク分解が実現できます。