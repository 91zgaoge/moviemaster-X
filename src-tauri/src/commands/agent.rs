use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use crate::agent::{AgentConfig, AgentManager, AgentMetrics};

/// Shared agent state
pub struct AgentState {
    pub agent: Arc<TokioMutex<Option<AgentManager>>>,
}

impl AgentState {
    pub fn new() -> Self {
        Self {
            agent: Arc::new(TokioMutex::new(None)),
        }
    }
}

impl Default for AgentState {
    fn default() -> Self {
        Self::new()
    }
}

/// Send message to AI agent
#[tauri::command]
pub async fn agent_send_message(
    message: String,
    state: State<'_, AgentState>,
) -> Result<AgentResponse, String> {
    let mut agent_guard = state.agent.lock().await;

    // Initialize agent if not exists
    if agent_guard.is_none() {
        let config = AgentConfig::default();
        match AgentManager::new(config).await {
            Ok(agent) => {
                *agent_guard = Some(agent);
            }
            Err(e) => return Err(format!("Failed to initialize agent: {}", e)),
        }
    }

    // Process message
    if let Some(agent) = agent_guard.as_ref() {
        match agent.process_message(message).await {
            Ok(response) => Ok(AgentResponse {
                content: response.content,
                tool_results: response.tool_results.iter().map(|t| ToolResult {
                    tool_name: t.tool_name.clone(),
                    success: t.success,
                    result: t.result.clone(),
                }).collect(),
                suggested_actions: response.suggested_actions,
            }),
            Err(e) => Err(format!("Agent error: {}", e)),
        }
    } else {
        Err("Agent not initialized".to_string())
    }
}

/// Get agent metrics
#[tauri::command]
pub async fn agent_get_metrics(
    state: State<'_, AgentState>,
) -> Result<AgentMetrics, String> {
    let agent_guard = state.agent.lock().await;

    if let Some(agent) = agent_guard.as_ref() {
        Ok(agent.get_metrics().await)
    } else {
        Ok(AgentMetrics::default())
    }
}

/// Get available skills
#[tauri::command]
pub async fn agent_get_available_skills(
    state: State<'_, AgentState>,
) -> Result<Vec<SkillInfo>, String> {
    let agent_guard = state.agent.lock().await;

    if let Some(agent) = agent_guard.as_ref() {
        // Get skills from agent's skill registry
        // For now return empty list, would need to expose skill registry
        Ok(vec![
            SkillInfo {
                id: "movie_search".to_string(),
                name: "Movie Search".to_string(),
                description: "Search local movie database".to_string(),
                version: 1,
            },
            SkillInfo {
                id: "pt_search".to_string(),
                name: "PT Site Search".to_string(),
                description: "Search torrents on PT sites".to_string(),
                version: 1,
            },
            SkillInfo {
                id: "qb_control".to_string(),
                name: "qBittorrent Control".to_string(),
                description: "Control qBittorrent downloads".to_string(),
                version: 1,
            },
            SkillInfo {
                id: "dup_detect".to_string(),
                name: "Duplicate Detection".to_string(),
                description: "Find and manage duplicate movies".to_string(),
                version: 1,
            },
            SkillInfo {
                id: "smart_update".to_string(),
                name: "Smart Update".to_string(),
                description: "Intelligently update movie metadata".to_string(),
                version: 1,
            },
        ])
    } else {
        Ok(vec![])
    }
}

/// Export agent knowledge
#[tauri::command]
pub async fn agent_export_knowledge(
    state: State<'_, AgentState>,
) -> Result<String, String> {
    let agent_guard = state.agent.lock().await;

    if let Some(agent) = agent_guard.as_ref() {
        match agent.export_knowledge().await {
            Ok(knowledge) => Ok(knowledge),
            Err(e) => Err(format!("Export failed: {}", e)),
        }
    } else {
        Err("Agent not initialized".to_string())
    }
}

/// Import agent knowledge
#[tauri::command]
pub async fn agent_import_knowledge(
    knowledge_json: String,
    state: State<'_, AgentState>,
) -> Result<(), String> {
    let agent_guard = state.agent.lock().await;

    if let Some(agent) = agent_guard.as_ref() {
        match agent.import_knowledge(&knowledge_json).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Import failed: {}", e)),
        }
    } else {
        Err("Agent not initialized".to_string())
    }
}

/// Test LLM connection
#[tauri::command]
pub async fn agent_test_llm_connection(
    endpoint: Option<String>,
    api_key: Option<String>,
) -> Result<bool, String> {
    let config = crate::agent::llm::LLMConfig {
        endpoint: endpoint.unwrap_or_else(|| "http://localhost:8000/v1".to_string()),
        model: "Qwen2.5-32B".to_string(),
        embedding_endpoint: "http://localhost:8000/v1/embeddings".to_string(),
        embedding_model: "bge-large-zh-v1.5".to_string(),
        temperature: 0.7,
        max_tokens: 4096,
        api_key: api_key.filter(|k| !k.is_empty()),
    };

    let result: Result<bool, String> = async {
        let client = crate::agent::llm::LLMClient::new(config)
            .await
            .map_err(|e| format!("Failed to create client: {}", e))?;
        client.test_connection()
            .await
            .map_err(|e| format!("Connection test failed: {}", e))
    }.await;

    result
}

/// Response structures for frontend
#[derive(Debug, Clone, serde::Serialize)]
pub struct AgentResponse {
    pub content: String,
    pub tool_results: Vec<ToolResult>,
    pub suggested_actions: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub result: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SkillInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: i32,
}
