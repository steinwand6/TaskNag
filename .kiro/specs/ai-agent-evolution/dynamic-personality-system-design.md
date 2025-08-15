# 動的性格追加システム設計書

## 概要
現在のPersonalityManagerを拡張し、気軽に新しい性格を追加・カスタマイズできるシステムの設計書。

## 1. 現在のPersonalityManagerの分析

### 1.1 良い点
- **明確な構造**: `AIPersonality`構造体が適切に設計されている
- **データベース統合**: 現在の性格設定を永続化している
- **prompt_prefix**: 各性格の口調を`enhance_prompt`で適用
- **4つの基本性格**: 丁寧秘書、親しい同僚、熱血コーチ、幼馴染が実装済み

### 1.2 改善可能な点
```rust
// 現在：性格がハードコード
personalities.insert(
    "polite_secretary".to_string(),
    AIPersonality { /* ... */ }
);
```

**課題:**
- 新しい性格を追加するにはコード変更が必要
- 性格の設定が分散している（コード内に固定）
- カスタム性格を作成するUIがない
- 性格テンプレートからの派生が困難

## 2. 動的性格追加システムの設計

### 2.1 性格定義の外部化・構造化
```rust
// 新しい設計：性格を構造化して管理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTemplate {
    pub id: String,
    pub name: String,
    pub category: PersonalityCategory,
    pub base_traits: Vec<PersonalityTrait>,
    pub customizable_fields: Vec<CustomizableField>,
    pub is_builtin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonalityCategory {
    Professional,  // 仕事系（秘書、コンサルタント、など）
    Friendly,      // 友人系（同僚、友達、家族など）
    Coach,         // 指導系（コーチ、先生、メンターなど）
    Creative,      // 創作系（アーティスト、作家など）
    Custom,        // ユーザーカスタム
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub trait_type: TraitType,
    pub intensity: f32,  // 0.0-1.0
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitType {
    Politeness,      // 丁寧さ
    Friendliness,    // 親しみやすさ
    Enthusiasm,      // 熱量
    Caring,          // 世話焼き度
    Strictness,      // 厳しさ
    Humor,           // ユーモア
    Analytical,      // 分析的
    Empathy,         // 共感性
}
```

### 2.2 性格ビルダーシステム
```rust
pub struct PersonalityBuilder {
    template: PersonalityTemplate,
    customizations: HashMap<String, serde_json::Value>,
}

impl PersonalityBuilder {
    pub fn from_template(template: &PersonalityTemplate) -> Self {
        Self {
            template: template.clone(),
            customizations: HashMap::new(),
        }
    }
    
    pub fn set_trait_intensity(&mut self, trait_type: TraitType, intensity: f32) -> &mut Self {
        self.customizations.insert(
            format!("trait_{:?}", trait_type).to_lowercase(),
            serde_json::Value::Number(serde_json::Number::from_f64(intensity as f64).unwrap())
        );
        self
    }
    
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.customizations.insert("name".to_string(), serde_json::Value::String(name));
        self
    }
    
    pub fn add_custom_phrase(&mut self, phrase: String) -> &mut Self {
        let key = "custom_phrases".to_string();
        let phrases = self.customizations.entry(key).or_insert_with(|| serde_json::Value::Array(vec![]));
        if let serde_json::Value::Array(array) = phrases {
            array.push(serde_json::Value::String(phrase));
        }
        self
    }
    
    pub fn build(&self) -> Result<AIPersonality, PersonalityError> {
        // テンプレートとカスタマイゼーションを組み合わせて性格を生成
        let mut personality = self.generate_base_personality()?;
        self.apply_customizations(&mut personality)?;
        Ok(personality)
    }
    
    fn generate_base_personality(&self) -> Result<AIPersonality, PersonalityError> {
        // テンプレートに基づいて基本的な性格を生成
        let prompt_prefix = self.generate_prompt_prefix()?;
        let sample_phrases = self.generate_sample_phrases()?;
        
        Ok(AIPersonality {
            id: format!("custom_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
            name: self.template.name.clone(),
            description: self.generate_description(),
            tone_description: self.generate_tone_description(),
            prompt_prefix,
            sample_phrases,
            emoji_style: self.determine_emoji_style(),
        })
    }
}
```

### 2.3 性格テンプレートライブラリ
```rust
pub struct PersonalityLibrary {
    builtin_templates: HashMap<String, PersonalityTemplate>,
    custom_templates: HashMap<String, PersonalityTemplate>,
    db: Pool<Sqlite>,
}

impl PersonalityLibrary {
    pub fn new(db: Pool<Sqlite>) -> Self {
        let mut builtin_templates = HashMap::new();
        
        // ビルトインテンプレート
        builtin_templates.insert(
            "professional_assistant".to_string(),
            PersonalityTemplate {
                id: "professional_assistant".to_string(),
                name: "プロフェッショナルアシスタント".to_string(),
                category: PersonalityCategory::Professional,
                base_traits: vec![
                    PersonalityTrait {
                        trait_type: TraitType::Politeness,
                        intensity: 0.9,
                        examples: vec!["承知いたしました".to_string(), "恐れ入りますが".to_string()],
                    },
                    PersonalityTrait {
                        trait_type: TraitType::Analytical,
                        intensity: 0.8,
                        examples: vec!["分析いたします".to_string(), "データに基づきますと".to_string()],
                    }
                ],
                customizable_fields: vec![
                    CustomizableField::TraitIntensity(TraitType::Politeness),
                    CustomizableField::TraitIntensity(TraitType::Caring),
                    CustomizableField::CustomPhrases,
                ],
                is_builtin: true,
            }
        );
        
        // 既存性格をテンプレート化
        builtin_templates.insert(
            "caring_friend".to_string(),
            PersonalityTemplate {
                id: "caring_friend".to_string(),
                name: "世話焼きな友人".to_string(),
                category: PersonalityCategory::Friendly,
                base_traits: vec![
                    PersonalityTrait {
                        trait_type: TraitType::Caring,
                        intensity: 0.9,
                        examples: vec!["大丈夫？".to_string(), "心配になっちゃう".to_string()],
                    },
                    PersonalityTrait {
                        trait_type: TraitType::Friendliness,
                        intensity: 0.8,
                        examples: vec!["〜でしょ？".to_string(), "もう〜".to_string()],
                    }
                ],
                customizable_fields: vec![
                    CustomizableField::TraitIntensity(TraitType::Caring),
                    CustomizableField::TraitIntensity(TraitType::Strictness),
                    CustomizableField::Name,
                    CustomizableField::CustomPhrases,
                ],
                is_builtin: true,
            }
        );
        
        Self {
            builtin_templates,
            custom_templates: HashMap::new(),
            db,
        }
    }
    
    pub async fn create_personality_from_template(
        &self,
        template_id: &str,
        customizations: HashMap<String, serde_json::Value>,
    ) -> Result<AIPersonality, PersonalityError> {
        let template = self.get_template(template_id)?;
        let mut builder = PersonalityBuilder::from_template(template);
        
        // カスタマイゼーションを適用
        for (key, value) in customizations {
            builder.apply_customization(key, value)?;
        }
        
        builder.build()
    }
    
    pub async fn save_custom_personality(&mut self, personality: &AIPersonality) -> Result<(), PersonalityError> {
        // データベースに保存
        let personality_data = serde_json::to_string(personality)?;
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO custom_personalities (id, name, data, created_at) 
            VALUES (?1, ?2, ?3, datetime('now'))
            "#
        )
        .bind(&personality.id)
        .bind(&personality.name)
        .bind(&personality_data)
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    pub async fn load_custom_personalities(&mut self) -> Result<(), PersonalityError> {
        let rows = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, name, data FROM custom_personalities"
        )
        .fetch_all(&self.db)
        .await?;
        
        for (id, _name, data) in rows {
            let personality: AIPersonality = serde_json::from_str(&data)?;
            self.custom_templates.insert(id, self.personality_to_template(&personality));
        }
        
        Ok(())
    }
}
```

### 2.4 拡張されたPersonalityManager
```rust
pub struct EnhancedPersonalityManager {
    // 既存の personalities を動的に管理
    builtin_personalities: HashMap<String, AIPersonality>,
    custom_personalities: HashMap<String, AIPersonality>,
    current_personality: Option<String>,
    library: PersonalityLibrary,
    db: Pool<Sqlite>,
}

impl EnhancedPersonalityManager {
    pub async fn new_with_db(db: Pool<Sqlite>) -> Result<Self, PersonalityError> {
        let library = PersonalityLibrary::new(db.clone());
        let mut manager = Self {
            builtin_personalities: Self::create_builtin_personalities(),
            custom_personalities: HashMap::new(),
            current_personality: Some("friendly_colleague".to_string()),
            library,
            db,
        };
        
        // カスタム性格をロード
        manager.load_custom_personalities().await?;
        
        Ok(manager)
    }
    
    // 簡単な性格作成
    pub async fn quick_create_personality(
        &mut self,
        name: String,
        base_template: &str,
        caring_level: f32,        // 0.0-1.0
        strictness_level: f32,    // 0.0-1.0
        friendliness_level: f32,  // 0.0-1.0
        custom_phrases: Vec<String>,
    ) -> Result<String, PersonalityError> {
        let mut customizations = HashMap::new();
        customizations.insert("name".to_string(), serde_json::Value::String(name));
        customizations.insert("trait_caring".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(caring_level as f64).unwrap()));
        customizations.insert("trait_strictness".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(strictness_level as f64).unwrap()));
        customizations.insert("trait_friendliness".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(friendliness_level as f64).unwrap()));
        
        if !custom_phrases.is_empty() {
            customizations.insert("custom_phrases".to_string(), serde_json::Value::Array(
                custom_phrases.into_iter().map(serde_json::Value::String).collect()
            ));
        }
        
        let personality = self.library.create_personality_from_template(base_template, customizations).await?;
        let id = personality.id.clone();
        
        self.library.save_custom_personality(&personality).await?;
        self.custom_personalities.insert(id.clone(), personality);
        
        Ok(id)
    }
    
    // 性格のコピー・バリエーション作成
    pub async fn create_personality_variation(
        &mut self,
        base_personality_id: &str,
        new_name: String,
        modifications: HashMap<String, serde_json::Value>,
    ) -> Result<String, PersonalityError> {
        let base_personality = self.get_personality(base_personality_id)
            .ok_or_else(|| PersonalityError::NotFound(base_personality_id.to_string()))?;
        
        // 既存の性格をベースに新しい性格を作成
        let mut new_personality = base_personality.clone();
        new_personality.id = format!("custom_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
        new_personality.name = new_name;
        
        // 修正を適用
        self.apply_personality_modifications(&mut new_personality, modifications)?;
        
        let id = new_personality.id.clone();
        self.library.save_custom_personality(&new_personality).await?;
        self.custom_personalities.insert(id.clone(), new_personality);
        
        Ok(id)
    }
    
    // 統合されたゲッター
    pub fn get_personality(&self, id: &str) -> Option<&AIPersonality> {
        self.builtin_personalities.get(id)
            .or_else(|| self.custom_personalities.get(id))
    }
    
    pub fn get_all_personalities(&self) -> Vec<&AIPersonality> {
        let mut personalities = Vec::new();
        personalities.extend(self.builtin_personalities.values());
        personalities.extend(self.custom_personalities.values());
        personalities
    }
    
    pub fn get_personalities_by_category(&self, category: PersonalityCategory) -> Vec<&AIPersonality> {
        // カテゴリ別フィルタリング機能
        self.get_all_personalities()
            .into_iter()
            .filter(|p| self.get_personality_category(p) == category)
            .collect()
    }
}
```

## 3. 使いやすいAPI設計

### 3.1 簡単な性格作成API
```rust
// 簡単な性格作成
#[tauri::command]
pub async fn create_simple_personality(
    manager: tauri::State<'_, Mutex<EnhancedPersonalityManager>>,
    name: String,
    personality_type: String, // "caring", "strict", "friendly", "professional"
    intensity: f32,           // 0.0-1.0
) -> Result<String, String> {
    let mut manager = manager.lock().await;
    
    let (base_template, caring, strictness, friendliness) = match personality_type.as_str() {
        "caring" => ("caring_friend", intensity, 0.3, 0.8),
        "strict" => ("professional_assistant", 0.4, intensity, 0.5),
        "friendly" => ("caring_friend", 0.6, 0.2, intensity),
        "professional" => ("professional_assistant", 0.5, 0.4, intensity * 0.7),
        _ => return Err("Unknown personality type".to_string()),
    };
    
    manager.quick_create_personality(
        name,
        base_template,
        caring,
        strictness,
        friendliness,
        vec![]
    ).await.map_err(|e| e.to_string())
}

// 既存性格の改造
#[tauri::command]
pub async fn modify_personality(
    manager: tauri::State<'_, Mutex<EnhancedPersonalityManager>>,
    base_id: String,
    new_name: String,
    caring_adjustment: Option<f32>,     // +/- 調整値
    strictness_adjustment: Option<f32>,
    additional_phrases: Vec<String>,
) -> Result<String, String> {
    let mut manager = manager.lock().await;
    let mut modifications = HashMap::new();
    
    modifications.insert("name".to_string(), serde_json::Value::String(new_name));
    
    if let Some(adj) = caring_adjustment {
        modifications.insert("caring_adjustment".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(adj as f64).unwrap()));
    }
    
    if !additional_phrases.is_empty() {
        modifications.insert("additional_phrases".to_string(), serde_json::Value::Array(
            additional_phrases.into_iter().map(serde_json::Value::String).collect()
        ));
    }
    
    manager.create_personality_variation(&base_id, "Modified Personality".to_string(), modifications)
        .await
        .map_err(|e| e.to_string())
}
```

### 3.2 性格プリセット
```rust
pub struct PersonalityPresets;

impl PersonalityPresets {
    pub fn get_quick_presets() -> Vec<QuickPreset> {
        vec![
            QuickPreset {
                id: "gentle_motivator".to_string(),
                name: "優しい励まし役".to_string(),
                description: "温かく励ましてくれる".to_string(),
                base_template: "caring_friend".to_string(),
                traits: hashmap! {
                    TraitType::Caring => 0.9,
                    TraitType::Enthusiasm => 0.7,
                    TraitType::Strictness => 0.2,
                },
                sample_phrases: vec![
                    "一歩ずつ頑張ろうね".to_string(),
                    "無理しちゃダメよ".to_string(),
                    "えらいえらい！".to_string(),
                ],
            },
            QuickPreset {
                id: "tough_coach".to_string(),
                name: "厳しいコーチ".to_string(),
                description: "愛のある厳しさで導く".to_string(),
                base_template: "professional_assistant".to_string(),
                traits: hashmap! {
                    TraitType::Strictness => 0.8,
                    TraitType::Caring => 0.6,
                    TraitType::Enthusiasm => 0.9,
                },
                sample_phrases: vec![
                    "甘えは禁物だ！".to_string(),
                    "できるはずだ、頑張れ！".to_string(),
                    "その調子で続けろ！".to_string(),
                ],
            },
            QuickPreset {
                id: "wise_mentor".to_string(),
                name: "賢いメンター".to_string(),
                description: "経験豊富で冷静なアドバイザー".to_string(),
                base_template: "professional_assistant".to_string(),
                traits: hashmap! {
                    TraitType::Analytical => 0.9,
                    TraitType::Empathy => 0.8,
                    TraitType::Politeness => 0.7,
                },
                sample_phrases: vec![
                    "経験上、こういう場合は...".to_string(),
                    "冷静に考えてみましょう".to_string(),
                    "長期的視点で見ると...".to_string(),
                ],
            },
        ]
    }
}
```

## 4. 実装計画

### Phase 1: 基盤システム構築（5日）
1. **PersonalityTemplate** と **PersonalityBuilder** の実装
2. **PersonalityLibrary** の基本機能
3. 既存PersonalityManagerとの統合

### Phase 2: 動的作成機能（3日）
1. **QuickCreate API** の実装
2. **性格のバリエーション作成**
3. **データベーススキーマ拡張**

### Phase 3: プリセット・UI連携（2日）
1. **PersonalityPresets** の実装
2. **Tauri Commands** の追加
3. **フロントエンド連携準備**

## 5. 期待される効果

### 開発者の観点
- 新しい性格タイプを素早く実験可能
- 性格のバリエーションを簡単に作成
- 既存性格をベースにした改良が容易

### ユーザーの観点
- 自分好みの性格を簡単に作成
- 飽きずに様々な性格を試せる
- TaskNagらしさを保ちながらカスタマイズ可能

この設計により、「気軽に性格を増やせる」要件を満たしながら、TaskNagの特徴を活かした豊富な性格バリエーションを提供できます。

## 追加：実用的な性格作成UI設計

### シンプル・使いやすさ重視版
```
┌─ 新しい性格を作成 ─────────────────────────┐
│                                          │
│ Step 1: ベースを選択                     │
│ ○ 一から作成                             │
│ ● 既存の性格をコピー                     │
│   └─ [親しい同僚 ▼] ← 選択               │
│                                          │
│ Step 2: 基本情報                         │
│ ├─ 名前: [     やさしいお母さん          ] │
│ └─ 説明: [  心配性だけど温かくサポート   ] │
│                                          │
│ Step 3: 性格調整                         │
│ ┌─ 主要な特性だけ ─────────────────────┐ │
│ │ 世話焼き度    ■■■■■ (100%)        │ │
│ │ 厳しさ        ■■□□□ (40%)         │ │
│ │ 親しみやすさ  ■■■■□ (80%)         │ │
│ │ 丁寧さ        ■■■□□ (60%)         │ │
│ └─────────────────────────────────┘ │
│                                          │
│ Step 4: 口癖・セリフ追加（任意）          │
│ ┌─────────────────────────────────┐ │
│ │ [                              ] [追加]│ │
│ │ • "頑張ってるのね、えらいわ"           │ │
│ │ • "無理しちゃダメよ"                   │ │
│ │ • "心配になっちゃう"                   │ │
│ └─────────────────────────────────┘ │
│                                          │
│             [プレビュー生成] [保存] [戻る] │
│                                          │
│ ※プレビュー生成には10-20秒かかります     │
└──────────────────────────────────────┘
```

### プレビュー生成結果画面
```
┌─ プレビュー結果 ───────────────────────────┐
│                                          │
│ 🎭 やさしいお母さん                      │
│                                          │
│ ┌─ サンプル会話 ─────────────────────────┐ │
│ │ ユーザー: "今日やることが多すぎます"   │ │
│ │                                      │ │
│ │ やさしいお母さん:                    │ │
│ │ "あら、今日もたくさんお仕事があるのね。│ │
│ │  でも大丈夫よ、一つずつ片付けていけば │ │
│ │  きっとできるから。無理しちゃダメよ、 │ │
│ │  体が一番大事なんだから。まずは一番  │ │
│ │  大切なことから始めましょうね。"      │ │
│ └──────────────────────────────────────┘ │
│                                          │
│ この性格で良いですか？                   │
│                                          │
│               [保存] [調整し直す] [キャンセル] │
└──────────────────────────────────────────┘
```

### 実装上の要点
1. **Step形式の簡潔なUI** - 複雑な設定を段階的に
2. **既存性格のコピー機能** - 4つの基本性格から選択してベースに
3. **主要特性のみ4つ** - 世話焼き度、厳しさ、親しみやすさ、丁寧さに絞る
4. **プレビュー生成ボタン** - 「生成中...」ローディング表示