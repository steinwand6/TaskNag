use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("JSON parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Ollama server not available at {0}")]
    ServerNotAvailable(String),
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Timeout after {0} seconds")]
    Timeout(u64),
}

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub base_url: String,
    client: Client,
    default_model: String,
    pub timeout_seconds: u64,
}

#[derive(Serialize, Debug)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GenerateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct GenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    pub response: String,
    pub done: bool,
    #[serde(default)]
    pub thinking: Option<String>,
    #[serde(default)]
    pub context: Option<Vec<i32>>,
    #[serde(default)]
    pub total_duration: Option<u64>,
    #[serde(default)]
    pub load_duration: Option<u64>,
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,
    #[serde(default)]
    pub eval_count: Option<i32>,
    #[serde(default)]
    pub eval_duration: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new(
            "http://localhost:11434".to_string(),
            "llama3:latest".to_string(),
            30,
        )
    }
}

impl OllamaClient {
    pub fn new(base_url: String, default_model: String, timeout_seconds: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self {
            base_url,
            client,
            default_model,
            timeout_seconds,
        }
    }
    
    /// Get current default model
    pub fn get_model(&self) -> &String {
        &self.default_model
    }
    
    /// Test connection to Ollama server
    pub async fn test_connection(&self) -> Result<bool, OllamaError> {
        let url = format!("{}/api/tags", self.base_url);
        log::info!("Ollama接続テスト URL: {}", url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                log::info!("Ollama応答ステータス: {}", status);
                
                if status.is_success() {
                    log::info!("Ollama接続成功");
                    Ok(true)
                } else {
                    log::error!("Ollama接続失敗 - ステータス: {}", status);
                    Err(OllamaError::ServerNotAvailable(self.base_url.clone()))
                }
            },
            Err(e) => {
                log::error!("Ollama接続エラー詳細: {:?}", e);
                
                if e.is_timeout() {
                    Err(OllamaError::Timeout(self.timeout_seconds))
                } else if e.is_connect() {
                    Err(OllamaError::ServerNotAvailable(self.base_url.clone()))
                } else {
                    Err(OllamaError::RequestError(e))
                }
            }
        }
    }
    
    /// List available models
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(OllamaError::ServerNotAvailable(self.base_url.clone()));
        }
        
        let models_response: ListModelsResponse = response.json().await?;
        Ok(models_response.models)
    }
    
    /// Generate text completion
    pub async fn generate(
        &self,
        prompt: &str,
        options: Option<GenerateOptions>,
    ) -> Result<GenerateResponse, OllamaError> {
        self.generate_with_model(&self.default_model, prompt, options).await
    }
    
    /// Generate text completion with specific model
    pub async fn generate_with_model(
        &self,
        model: &str,
        prompt: &str,
        options: Option<GenerateOptions>,
    ) -> Result<GenerateResponse, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options,
            format: None,
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            if response.status().as_u16() == 404 {
                return Err(OllamaError::ModelNotFound(model.to_string()));
            }
            return Err(OllamaError::ServerNotAvailable(self.base_url.clone()));
        }
        
        let generate_response: GenerateResponse = response.json().await?;
        Ok(generate_response)
    }
    
    /// Get actual response content (either response or thinking field)
    pub fn get_response_content(response: &GenerateResponse) -> String {
        if !response.response.is_empty() {
            response.response.clone()
        } else if let Some(thinking) = &response.thinking {
            thinking.clone()
        } else {
            "No response generated".to_string()
        }
    }
    
    /// Generate text with Japanese support and smart response extraction
    pub async fn generate_japanese(
        &self,
        prompt: &str,
        options: Option<GenerateOptions>,
    ) -> Result<String, OllamaError> {
        let response = self.generate(prompt, options).await?;
        Ok(Self::get_response_content(&response))
    }
    
    /// Generate JSON response
    pub async fn generate_json(
        &self,
        prompt: &str,
        options: Option<GenerateOptions>,
    ) -> Result<serde_json::Value, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);
        log::info!("JSON生成リクエスト URL: {}, モデル: {}", url, self.default_model);
        
        // gemma3:12bモデルはformat: "json"に対応
        let request = GenerateRequest {
            model: self.default_model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options,
            format: Some("json".to_string()),
        };
        
        log::info!("リクエスト送信中...");
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        let status = response.status();
        log::info!("レスポンスステータス: {}", status);
        
        if !status.is_success() {
            log::error!("HTTP エラー - ステータス: {}", status);
            return Err(OllamaError::ServerNotAvailable(self.base_url.clone()));
        }
        
        log::info!("JSON パース中...");
        let generate_response: GenerateResponse = response.json().await?;
        
        log::info!("生成されたレスポンス長: {}", generate_response.response.len());
        log::info!("レスポンス内容（最初の200文字）: {}", 
                  &generate_response.response.chars().take(200).collect::<String>());
        
        let json_value: serde_json::Value = serde_json::from_str(&generate_response.response)?;
        log::info!("JSON パース成功");
        Ok(json_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_client_creation() {
        let client = OllamaClient::default();
        assert_eq!(client.base_url, "http://localhost:11434");
        assert_eq!(client.default_model, "llama3:latest");
        assert_eq!(client.timeout_seconds, 30);
    }
    
    #[tokio::test]
    async fn test_custom_client() {
        let client = OllamaClient::new(
            "http://custom:8080".to_string(),
            "mistral:latest".to_string(),
            60,
        );
        assert_eq!(client.base_url, "http://custom:8080");
        assert_eq!(client.default_model, "mistral:latest");
        assert_eq!(client.timeout_seconds, 60);
    }
}