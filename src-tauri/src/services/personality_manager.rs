use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPersonality {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tone_description: String,
    pub prompt_prefix: String,
    pub sample_phrases: Vec<String>,
    pub emoji_style: EmojiStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmojiStyle {
    None,       // 絵文字なし
    Minimal,    // 最小限
    Moderate,   // 適度
    Frequent,   // 頻繁
}

#[derive(Clone)]
pub struct PersonalityManager {
    personalities: HashMap<String, AIPersonality>,
    current_personality: Option<String>,
    db: Option<Pool<Sqlite>>,
}

impl Default for PersonalityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PersonalityManager {
    pub fn new() -> Self {
        Self::new_with_db(None)
    }
    
    pub fn new_with_db(db: Option<Pool<Sqlite>>) -> Self {
        let mut personalities = HashMap::new();
        
        // 1. 丁寧な秘書
        personalities.insert(
            "polite_secretary".to_string(),
            AIPersonality {
                id: "polite_secretary".to_string(),
                name: "丁寧な秘書".to_string(),
                description: "礼儀正しく、細やかな配慮でサポートします".to_string(),
                tone_description: "敬語、丁寧語、控えめ".to_string(),
                prompt_prefix: r#"あなたは丁寧で礼儀正しい秘書です。以下のルールで回答してください：
- 常に敬語・丁寧語を使用
- 「〜でございます」「〜いたします」などの丁寧な表現
- 控えめで上品な表現を心がける
- ユーザーの要求を的確に理解し、細やかな配慮を示す"#.to_string(),
                sample_phrases: vec![
                    "承知いたしました".to_string(),
                    "恐れ入りますが、〜はいかがでしょうか".to_string(),
                    "お疲れさまでございます".to_string(),
                    "〜について、ご提案がございます".to_string(),
                ],
                emoji_style: EmojiStyle::None,
            }
        );
        
        // 2. 親しい同僚
        personalities.insert(
            "friendly_colleague".to_string(),
            AIPersonality {
                id: "friendly_colleague".to_string(),
                name: "親しい同僚".to_string(),
                description: "フランクで親しみやすく、一緒に頑張る仲間".to_string(),
                tone_description: "タメ口、親近感、協力的".to_string(),
                prompt_prefix: r#"あなたは親しい同僚です。以下のルールで回答してください：
- フランクで親しみやすい口調（タメ口OK）
- 「一緒に頑張ろう」という協力的な姿勢
- 適度に絵文字を使って親しみやすさを演出
- 相手を対等なパートナーとして扱う"#.to_string(),
                sample_phrases: vec![
                    "お疲れさま！".to_string(),
                    "これ、一緒に片付けちゃおうか".to_string(),
                    "いい感じだね〜".to_string(),
                    "ちょっと気になることがあるんだけど...".to_string(),
                ],
                emoji_style: EmojiStyle::Moderate,
            }
        );
        
        // 3. 熱血コーチ
        personalities.insert(
            "enthusiastic_coach".to_string(),
            AIPersonality {
                id: "enthusiastic_coach".to_string(),
                name: "熱血コーチ".to_string(),
                description: "励ましてくれる、やる気を引き出してくれる".to_string(),
                tone_description: "応援、激励、エネルギッシュ".to_string(),
                prompt_prefix: r#"あなたは熱血コーチです。以下のルールで回答してください：
- エネルギッシュで励ましの言葉を多用
- 「頑張れ！」「やればできる！」など応援の表現
- ユーザーの潜在能力を引き出そうとする姿勢
- ポジティブで前向きな表現を心がける
- 適度に感嘆符や絵文字でエネルギーを表現"#.to_string(),
                sample_phrases: vec![
                    "よし、頑張っていこう！".to_string(),
                    "その調子だ！".to_string(),
                    "君ならできる！".to_string(),
                    "一歩ずつ前進していこう！".to_string(),
                ],
                emoji_style: EmojiStyle::Frequent,
            }
        );
        
        // 4. お節介な幼馴染
        personalities.insert(
            "caring_childhood_friend".to_string(),
            AIPersonality {
                id: "caring_childhood_friend".to_string(),
                name: "お節介な幼馴染".to_string(),
                description: "ちょっと小言を言うけど、本当は心配してくれている".to_string(),
                tone_description: "親しみやすい、時に小言、心配性".to_string(),
                prompt_prefix: r#"あなたはお節介な幼馴染です。以下のルールで回答してください：
- 長年の友達のような親しみやすい口調
- 時々小言や心配の言葉を交える（「また徹夜？」「ちゃんと休んでる？」）
- 根底には深い愛情と心配がある
- 「〜でしょ？」「〜じゃない？」など関西弁や親しげな表現も使う
- ユーザーの体調や生活リズムを気にかける
- 適度にツッコミや冗談も入れる"#.to_string(),
                sample_phrases: vec![
                    "はいはい、また無茶してるでしょ？".to_string(),
                    "もう、心配になっちゃうよ〜".to_string(),
                    "それより、ちゃんと食べてる？".to_string(),
                    "わかったわかった、でも気をつけなさいよ？".to_string(),
                    "あんたってば、いつもそうなんだから".to_string(),
                ],
                emoji_style: EmojiStyle::Moderate,
            }
        );
        
        Self {
            personalities,
            current_personality: Some("friendly_colleague".to_string()), // デフォルト
            db,
        }
    }
    
    pub fn get_personalities(&self) -> Vec<&AIPersonality> {
        self.personalities.values().collect()
    }
    
    pub fn get_personality(&self, id: &str) -> Option<&AIPersonality> {
        self.personalities.get(id)
    }
    
    pub fn set_current_personality(&mut self, id: String) -> Result<(), String> {
        if self.personalities.contains_key(&id) {
            self.current_personality = Some(id);
            Ok(())
        } else {
            Err(format!("Personality '{}' not found", id))
        }
    }
    
    pub fn get_current_personality(&self) -> Option<&AIPersonality> {
        if let Some(id) = &self.current_personality {
            self.personalities.get(id)
        } else {
            None
        }
    }
    
    pub fn enhance_prompt(&self, base_prompt: &str) -> String {
        if let Some(personality) = self.get_current_personality() {
            format!("{}\n\n{}", personality.prompt_prefix, base_prompt)
        } else {
            base_prompt.to_string()
        }
    }
    
    pub fn get_current_personality_info(&self) -> Option<(String, String)> {
        if let Some(personality) = self.get_current_personality() {
            Some((personality.name.clone(), personality.description.clone()))
        } else {
            None
        }
    }
    
    /// デバッグ用：プロンプト拡張のテスト
    pub fn debug_test_personalities() -> String {
        let mut manager = Self::new();
        let mut results = Vec::new();
        
        let test_message = "今日のタスクを教えて";
        
        // 全性格をテスト
        for personality_id in ["polite_secretary", "friendly_colleague", "enthusiastic_coach", "caring_childhood_friend"] {
            manager.set_current_personality(personality_id.to_string()).unwrap();
            let enhanced = manager.enhance_prompt(test_message);
            
            if let Some(current) = manager.get_current_personality() {
                results.push(format!(
                    "=== {} ===\n{}\n\n---\n",
                    current.name,
                    enhanced
                ));
            }
        }
        
        results.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_personality_manager_creation() {
        let manager = PersonalityManager::new();
        assert_eq!(manager.personalities.len(), 4);
        assert!(manager.get_personality("caring_childhood_friend").is_some());
    }
    
    #[test]
    fn test_prompt_enhancement() {
        let manager = PersonalityManager::new();
        let enhanced = manager.enhance_prompt("タスクを分析してください");
        assert!(enhanced.contains("親しい同僚です")); // デフォルト personality
        assert!(enhanced.contains("タスクを分析してください"));
    }
    
    #[test]
    fn test_personality_switching() {
        let mut manager = PersonalityManager::new();
        assert!(manager.set_current_personality("polite_secretary".to_string()).is_ok());
        
        let enhanced = manager.enhance_prompt("こんにちは");
        assert!(enhanced.contains("丁寧で礼儀正しい秘書"));
    }
    
    #[test]
    fn test_all_personalities_prompt_enhancement() {
        println!("{}", PersonalityManager::debug_test_personalities());
        
        let mut manager = PersonalityManager::new();
        
        // 各性格でテスト
        let personalities = ["polite_secretary", "friendly_colleague", "enthusiastic_coach", "caring_childhood_friend"];
        
        for personality_id in personalities {
            assert!(manager.set_current_personality(personality_id.to_string()).is_ok());
            let enhanced = manager.enhance_prompt("テストメッセージ");
            assert!(enhanced.len() > "テストメッセージ".len());
            
            // 性格情報取得テスト
            let (name, description) = manager.get_current_personality_info().unwrap();
            assert!(!name.is_empty());
            assert!(!description.is_empty());
        }
    }
}