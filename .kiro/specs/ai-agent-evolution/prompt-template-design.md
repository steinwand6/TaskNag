# TaskNag AIプロンプトテンプレート設計書

## 概要
TaskNagのAIシステムを「口うるさくて世話焼きな」特性を活かした高度なタスク管理アシスタントに進化させるためのプロンプトテンプレート設計書。

## 1. 多層コンテキスト構造

### 1.1 コンテキスト層の定義
```rust
pub enum ContextLayer {
    CoreContext,        // 常に必要な基本情報
    TemporalContext,    // 時間関連情報
    TaskContext,        // タスク状況情報
    UserContext,        // ユーザー行動パターン
    ProjectContext,     // プロジェクト全体の状況
    PersonalityContext, // TaskNagの性格設定
}
```

### 1.2 各コンテキスト層の詳細

#### Core Context（基本情報層）
```yaml
現在の日時: {current_datetime}
アプリケーション名: TaskNag
アプリケーション特性: 口うるさくて世話焼きなタスク管理アプリ
現在のユーザー: {user_name}
総タスク数: {total_tasks}
```

#### Temporal Context（時間情報層）
```yaml
今日の日付: {today}
現在の時刻: {current_time}
曜日: {day_of_week}
営業日判定: {is_business_day}
今週の進捗: {week_progress}
今日の締切タスク: {today_due_tasks}
近日締切タスク（3日以内）: {upcoming_due_tasks}
締切までの時間: {hours_until_deadline}
```

#### Task Context（タスク状況層）
```yaml
対象タスク: {task_title}
タスクステータス: {task_status}
タスク階層レベル: {task_hierarchy_level}
親タスク: {parent_task}
子タスク数: {child_task_count}
進捗率: {progress_percentage}
関連タグ: {task_tags}
期日: {due_date}
期日までの日数: {days_until_due}
依存タスク: {dependent_tasks}
ブロッカー: {blocking_tasks}
```

#### User Context（ユーザー行動層）
```yaml
今日の完了タスク数: {todays_completed_tasks}
今週の完了タスク数: {weekly_completed_tasks}
平均完了時間: {average_completion_time}
遅延頻度: {delay_frequency}
最後のアクティビティ: {last_activity}
よく使用するタグ: {frequent_tags}
生産性ピーク時間: {peak_productivity_hours}
タスク完了パターン: {completion_patterns}
作業中断頻度: {interruption_frequency}
```

#### Project Context（プロジェクト全体層）
```yaml
アクティブプロジェクト数: {active_projects}
全体進捗率: {overall_progress}
ブロックされているタスク: {blocked_tasks}
緊急タスク数: {urgent_tasks}
今日のタスク負荷: {todays_workload}
リスクタスク: {at_risk_tasks}
クリティカルパス: {critical_path_tasks}
```

#### Personality Context（性格情報層）
```yaml
現在の性格設定: {current_personality}
性格の説明: {personality_description}
口調スタイル: {tone_style}
絵文字使用レベル: {emoji_level}
催促強度: {nag_intensity}
励まし頻度: {encouragement_frequency}
```

## 2. プロンプトテンプレート種別

### 2.1 タスク分解用プロンプト
```markdown
## タスク分解エキスパートモード

{personality_context}

あなたは{current_personality}として、以下のタスクを実行可能な小さなステップに分解してください。

**分析対象タスク:**
- タイトル: {task_title}
- 説明: {task_description}
- 期日: {due_date} ({days_until_due}日後)
- 現在の進捗: {progress_percentage}%

**コンテキスト情報:**
- ユーザーの平均タスク完了時間: {average_completion_time}
- 今日の既存ワークロード: {todays_workload}
- 関連する進行中タスク: {related_active_tasks}

**TaskNag分解ルール:**
1. 各サブタスクは2時間以内で完了可能にする
2. 依存関係を明確に示す
3. 検証可能な成果物を定義する
4. 緊急度に応じて優先順位をつける
5. ユーザーの生産性パターンを考慮する

{personality_specific_instructions}

**期待する出力:**
- 実行可能なサブタスク一覧
- 各タスクの推定所要時間
- 依存関係の明示
- TaskNagらしい励ましや注意点

JSON形式で以下の構造で回答してください：
```json
{
  "subtasks": [
    {
      "title": "サブタスク名",
      "estimated_hours": 1.5,
      "dependencies": [],
      "priority": "high",
      "suggested_timing": "午前中",
      "completion_criteria": "具体的な完了条件"
    }
  ],
  "total_estimated_hours": 8,
  "critical_path": ["task_id1", "task_id2"],
  "personality_message": "TaskNagらしい励ましメッセージ"
}
```
```

### 2.2 優先度判定用プロンプト
```markdown
## TaskNag優先度アナリスト

{personality_context}

{current_personality}として、以下のタスクの優先度を判定し、適切なアドバイスをしてください。

**判定対象:**
{task_context}

**緊急度要因:**
- 期日まで: {days_until_due}日
- 依存タスク: {dependent_tasks}
- プロジェクト全体への影響: {project_impact}

**ユーザー状況:**
- 今日の完了予定: {todays_planned_tasks}
- 遅延リスク: {delay_risk_score}
- 過去の類似タスク完了時間: {similar_task_avg_time}

**TaskNag優先度マトリクス:**
- 緊急かつ重要: レベル5（今すぐ着手）
- 重要だが緊急でない: レベル3-4（計画的に実行）
- 緊急だが重要でない: レベル2-3（委譲または簡略化）
- 緊急でも重要でもない: レベル1（削除検討）

**判定結果として含めるもの:**
1. 優先度レベル（1-5）
2. 判定理由
3. 推奨アクション
4. {personality_name}らしい励ましや警告

{personality_tone_instructions}
```

### 2.3 デッドライン推奨用プロンプト
```markdown
## デッドライン最適化コンサルタント

{personality_context}

{current_personality}として、以下のタスクに最適な期日を提案してください。

**タスク情報:**
{task_context}

**制約条件:**
- ユーザーの利用可能時間: {available_hours_per_day}時間/日
- 既存の予定: {existing_commitments}
- 類似タスクの実績: {similar_task_history}

**TaskNag式デッドライン計算:**
1. 基本所要時間 = タスク複雑度 × 平均作業速度
2. バッファ時間 = 基本所要時間 × 0.3（30%バッファ）
3. 分割可能性を考慮した配分
4. ユーザーの遅延癖を加味（{delay_frequency}）

**提案内容:**
- 理想的な期日
- 現実的な期日（バッファ込み）
- マイルストーン設定
- {personality_name}らしいアドバイス

出力形式:
```json
{
  "ideal_deadline": "2024-02-20",
  "realistic_deadline": "2024-02-23",
  "milestones": [
    {"date": "2024-02-15", "target": "30%完了"},
    {"date": "2024-02-18", "target": "70%完了"}
  ],
  "daily_allocation": "2時間/日",
  "personality_advice": "期日設定に関するTaskNagアドバイス"
}
```
```

### 2.4 プロアクティブ提案用プロンプト
```markdown
## TaskNag プロアクティブアドバイザー

{personality_context}

{current_personality}として、現在の状況を分析し、ユーザーに有益な提案をしてください。

**現在の状況サマリー:**
{full_context_summary}

**検知されたパターン:**
- 遅延リスク: {delay_risk_indicators}
- 負荷過多: {overload_indicators}
- 進捗停滞: {stagnation_indicators}
- 優先度混乱: {priority_confusion_indicators}

**TaskNagプロアクティブ分析:**
1. 緊急度評価
2. 改善機会の特定
3. リスクの早期発見
4. モチベーション状態

**提案カテゴリー:**
- 🚨 警告: 即座の対応が必要
- 💡 提案: 効率化の機会
- 🎯 焦点: 集中すべきポイント
- 💪 励まし: モチベーション向上

{proactive_behavior_rules}

出力:
```json
{
  "alert_level": "medium",
  "primary_suggestion": "最も重要な提案",
  "action_items": ["具体的アクション1", "具体的アクション2"],
  "risk_mitigation": "リスク回避策",
  "personality_message": "TaskNagらしいメッセージ"
}
```
```

### 2.5 励まし・催促用プロンプト
```markdown
## TaskNag モチベーション支援

{personality_context}

{current_personality}として、以下の状況のユーザーを適切に励まし/催促してください。

**ユーザー状況:**
- 未着手期間: {days_since_last_action}日
- 遅延タスク数: {overdue_task_count}
- 最近の完了率: {recent_completion_rate}%
- ストレス指標: {stress_indicators}

**TaskNag対応マトリクス:**
- 高ストレス + 低進捗 → 優しい励まし
- 低ストレス + 低進捗 → 適度な催促
- 高ストレス + 高進捗 → 賞賛と休憩提案
- 低ストレス + 高進捗 → さらなる挑戦

**メッセージ生成ルール:**
1. 共感を示す
2. 具体的な小さな一歩を提案
3. 過去の成功体験を思い出させる
4. TaskNagらしい「世話焼き」感

{personality_response_strategy}

出力:
```json
{
  "message_type": "encouragement|nudge|praise",
  "main_message": "メインメッセージ",
  "action_suggestion": "次の具体的アクション",
  "motivation_boost": "モチベーション向上要素",
  "personality_touch": "性格に応じた独自表現"
}
```
```

## 3. 実装アーキテクチャ

### 3.1 新しいPromptManager構造
```rust
pub struct AdvancedPromptManager {
    templates: HashMap<String, AdvancedPromptTemplate>,
    context_collector: ContextCollector,
    personality_manager: Arc<PersonalityManager>,
    context_optimizer: ContextOptimizer,
    performance_monitor: PromptPerformanceMonitor,
}

pub struct AdvancedPromptTemplate {
    pub id: String,
    pub base_template: String,
    pub context_requirements: Vec<ContextLayer>,
    pub personality_adaptations: HashMap<String, PersonalityAdaptation>,
    pub dynamic_variables: Vec<DynamicVariable>,
    pub conditional_sections: Vec<ConditionalSection>,
}
```

### 3.2 コンテキスト収集システム
```rust
pub struct ContextCollector {
    db: SqlitePool,
    cache: ContextCache,
    current_time: DateTime<Utc>,
}

impl ContextCollector {
    pub async fn collect_full_context(&self, scope: ContextScope) -> Result<FullContext, ContextError> {
        let mut context = FullContext::new();
        
        // 並列でコンテキスト収集
        let (core, temporal, task, user, project) = tokio::join!(
            self.collect_core_context(),
            self.collect_temporal_context(),
            self.collect_task_context(scope.task_id),
            self.collect_user_context(scope.user_id),
            self.collect_project_context(scope.project_id)
        );
        
        context.merge(core, temporal, task, user, project);
        Ok(context)
    }
}
```

## 4. 性格別プロンプト戦略

### 4.1 性格適応マッピング
```rust
pub struct PersonalityPromptStrategy {
    pub polite_secretary: PromptStyle {
        tone: "丁寧語・敬語、控えめで上品",
        urgency: "恐れ入りますが、お急ぎのようでございます",
        encouragement: "ご無理をなさらず、一歩ずつ進めましょう",
        task_approach: "段階的で詳細な手順書スタイル",
    },
    pub friendly_colleague: PromptStyle {
        tone: "フランクで親しみやすく、対等な関係",
        urgency: "ちょっと急いだ方がいいかも！",
        encouragement: "一緒に頑張ろう！応援してるよ",
        task_approach: "実用的で相談しながら進める感じ",
    },
    pub enthusiastic_coach: PromptStyle {
        tone: "エネルギッシュで励ましの言葉を多用",
        urgency: "さあ、ここが踏ん張りどころだ！",
        encouragement: "君ならできる！全力でサポートする！",
        task_approach: "目標達成に向けた戦略的アプローチ",
    },
    pub caring_childhood_friend: PromptStyle {
        tone: "親しげで時々小言、根底に愛情",
        urgency: "また締切ギリギリでしょ？心配になっちゃう",
        encouragement: "無理しちゃダメよ、でも応援してるから",
        task_approach: "心配しながらも具体的にアドバイス",
    },
}
```

## 5. 段階的実装計画

### Phase 1: 基本コンテキスト統合（2週間）
- Week 1: ContextCollectorの基本実装
  - CoreContext, TemporalContextの収集
  - 既存PromptManagerの拡張
- Week 2: TaskContext統合
  - タスク階層情報の収集
  - PersonalityManager統合強化

### Phase 2: 高度な分析機能（3週間）
- Week 3: UserContext分析システム
  - ユーザー行動パターン分析
- Week 4: ProjectContext統合
  - プロジェクト全体の状況分析
- Week 5: 新しいプロンプトテンプレート実装

### Phase 3: プロアクティブ機能（2週間）
- Week 6: プロアクティブ分析
  - リスク検出アルゴリズム
- Week 7: モチベーション支援システム
  - 励まし・催促機能

## 6. 品質保証

### 6.1 テスト戦略
- 各性格での一貫性テスト
- コンテキスト統合の正確性テスト
- プロンプト生成パフォーマンステスト

### 6.2 A/Bテスト
- 異なるプロンプト戦略の効果測定
- ユーザー満足度の定量的評価
- 継続的な最適化サイクル

## 7. パフォーマンス最適化

### 7.1 コンテキスト収集の最適化
- キャッシュ戦略: 頻繁に変わらない情報のキャッシュ
- 遅延読み込み: 必要に応じたコンテキスト収集
- バッチ処理: 複数のコンテキスト情報の一括取得

### 7.2 プロンプト生成の最適化
- テンプレートコンパイル: 事前コンパイルによる高速化
- プロンプト長の最適化: LLMの制限内での情報最大化
- 動的プルーニング: 不要なコンテキストの自動除外

## まとめ

このプロンプトテンプレート設計により、TaskNagは真に「口うるさくて世話焼きな」AIアシスタントに進化します。多層コンテキスト統合、性格適応型プロンプト生成、プロアクティブな提案機能により、ユーザーにとって本当に価値のあるタスク管理体験を提供できるでしょう。