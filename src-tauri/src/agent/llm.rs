use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// LLM Configuration
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub endpoint: String,
    pub model: String,
    pub embedding_endpoint: String,
    pub embedding_model: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub api_key: Option<String>,
}

/// LLM Client for OpenAI-compatible APIs (vLLM, Ollama, etc.)
pub struct LLMClient {
    config: LLMConfig,
    http_client: Client,
}

/// Message for chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Tool definition for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub r#type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Tool call from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

/// Chat completion request
#[derive(Debug, Clone, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

/// Chat completion response
#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize)]
struct Choice {
    message: ResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseMessage {
    role: String,
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

/// LLM Response
#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tokens_used: i32,
}

/// Embedding request
#[derive(Debug, Clone, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: String,
}

/// Embedding response
#[derive(Debug, Clone, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Clone, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: i32,
}

impl LLMClient {
    pub async fn new(config: LLMConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Generate chat completion
    pub async fn chat_completion(
        &self,
        messages: Vec<super::Message>,
    ) -> Result<LLMResponse, Box<dyn std::error::Error + Send + Sync>> {
        let chat_messages: Vec<ChatMessage> = messages
            .into_iter()
            .map(|m| ChatMessage {
                role: match m.role {
                    super::MessageRole::System => "system".to_string(),
                    super::MessageRole::User => "user".to_string(),
                    super::MessageRole::Assistant => "assistant".to_string(),
                    super::MessageRole::Tool => "tool".to_string(),
                },
                content: m.content,
            })
            .collect();

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: chat_messages,
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            tools: None, // Can be extended with movie-specific tools
        };

        let url = format!("{}/chat/completions", self.config.endpoint);

        let mut req_builder = self
            .http_client
            .post(&url)
            .json(&request);

        // Add API key if configured
        if let Some(ref api_key) = self.config.api_key {
            if !api_key.is_empty() {
                req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
            }
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("LLM API error: {}", error_text).into());
        }

        let completion: ChatCompletionResponse = response.json().await?;

        if let Some(choice) = completion.choices.first() {
            Ok(LLMResponse {
                content: choice.message.content.clone().unwrap_or_default(),
                tool_calls: choice.message.tool_calls.clone(),
                tokens_used: completion.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
            })
        } else {
            Err("No completion choices returned".into())
        }
    }

    /// Generate embedding for text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        let request = EmbeddingRequest {
            model: self.config.embedding_model.clone(),
            input: text.to_string(),
        };

        let mut req_builder = self
            .http_client
            .post(&self.config.embedding_endpoint)
            .json(&request);

        // Add API key if configured
        if let Some(ref api_key) = self.config.api_key {
            if !api_key.is_empty() {
                req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
            }
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Embedding API error: {}", error_text).into());
        }

        let embedding_response: EmbeddingResponse = response.json().await?;

        embedding_response
            .data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| "No embedding returned".into())
    }

    /// Stream chat completion (for real-time responses)
    pub async fn stream_chat_completion(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(Debug, Clone, Serialize)]
        struct StreamRequest {
            model: String,
            messages: Vec<ChatMessage>,
            temperature: f32,
            max_tokens: i32,
            stream: bool,
        }

        let request = StreamRequest {
            model: self.config.model.clone(),
            messages,
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            stream: true,
        };

        let url = format!("{}/chat/completions", self.config.endpoint);

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("LLM streaming error: {}", error_text).into());
        }

        Ok(response)
    }

    /// Test connection to LLM endpoint
    pub async fn test_connection(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }];

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 10,
            tools: None,
        };

        let url = format!("{}/chat/completions", self.config.endpoint);

        let response = self.http_client.post(&url).json(&request).send().await?;

        Ok(response.status().is_success())
    }
}
