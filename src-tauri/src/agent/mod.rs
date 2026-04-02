use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod llm;
pub mod memory;
pub mod skills;
pub mod agent_loop;

use llm::{LLMClient, LLMConfig};
use memory::{DualMemory, MemoryConfig, MemoryEntry};
use skills::{SkillRegistry, Skill};

/// Agent configuration matching local LLM setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// LLM endpoint (vLLM/Ollama compatible)
    pub llm_endpoint: String,
    /// Model name (Qwen2.5-32B)
    pub model_name: String,
    /// Embedding endpoint for semantic search
    pub embedding_endpoint: String,
    /// Embedding model (bge-large-zh-v1.5)
    pub embedding_model: String,
    /// Temperature for generation
    pub temperature: f32,
    /// Max tokens per request
    pub max_tokens: i32,
    /// API key for authentication (optional)
    pub api_key: Option<String>,
    /// System prompt template
    pub system_prompt: String,
    /// Learning enabled flag
    pub learning_enabled: bool,
    /// Auto-evolve skills
    pub auto_evolve: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            llm_endpoint: "http://localhost:8000/v1".to_string(),
            model_name: "Qwen2.5-32B".to_string(),
            embedding_endpoint: "http://localhost:8000/v1/embeddings".to_string(),
            embedding_model: "bge-large-zh-v1.5".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            api_key: None,
            system_prompt: include_str!("./system_prompt.md").to_string(),
            learning_enabled: true,
            auto_evolve: true,
        }
    }
}

/// Agent state for learning loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Unique session ID
    pub session_id: String,
    /// Current context/window
    pub context: Vec<Message>,
    /// Active tools available
    pub available_tools: Vec<String>,
    /// Learned patterns
    pub learned_patterns: Vec<Pattern>,
    /// Skill versions
    pub skill_versions: HashMap<String, i32>,
    /// Performance metrics
    pub metrics: AgentMetrics,
}

impl Default for AgentState {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            context: vec![],
            available_tools: vec![],
            learned_patterns: vec![],
            skill_versions: HashMap::new(),
            metrics: AgentMetrics::default(),
        }
    }
}

/// Agent metrics for tracking performance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentMetrics {
    pub total_interactions: i32,
    pub successful_tasks: i32,
    pub failed_tasks: i32,
    pub avg_response_time_ms: f64,
    pub user_satisfaction_score: f32,
}

/// Message structure for conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Learned pattern from interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub description: String,
    pub trigger_conditions: Vec<String>,
    pub action_template: String,
    pub success_rate: f32,
    pub usage_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    Command,
    Workflow,
    Preference,
    Correction,
}

/// Main Agent Manager - Central orchestrator
pub struct AgentManager {
    config: AgentConfig,
    llm_client: Arc<RwLock<LLMClient>>,
    memory: Arc<RwLock<DualMemory>>,
    skills: Arc<RwLock<SkillRegistry>>,
    state: Arc<RwLock<AgentState>>,
}

impl AgentManager {
    pub async fn new(config: AgentConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let llm_config = LLMConfig {
            endpoint: config.llm_endpoint.clone(),
            model: config.model_name.clone(),
            embedding_endpoint: config.embedding_endpoint.clone(),
            embedding_model: config.embedding_model.clone(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            api_key: config.api_key.clone(),
        };

        let llm_client = Arc::new(RwLock::new(LLMClient::new(llm_config).await?));

        let memory_config = MemoryConfig {
            embedding_endpoint: config.embedding_endpoint.clone(),
            embedding_model: config.embedding_model.clone(),
            max_entries: 10000,
            similarity_threshold: 0.75,
        };
        let memory = Arc::new(RwLock::new(DualMemory::new(memory_config).await?));

        let skills = Arc::new(RwLock::new(SkillRegistry::new()));
        let state = Arc::new(RwLock::new(AgentState::default()));

        Ok(Self {
            config,
            llm_client,
            memory,
            skills,
            state,
        })
    }

    /// Process user input and generate response with learning
    pub async fn process_message(
        &self,
        user_message: String,
    ) -> Result<AgentResponse, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();

        // 1. Retrieve relevant context from dual memory
        let relevant_memories = {
            let memory = self.memory.read().await;
            memory.retrieve_similar(&user_message, 5).await?
        };

        // 2. Get available skills
        let available_skills = {
            let skills = self.skills.read().await;
            skills.get_available_skills().await
        };

        // 3. Build enhanced context
        let context = self.build_context(&user_message, &relevant_memories, &available_skills).await;

        // 4. Generate response with LLM
        let llm_response = {
            let llm = self.llm_client.read().await;
            llm.chat_completion(context).await?
        };

        // 5. Parse and execute any tool calls
        let tool_results = self.execute_tool_calls(&llm_response.tool_calls).await;

        // 6. Store interaction in memory
        {
            let memory = self.memory.write().await;
            memory.store(MemoryEntry {
                id: uuid::Uuid::new_v4().to_string(),
                content: user_message.clone(),
                memory_type: memory::MemoryType::ShortTerm,
                timestamp: chrono::Utc::now(),
                embedding: vec![], // Will be computed by memory system
                metadata: Some(serde_json::json!({
                    "response": llm_response.content,
                    "tools_used": tool_results.len(),
                })),
            }).await?;
        }

        // 7. Learn from interaction if enabled
        if self.config.learning_enabled {
            self.learn_from_interaction(&user_message, &llm_response).await?;
        }

        // 8. Update metrics
        {
            let mut state = self.state.write().await;
            state.metrics.total_interactions += 1;
            state.metrics.avg_response_time_ms =
                (state.metrics.avg_response_time_ms * (state.metrics.total_interactions - 1) as f64
                + start_time.elapsed().as_millis() as f64)
                / state.metrics.total_interactions as f64;
        }

        Ok(AgentResponse {
            content: llm_response.content,
            tool_results,
            suggested_actions: vec![],
        })
    }

    /// Build context from frozen snapshot + live state
    async fn build_context(
        &self,
        user_message: &str,
        memories: &[MemoryEntry],
        skills: &[Skill],
    ) -> Vec<Message> {
        let mut context = vec![];

        // Frozen snapshot (system + learned patterns)
        context.push(Message {
            role: MessageRole::System,
            content: self.config.system_prompt.clone(),
            timestamp: chrono::Utc::now(),
            metadata: None,
        });

        // Add relevant memories as context
        for memory in memories {
            context.push(Message {
                role: MessageRole::System,
                content: format!("[Memory] {}", memory.content),
                timestamp: memory.timestamp,
                metadata: memory.metadata.as_ref().map(|m| {
                    if let Ok(map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(m.clone()) {
                        map
                    } else {
                        let mut map = HashMap::new();
                        map.insert("data".to_string(), m.clone());
                        map
                    }
                }),
            });
        }

        // Add available skills
        let skills_desc = skills.iter()
            .map(|s| format!("- {}: {}", s.name, s.description))
            .collect::<Vec<_>>()
            .join("\n");

        if !skills_desc.is_empty() {
            context.push(Message {
                role: MessageRole::System,
                content: format!("Available skills:\n{}", skills_desc),
                timestamp: chrono::Utc::now(),
                metadata: None,
            });
        }

        // Live state - user message
        context.push(Message {
            role: MessageRole::User,
            content: user_message.to_string(),
            timestamp: chrono::Utc::now(),
            metadata: None,
        });

        context
    }

    /// Execute tool calls from LLM response
    async fn execute_tool_calls(
        &self,
        tool_calls: &Option<Vec<llm::ToolCall>>,
    ) -> Vec<ToolResult> {
        let mut results = vec![];

        if let Some(calls) = tool_calls {
            for call in calls {
                let result = self.execute_single_tool(call).await;
                results.push(ToolResult {
                    tool_name: call.function.name.clone(),
                    success: result.is_ok(),
                    result: result.unwrap_or_else(|e| format!("Error: {}", e)),
                });
            }
        }

        results
    }

    /// Execute a single tool
    async fn execute_single_tool(
        &self,
        tool_call: &llm::ToolCall,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // For now, return mock results since skills needs mutable access
        // In a full implementation, we'd use a channel or other mechanism
        Ok(format!("Would execute skill: {} with args: {}",
            tool_call.function.name,
            tool_call.function.arguments))
    }

    /// Learn from interaction - Hermes Agent style closed loop
    async fn learn_from_interaction(
        &self,
        user_message: &str,
        llm_response: &llm::LLMResponse,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check for patterns that can be learned
        if user_message.contains("记住") || user_message.contains("下次") {
            // Extract pattern for future use
            let pattern = Pattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Preference,
                description: format!("User preference: {}", user_message),
                trigger_conditions: vec![user_message.to_string()],
                action_template: llm_response.content.clone(),
                success_rate: 1.0,
                usage_count: 1,
                created_at: chrono::Utc::now(),
            };

            let memory = self.memory.write().await;
            memory.store(MemoryEntry {
                id: pattern.id.clone(),
                content: serde_json::to_string(&pattern)?,
                memory_type: memory::MemoryType::LongTerm,
                timestamp: chrono::Utc::now(),
                embedding: vec![],
                metadata: Some(serde_json::json!({"type": "pattern"})),
            }).await?;
        }

        // Auto-evolve skills if enabled (disabled for now - requires different architecture)
        // if self.config.auto_evolve {
        //     self.evolve_skills().await?;
        // }

        Ok(())
    }

    // /// Evolve skills based on usage patterns
    // async fn evolve_skills(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     let mut skills = self.skills.write().await;
    //     skills.evolve_skills().await
    // }

    /// Get current agent metrics
    pub async fn get_metrics(&self) -> AgentMetrics {
        let state = self.state.read().await;
        state.metrics.clone()
    }

    /// Export learned knowledge
    pub async fn export_knowledge(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let memory = self.memory.read().await;
        let entries = memory.export_all().await?;
        Ok(serde_json::to_string_pretty(&entries)?)
    }

    /// Import learned knowledge
    pub async fn import_knowledge(&self, knowledge_json: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let entries: Vec<MemoryEntry> = serde_json::from_str(knowledge_json)?;
        let mut memory = self.memory.write().await;
        for entry in entries {
            memory.store(entry).await?;
        }
        Ok(())
    }
}

/// Agent response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub content: String,
    pub tool_results: Vec<ToolResult>,
    pub suggested_actions: Vec<String>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub result: String,
}
