# プロアクティブ通知システム設計書

## 概要
TaskNagの「口うるさくて世話焼きな」特性を最大限活かした、性格統合型プロアクティブ通知システムの設計書。既存の通知機能を超えた、AIが能動的に口出し・励ましを行う独自路線システム。

## 1. TaskNag独自のプロアクティブ思想

### 1.1 従来の通知システムとの差別化
```
従来の通知システム:
├─ 期日リマインダー（機械的）
├─ 進捗確認（定期的）
└─ 完了通知（事後報告）

TaskNagプロアクティブシステム:
├─ 🏠 生活パターン監視（「また夜更かし？」）
├─ 💭 行動予測・先回り提案（「明日忙しくなりそうね」）
├─ 😤 愛のある口出し（「そのタスク、後回しにしすぎよ」）
├─ 💪 タイミング見計らった励まし（「疲れてそう、休憩は？」）
├─ 🎯 成長サポート（「前より早くできるようになったね」）
└─ 🤝 相談・雑談相手（「今日はどう？何か困ってる？」）
```

### 1.2 TaskNagらしい「おせっかい」の価値
- **予防的ケア**: 問題が起きる前に気づいて声かけ
- **感情的サポート**: データだけでなく気持ちに寄り添う
- **習慣形成支援**: 良い習慣を褒め、悪い習慣を愛情で正す
- **成長実感**: 小さな改善も見逃さず褒める

## 2. 性格統合プロアクティブエンジン

### 2.1 性格別プロアクティブ戦略
```rust
#[derive(Debug, Clone)]
pub struct PersonalityProactiveStrategy {
    pub personality_id: String,
    pub monitoring_focus: Vec<MonitoringAspect>,
    pub intervention_style: InterventionStyle,
    pub message_templates: HashMap<ProactiveEventType, Vec<MessageTemplate>>,
    pub timing_preferences: TimingPreferences,
}

#[derive(Debug, Clone)]
pub enum MonitoringAspect {
    WorkLifeBalance,      // 仕事と休息のバランス
    TaskCompletion,       // タスク完了パターン
    StressIndicators,     // ストレス兆候
    MotivationLevel,      // モチベーション状態
    ProductivityTrends,   // 生産性の変化
    HealthHabits,        // 生活習慣
}

#[derive(Debug, Clone)]
pub enum InterventionStyle {
    GentleNudge,         // そっと後押し
    FriendlyReminder,    // 親しげなリマインド
    ConcernedWarning,    // 心配からの警告
    EnthusiasticCheer,   // 元気な応援
    WiseAdvice,          // 経験からのアドバイス
}

impl PersonalityProactiveStrategy {
    pub fn for_caring_childhood_friend() -> Self {
        Self {
            personality_id: "caring_childhood_friend".to_string(),
            monitoring_focus: vec![
                MonitoringAspect::WorkLifeBalance,
                MonitoringAspect::StressIndicators,
                MonitoringAspect::HealthHabits,
            ],
            intervention_style: InterventionStyle::ConcernedWarning,
            message_templates: hashmap! {
                ProactiveEventType::LateNightWork => vec![
                    MessageTemplate::new("また夜更かし？体壊すよ〜", 0.9),
                    MessageTemplate::new("もう、こんな時間まで働いちゃダメでしょ", 0.8),
                ],
                ProactiveEventType::TaskDelayed => vec![
                    MessageTemplate::new("あのタスク、まだやってないでしょ？心配になっちゃう", 0.9),
                    MessageTemplate::new("締切大丈夫？手伝えることがあったら言ってね", 0.7),
                ],
                ProactiveEventType::GoodProgress => vec![
                    MessageTemplate::new("おお、順調じゃない！その調子その調子", 0.8),
                    MessageTemplate::new("頑張ってるね〜、えらいえらい", 0.9),
                ],
            },
            timing_preferences: TimingPreferences {
                preferred_hours: vec![9, 12, 15, 18, 21], // 9時、昼、3時、6時、9時
                avoid_hours: vec![22, 23, 0, 1, 2, 3, 4, 5, 6, 7], // 夜10時〜朝7時は避ける
                max_daily_interventions: 5,
                min_interval_minutes: 120, // 最低2時間は空ける
            },
        }
    }
    
    pub fn for_enthusiastic_coach() -> Self {
        Self {
            personality_id: "enthusiastic_coach".to_string(),
            monitoring_focus: vec![
                MonitoringAspect::TaskCompletion,
                MonitoringAspect::MotivationLevel,
                MonitoringAspect::ProductivityTrends,
            ],
            intervention_style: InterventionStyle::EnthusiasticCheer,
            message_templates: hashmap! {
                ProactiveEventType::TaskDelayed => vec![
                    MessageTemplate::new("さあ、ここが踏ん張りどころだ！一緒に乗り越えよう！", 0.9),
                    MessageTemplate::new("大丈夫、君ならできる！まずは小さな一歩から！", 0.8),
                ],
                ProactiveEventType::LowMotivation => vec![
                    MessageTemplate::new("調子が上がらない？そんな日もある！でも諦めるな！", 0.9),
                    MessageTemplate::new("今日は少しペースを落としても良い、継続が大事だ！", 0.7),
                ],
            },
            timing_preferences: TimingPreferences {
                preferred_hours: vec![8, 10, 14, 16, 19], // より積極的なタイミング
                max_daily_interventions: 8,
                min_interval_minutes: 90,
                ..Default::default()
            },
        }
    }
}
```

### 2.2 プロアクティブ行動検出エンジン
```rust
pub struct ProactiveBehaviorDetector {
    user_patterns: UserBehaviorPatterns,
    current_context: CurrentContext,
    detection_rules: Vec<DetectionRule>,
    personality_strategy: PersonalityProactiveStrategy,
}

#[derive(Debug, Clone)]
pub struct UserBehaviorPatterns {
    pub work_hours: TimeRange,
    pub typical_task_duration: HashMap<String, Duration>,
    pub break_patterns: Vec<BreakPattern>,
    pub stress_indicators: Vec<StressIndicator>,
    pub motivation_cycles: MotivationPattern,
    pub sleep_schedule: SleepPattern,
}

#[derive(Debug, Clone)]
pub struct CurrentContext {
    pub current_time: DateTime<Utc>,
    pub active_tasks: Vec<Task>,
    pub recent_activity: Vec<UserActivity>,
    pub current_workload: WorkloadLevel,
    pub last_break: Option<DateTime<Utc>>,
    pub today_completion_rate: f32,
}

#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub rule_id: String,
    pub event_type: ProactiveEventType,
    pub conditions: Vec<Condition>,
    pub priority: u8,
    pub cooldown_minutes: u32,
}

#[derive(Debug, Clone)]
pub enum ProactiveEventType {
    // 生活リズム関連（アプリ使用パターンベース）
    LateNightActivity,       // 夜遅いアプリ使用
    EarlyMorningActivity,    // 早朝アプリ使用
    HighTaskLoad,           // 大量のタスク登録
    FrequentChecking,       // 頻繁なアプリ確認
    
    // タスク関連
    TaskDelayed,            // タスク遅延
    TaskStuck,              // タスク停滞
    DeadlineApproaching,    // 締切接近
    
    // モチベーション関連
    LowMotivation,          // 低モチベーション
    HighStress,             // 高ストレス
    ProductivityDrop,       // 生産性低下
    
    // ポジティブイベント
    GoodProgress,           // 順調な進捗
    TaskCompleted,          // タスク完了
    ProductivityImproved,   // 生産性向上
    
    // 相談・雑談
    IdleTooLong,           // 長時間無活動
    SeemsBored,            // 退屈そう
    NeedsEncouragement,    // 励ましが必要
}

impl ProactiveBehaviorDetector {
    pub async fn detect_events(&self) -> Vec<ProactiveEvent> {
        let mut events = Vec::new();
        
        // 各検出ルールを実行
        for rule in &self.detection_rules {
            if self.evaluate_conditions(&rule.conditions) {
                events.push(ProactiveEvent {
                    event_type: rule.event_type.clone(),
                    priority: rule.priority,
                    detected_at: Utc::now(),
                    context: self.current_context.clone(),
                    rule_id: rule.rule_id.clone(),
                });
            }
        }
        
        // 優先度とクールダウンでフィルタリング
        self.filter_events(events)
    }
    
    fn evaluate_conditions(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|condition| {
            match condition {
                Condition::TimeRange(start, end) => {
                    let current_hour = self.current_context.current_time.hour();
                    current_hour >= *start && current_hour <= *end
                },
                Condition::WorkingTooLong(duration) => {
                    self.current_context.recent_activity
                        .iter()
                        .filter(|a| a.activity_type == ActivityType::TaskWork)
                        .map(|a| a.duration)
                        .sum::<Duration>() > *duration
                },
                Condition::TaskOverdue(days) => {
                    self.current_context.active_tasks
                        .iter()
                        .any(|t| t.is_overdue_by_days(*days))
                },
                Condition::LowCompletionRate(threshold) => {
                    self.current_context.today_completion_rate < *threshold
                },
                Condition::NoBreakSince(duration) => {
                    if let Some(last_break) = self.current_context.last_break {
                        Utc::now().signed_duration_since(last_break) > *duration
                    } else {
                        true
                    }
                },
                // 他の条件...
            }
        })
    }
}
```

### 2.3 TaskNag専用メッセージ生成
```rust
pub struct TaskNagMessageGenerator {
    personality_manager: Arc<PersonalityManager>,
    message_personalizer: MessagePersonalizer,
    context_aware_generator: ContextAwareGenerator,
}

#[derive(Debug, Clone)]
pub struct ProactiveMessage {
    pub message_type: ProactiveMessageType,
    pub content: String,
    pub tone: MessageTone,
    pub urgency: UrgencyLevel,
    pub suggested_actions: Vec<SuggestedAction>,
    pub follow_up_timing: Option<Duration>,
    pub personality_signature: String,
}

#[derive(Debug, Clone)]
pub enum ProactiveMessageType {
    // 世話焼き系
    HealthConcern,          // 健康心配
    WorkLifeBalance,        // 生活バランス
    GentleNagging,          // 優しい小言
    
    // 励まし系  
    Encouragement,          // 励まし
    Celebration,            // お祝い
    MotivationBoost,        // やる気アップ
    
    // 提案系
    ProductivityTip,        // 生産性向上提案
    TaskSuggestion,         // タスク提案
    BreakSuggestion,        // 休憩提案
    
    // 相談系
    CheckIn,                // 様子伺い
    ConversationStarter,    // 会話のきっかけ
    ListeningEar,          // 相談受付
}

impl TaskNagMessageGenerator {
    pub async fn generate_proactive_message(
        &self,
        event: &ProactiveEvent,
        user_context: &UserContext,
    ) -> Result<ProactiveMessage, MessageGenerationError> {
        // 現在の性格を取得
        let personality = self.personality_manager.get_current_personality()
            .ok_or(MessageGenerationError::NoPersonality)?;
            
        // 性格別戦略を取得
        let strategy = PersonalityProactiveStrategy::for_personality(&personality.id);
        
        // ベースメッセージテンプレートを選択
        let base_template = self.select_template(&event.event_type, &strategy)?;
        
        // コンテキストを考慮してメッセージを生成
        let context_enhanced_message = self.context_aware_generator
            .enhance_with_context(base_template, user_context, event).await?;
            
        // 性格特性を適用
        let personalized_message = self.message_personalizer
            .apply_personality(context_enhanced_message, personality)?;
            
        // 提案アクションを生成
        let suggested_actions = self.generate_suggested_actions(event, user_context)?;
        
        Ok(ProactiveMessage {
            message_type: self.determine_message_type(&event.event_type),
            content: personalized_message,
            tone: strategy.intervention_style.to_message_tone(),
            urgency: self.calculate_urgency(event),
            suggested_actions,
            follow_up_timing: self.calculate_follow_up_timing(event),
            personality_signature: personality.name.clone(),
        })
    }
    
    fn select_template(
        &self, 
        event_type: &ProactiveEventType, 
        strategy: &PersonalityProactiveStrategy
    ) -> Result<String, MessageGenerationError> {
        strategy.message_templates
            .get(event_type)
            .and_then(|templates| templates.choose(&mut rand::thread_rng()))
            .map(|template| template.content.clone())
            .ok_or(MessageGenerationError::NoTemplate)
    }
}
```

## 3. TaskNag独自通知体験設計

### 3.1 通知表示パターン
```
Pattern 1: さりげない気遣い
┌─────────────────────────────────────┐
│ 🤗 お節介な幼馴染                  │
│                                     │
│ "今日はタスクがたくさんあるのね〜   │
│  無理しちゃダメよ？                 │
│  たまには休憩も大事だからね"        │
│                                     │
│         [休憩する] [もう少し続ける]  │
└─────────────────────────────────────┘

Pattern 2: 愛のある小言
┌─────────────────────────────────────┐
│ 😤 お節介な幼馴染                  │
│                                     │
│ "あのレポートのタスク、              │
│  もう3日も手つけてないでしょ？      │
│  心配になっちゃうよ〜               │
│  今日少しでもやってみない？"        │
│                                     │
│    [今やる] [明日やる] [話を聞いて]  │
└─────────────────────────────────────┘

Pattern 3: 嬉しい報告
┌─────────────────────────────────────┐
│ 🎉 お節介な幼馴染                  │
│                                     │
│ "やったじゃない！                   │
│  今週はタスク完了率80%よ！          │
│  先週より20%も上がってる〜          │
│  この調子でいこうね！"              │
│                                     │
│              [ありがとう] [次の目標] │
└─────────────────────────────────────┘

Pattern 4: 相談モード
┌─────────────────────────────────────┐
│ 💭 お節介な幼馴染                  │
│                                     │
│ "なんか今日、                       │
│  いつもより元気ないみたい...        │
│  何か困ってることある？             │
│  良かったら話聞くよ？"              │
│                                     │
│      [話したい] [大丈夫] [後で話す]  │
└─────────────────────────────────────┘
```

### 3.2 通知タイミング戦略
```rust
pub struct NotificationTimingEngine {
    user_schedule: UserSchedule,
    interruption_cost_calculator: InterruptionCostCalculator,
    attention_window_detector: AttentionWindowDetector,
}

#[derive(Debug, Clone)]
pub struct OptimalTiming {
    pub ideal_time: DateTime<Utc>,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub interruption_cost: f32,    // 0.0-1.0 (低いほど良い)
    pub attention_score: f32,      // 0.0-1.0 (高いほど良い)
    pub personality_alignment: f32, // 性格との適合度
}

impl NotificationTimingEngine {
    pub async fn find_optimal_timing(
        &self,
        message: &ProactiveMessage,
        user_context: &UserContext,
    ) -> OptimalTiming {
        // ユーザーの作業パターンを分析
        let work_pattern = self.analyze_current_work_pattern(user_context);
        
        // 中断コストを計算
        let interruption_cost = self.interruption_cost_calculator
            .calculate_cost(&work_pattern);
            
        // 注意力の状態を検出
        let attention_score = self.attention_window_detector
            .detect_attention_level(user_context);
            
        // 性格との適合度チェック
        let personality_alignment = self.check_personality_timing_alignment(message);
        
        // 最適タイミングを決定
        self.calculate_optimal_window(
            interruption_cost,
            attention_score,
            personality_alignment
        )
    }
    
    fn analyze_current_work_pattern(&self, context: &UserContext) -> WorkPattern {
        match context.current_activity {
            Some(Activity::DeepWork) => WorkPattern::DeepFocus,
            Some(Activity::LightTask) => WorkPattern::LightWork,
            Some(Activity::Break) => WorkPattern::RestTime,
            None => WorkPattern::Idle,
        }
    }
}
```

### 3.3 相互作用・学習機能
```rust
pub struct ProactiveInteractionLearner {
    interaction_history: Vec<ProactiveInteraction>,
    effectiveness_tracker: EffectivenessTracker,
    personalization_engine: PersonalizationEngine,
}

#[derive(Debug, Clone)]
pub struct ProactiveInteraction {
    pub message_id: String,
    pub event_type: ProactiveEventType,
    pub personality_used: String,
    pub user_response: UserResponse,
    pub outcome: InteractionOutcome,
    pub timing_quality: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum UserResponse {
    Positive(String),           // "ありがとう"、"助かった"
    Neutral(String),            // "わかった"、"後で"
    Negative(String),           // "うるさい"、"忙しい"
    ActionTaken(ActionType),    // 実際に休憩した、タスクを開始した
    Ignored,                    // 反応なし
}

#[derive(Debug, Clone)]
pub enum InteractionOutcome {
    SuccessfulIntervention,     // 問題解決に貢献
    TimingProblem,              // タイミングが悪かった
    PersonalityMismatch,        // 性格が合わなかった
    OverIntervention,           // 介入しすぎ
    UnderIntervention,          // 介入不足
}

impl ProactiveInteractionLearner {
    pub async fn learn_from_interaction(&mut self, interaction: ProactiveInteraction) {
        // 効果測定
        let effectiveness = self.effectiveness_tracker
            .measure_effectiveness(&interaction);
            
        // パーソナライゼーション調整
        self.personalization_engine
            .adjust_based_on_feedback(&interaction);
            
        // 履歴に記録
        self.interaction_history.push(interaction);
        
        // 学習パターンの更新
        self.update_learning_patterns().await;
    }
    
    pub fn get_personalization_suggestions(&self) -> PersonalizationSuggestions {
        PersonalizationSuggestions {
            preferred_timing_windows: self.extract_preferred_timings(),
            effective_message_types: self.identify_effective_messages(),
            personality_preferences: self.analyze_personality_preferences(),
            intervention_frequency: self.calculate_optimal_frequency(),
        }
    }
}
```

## 4. 実装アーキテクチャ

### 4.1 システム構成
```rust
pub struct TaskNagProactiveSystem {
    // コア検出エンジン
    behavior_detector: ProactiveBehaviorDetector,
    message_generator: TaskNagMessageGenerator,
    timing_engine: NotificationTimingEngine,
    
    // 統合マネージャー
    personality_manager: Arc<PersonalityManager>,
    context_collector: ContextCollector,
    interaction_learner: ProactiveInteractionLearner,
    
    // 設定・制御
    user_preferences: ProactivePreferences,
    active_sessions: HashMap<String, ProactiveSession>,
    
    // データベース
    db: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct ProactivePreferences {
    pub enabled: bool,
    pub max_daily_interventions: u32,
    pub quiet_hours: Vec<TimeRange>,
    pub preferred_intervention_types: Vec<ProactiveMessageType>,
    pub sensitivity_level: f32,  // 0.0-1.0
}

impl TaskNagProactiveSystem {
    pub async fn start_monitoring(&mut self) -> Result<(), ProactiveError> {
        // バックグラウンドタスクとして監視開始
        let detector = self.behavior_detector.clone();
        let generator = self.message_generator.clone();
        let timing_engine = self.timing_engine.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_minutes(5));
            
            loop {
                interval.tick().await;
                
                // イベント検出
                if let Ok(events) = detector.detect_events().await {
                    for event in events {
                        // メッセージ生成
                        if let Ok(message) = generator.generate_proactive_message(&event, &user_context).await {
                            // 最適タイミング計算
                            let timing = timing_engine.find_optimal_timing(&message, &user_context).await;
                            
                            // 通知スケジューリング
                            Self::schedule_notification(message, timing).await;
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
}
```

### 4.2 Tauri統合
```rust
// Tauri Commands
#[tauri::command]
pub async fn enable_proactive_mode(
    system: tauri::State<'_, Mutex<TaskNagProactiveSystem>>,
    preferences: ProactivePreferences,
) -> Result<(), String> {
    let mut system = system.lock().await;
    system.user_preferences = preferences;
    system.start_monitoring().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn respond_to_proactive_message(
    system: tauri::State<'_, Mutex<TaskNagProactiveSystem>>,
    message_id: String,
    response: UserResponse,
) -> Result<(), String> {
    let mut system = system.lock().await;
    system.record_user_response(message_id, response).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_proactive_insights(
    system: tauri::State<'_, Mutex<TaskNagProactiveSystem>>,
) -> Result<ProactiveInsights, String> {
    let system = system.lock().await;
    Ok(system.generate_insights())
}
```

## 5. 実装計画

### Phase 1: 基本検出システム（1週間）
1. **ProactiveBehaviorDetector** の基本実装
2. **基本的な検出ルール**（夜更かし、長時間作業、タスク遅延）
3. **PersonalityManager統合**

### Phase 2: メッセージ生成（1週間）
1. **TaskNagMessageGenerator** 実装
2. **性格別メッセージテンプレート**
3. **コンテキスト考慮メッセージ生成**

### Phase 3: タイミング・学習（1週間）
1. **NotificationTimingEngine** 実装
2. **ProactiveInteractionLearner** 基本機能
3. **効果測定・改善サイクル**

### Phase 4: UI統合・完成（3日）
1. **プロアクティブ通知UI**
2. **設定画面**
3. **システム統合テスト**

## 6. 期待される効果

### ユーザー体験の革新
- **能動的パートナー感**: AIが気にかけてくれている実感
- **生活リズム改善**: 健康的な作業習慣の形成
- **ストレス軽減**: 問題の早期発見・予防
- **モチベーション維持**: 適切なタイミングでの励まし

### TaskNagの差別化
- **感情的なつながり**: 単なるツールから相棒へ
- **個性的な体験**: 性格による多様な関わり方
- **成長を共有**: ユーザーの変化を一緒に喜ぶ
- **予防的サポート**: 問題になる前の対処

このプロアクティブ通知システムにより、TaskNagは真に「口うるさくて世話焼きな」AIアシスタントとして、ユーザーの生産性向上と幸福度向上に貢献できます。