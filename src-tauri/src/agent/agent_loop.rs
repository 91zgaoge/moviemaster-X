use super::{AgentManager, AgentConfig, AgentResponse};
use tokio::sync::mpsc;

/// Agent loop events
#[derive(Debug, Clone)]
pub enum AgentEvent {
    UserMessage(String),
    ToolResult(String, String),
    SystemMessage(String),
    ClearContext,
}

/// Agent loop - continuous learning and execution
pub struct AgentLoop {
    agent: AgentManager,
    event_receiver: mpsc::Receiver<AgentEvent>,
    response_sender: mpsc::Sender<AgentResponse>,
}

impl AgentLoop {
    pub async fn new(
        config: AgentConfig,
        event_receiver: mpsc::Receiver<AgentEvent>,
        response_sender: mpsc::Sender<AgentResponse>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let agent = AgentManager::new(config).await?;

        Ok(Self {
            agent,
            event_receiver,
            response_sender,
        })
    }

    /// Run the agent loop
    pub async fn run(mut self) {
        log::info!("Starting AI Agent loop...");

        while let Some(event) = self.event_receiver.recv().await {
            match event {
                AgentEvent::UserMessage(content) => {
                    log::info!("Processing user message: {}", content);

                    match self.agent.process_message(content).await {
                        Ok(response) => {
                            if let Err(e) = self.response_sender.send(response).await {
                                log::error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            let error_msg = e.to_string();
                            log::error!("Agent processing error: {}", error_msg);
                            let error_response = AgentResponse {
                                content: format!("Sorry, I encountered an error: {}", error_msg),
                                tool_results: vec![],
                                suggested_actions: vec![],
                            };
                            if let Err(send_err) = self.response_sender.send(error_response).await {
                                log::error!("Failed to send error response: {}", send_err);
                                break;
                            }
                        }
                    }
                }
                AgentEvent::ToolResult(tool_name, result) => {
                    log::info!("Tool result received for {}: {}", tool_name, result);
                    // Tool results are handled within process_message
                }
                AgentEvent::SystemMessage(content) => {
                    log::info!("System message: {}", content);
                }
                AgentEvent::ClearContext => {
                    log::info!("Clearing agent context");
                    // Context clearing would be implemented here
                }
            }
        }

        log::info!("AI Agent loop ended");
    }
}

/// Create a new agent session with channels
pub async fn create_agent_session(
    config: AgentConfig,
) -> Result<(
    mpsc::Sender<AgentEvent>,
    mpsc::Receiver<AgentResponse>,
), Box<dyn std::error::Error + Send + Sync>> {
    let (event_tx, event_rx) = mpsc::channel(100);
    let (response_tx, response_rx) = mpsc::channel(100);

    let agent_loop = AgentLoop::new(config, event_rx, response_tx).await?;

    // Spawn the agent loop
    tokio::spawn(async move {
        agent_loop.run().await;
    });

    Ok((event_tx, response_rx))
}

/// Parse tool calls from LLM response
pub fn parse_tool_calls(content: &str) -> Vec<ParsedToolCall> {
    let mut tools = vec![];

    // Look for tool call format: <tool>...name: xxx\narguments: {...}</tool>
    let re = regex::Regex::new(r"<tool>\s*name:\s*(\w+)\s*arguments:\s*(\{[^}]*\})\s*</tool>").unwrap();

    for cap in re.captures_iter(content) {
        if let (Some(name), Some(args)) = (cap.get(1), cap.get(2)) {
            tools.push(ParsedToolCall {
                name: name.as_str().to_string(),
                arguments: args.as_str().to_string(),
            });
        }
    }

    tools
}

/// Parsed tool call
#[derive(Debug, Clone)]
pub struct ParsedToolCall {
    pub name: String,
    pub arguments: String,
}

/// Extract clean response (remove tool call markup)
pub fn clean_response(content: &str) -> String {
    let re = regex::Regex::new(r"<tool>[^\u003c]*</tool>").unwrap();
    re.replace_all(content, "[Tool call executed]").to_string()
}
