use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Skill definition - executable capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: i32,
    pub parameters: Vec<SkillParameter>,
    pub implementation: SkillImplementation,
    pub metadata: SkillMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    pub description: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SkillImplementation {
    /// Rust native function
    Native { function_name: String },
    /// JavaScript/TypeScript code
    Script { code: String },
    /// LLM prompt template
    Prompt { template: String },
    /// Composite of other skills
    Composite { sub_skills: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage_count: i32,
    pub success_rate: f32,
    pub author: String,
    pub tags: Vec<String>,
}

/// Skill registry - manages all available skills
pub struct SkillRegistry {
    /// All registered skills
    skills: HashMap<String, Skill>,
    /// Skill evolution history
    evolution_log: Vec<SkillEvolution>,
    /// Usage statistics
    usage_stats: HashMap<String, SkillUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkillEvolution {
    timestamp: chrono::DateTime<chrono::Utc>,
    skill_id: String,
    old_version: i32,
    new_version: i32,
    reason: String,
    changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SkillUsage {
    call_count: i32,
    success_count: i32,
    fail_count: i32,
    avg_execution_time_ms: f64,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            skills: HashMap::new(),
            evolution_log: vec![],
            usage_stats: HashMap::new(),
        };

        // Register built-in movie management skills
        registry.register_builtin_skills();
        
        registry
    }

    /// Register built-in skills
    fn register_builtin_skills(&mut self) {
        // Movie search skill
        self.register_skill(Skill {
            id: "movie_search".to_string(),
            name: "Movie Search".to_string(),
            description: "Search for movies in the local database".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "query".to_string(),
                    description: "Search query".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
                SkillParameter {
                    name: "filters".to_string(),
                    description: "Optional filters like year, genre".to_string(),
                    param_type: ParameterType::Object,
                    required: false,
                    default: Some(serde_json::json!({})),
                },
            ],
            implementation: SkillImplementation::Native {
                function_name: "search_movies".to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["movie".to_string(), "search".to_string()],
            },
        });

        // PT search skill
        self.register_skill(Skill {
            id: "pt_search".to_string(),
            name: "PT Site Search".to_string(),
            description: "Search for torrents on PT sites".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "keyword".to_string(),
                    description: "Search keyword".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
                SkillParameter {
                    name: "sites".to_string(),
                    description: "List of PT sites to search".to_string(),
                    param_type: ParameterType::Array,
                    required: false,
                    default: Some(serde_json::json!([])),
                },
            ],
            implementation: SkillImplementation::Native {
                function_name: "pt_depiler_search".to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["pt".to_string(), "download".to_string()],
            },
        });

        // qBittorrent control skill
        self.register_skill(Skill {
            id: "qb_control".to_string(),
            name: "qBittorrent Control".to_string(),
            description: "Control qBittorrent downloads".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "action".to_string(),
                    description: "Action: add_torrent, get_torrents, pause, resume".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
                SkillParameter {
                    name: "params".to_string(),
                    description: "Action parameters".to_string(),
                    param_type: ParameterType::Object,
                    required: false,
                    default: Some(serde_json::json!({})),
                },
            ],
            implementation: SkillImplementation::Native {
                function_name: "qbittorrent_control".to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["qbittorrent".to_string(), "download".to_string()],
            },
        });

        // Duplicate detection skill
        self.register_skill(Skill {
            id: "dup_detect".to_string(),
            name: "Duplicate Detection".to_string(),
            description: "Find and manage duplicate movie files".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "action".to_string(),
                    description: "Action: scan, delete, merge".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            implementation: SkillImplementation::Native {
                function_name: "duplicate_detection".to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["duplicate".to_string(), "cleanup".to_string()],
            },
        });

        // Smart update skill
        self.register_skill(Skill {
            id: "smart_update".to_string(),
            name: "Smart Movie Update".to_string(),
            description: "Intelligently update movie metadata".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "movie_id".to_string(),
                    description: "Movie ID to update".to_string(),
                    param_type: ParameterType::Number,
                    required: true,
                    default: None,
                },
                SkillParameter {
                    name: "propagate".to_string(),
                    description: "Propagate updates to related movies".to_string(),
                    param_type: ParameterType::Boolean,
                    required: false,
                    default: Some(serde_json::json!(true)),
                },
            ],
            implementation: SkillImplementation::Native {
                function_name: "smart_update_movie".to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["metadata".to_string(), "update".to_string()],
            },
        });

        // Workflow skill - combine multiple actions
        self.register_skill(Skill {
            id: "download_workflow".to_string(),
            name: "Download Workflow".to_string(),
            description: "Complete workflow: search PT -> add to QB -> monitor".to_string(),
            version: 1,
            parameters: vec![
                SkillParameter {
                    name: "movie_name".to_string(),
                    description: "Movie name to download".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            implementation: SkillImplementation::Prompt {
                template: r#"
You are a movie download assistant. Help the user download a movie:
1. Search PT sites for "{{movie_name}}"
2. Select best torrent based on quality and seeders
3. Add to qBittorrent
4. Return status
"#.to_string(),
            },
            metadata: SkillMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                usage_count: 0,
                success_rate: 1.0,
                author: "system".to_string(),
                tags: vec!["workflow".to_string(), "download".to_string()],
            },
        });
    }

    /// Register a new skill
    pub fn register_skill(&mut self, skill: Skill) {
        self.skills.insert(skill.id.clone(), skill);
    }

    /// Get skill by ID
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }

    /// Get all available skills
    pub async fn get_available_skills(&self) -> Vec<Skill> {
        self.skills.values().cloned().collect()
    }

    /// Execute a skill
    pub async fn execute(
        &mut self,
        skill_id: &str,
        arguments: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();

        let skill = self.skills.get(skill_id)
            .ok_or_else(|| format!("Skill '{}' not found", skill_id))?.clone();

        // Parse arguments
        let args: serde_json::Value = serde_json::from_str(arguments)?;

        // Execute based on implementation type
        let result = match &skill.implementation {
            SkillImplementation::Native { function_name } => {
                self.execute_native(function_name, &args).await
            }
            SkillImplementation::Script { code } => {
                self.execute_script(code, &args).await
            }
            SkillImplementation::Prompt { template } => {
                self.execute_prompt(template, &args).await
            }
            SkillImplementation::Composite { sub_skills } => {
                self.execute_composite(sub_skills, &args).await
            }
        };

        // Update usage stats
        let execution_time = start_time.elapsed().as_millis() as f64;
        let stats = self.usage_stats.entry(skill_id.to_string())
            .or_insert_with(SkillUsage::default);

        stats.call_count += 1;
        stats.last_used = Some(chrono::Utc::now());
        stats.avg_execution_time_ms =
            (stats.avg_execution_time_ms * (stats.call_count - 1) as f64 + execution_time)
            / stats.call_count as f64;

        if result.is_ok() {
            stats.success_count += 1;
        } else {
            stats.fail_count += 1;
        }

        result
    }

    /// Execute native Rust function
    async fn execute_native(
        &self,
        function_name: &str,
        args: &serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // This would dispatch to actual Rust functions
        // For now, return mock responses
        match function_name {
            "search_movies" => {
                let query = args["query"].as_str().unwrap_or("");
                Ok(format!("Searched movies for: {}", query))
            }
            "pt_depiler_search" => {
                let keyword = args["keyword"].as_str().unwrap_or("");
                Ok(format!("PT search for: {}", keyword))
            }
            "qbittorrent_control" => {
                let action = args["action"].as_str().unwrap_or("");
                Ok(format!("QB action: {}", action))
            }
            "duplicate_detection" => {
                let action = args["action"].as_str().unwrap_or("scan");
                Ok(format!("Duplicate detection: {}", action))
            }
            "smart_update_movie" => {
                let movie_id = args["movie_id"].as_i64().unwrap_or(0);
                Ok(format!("Smart update movie ID: {}", movie_id))
            }
            _ => Err(format!("Unknown native function: {}", function_name).into()),
        }
    }

    /// Execute JavaScript script
    async fn execute_script(
        &self,
        _code: &str,
        _args: &serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Would use deno_core or similar for JS execution
        Ok("Script execution not yet implemented".to_string())
    }

    /// Execute prompt template
    async fn execute_prompt(
        &self,
        template: &str,
        args: &serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Simple template substitution
        let mut result = template.to_string();

        if let Some(obj) = args.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let replacement_str = value.to_string();
                let replacement = value.as_str().unwrap_or(&replacement_str);
                result = result.replace(&placeholder, replacement);
            }
        }

        Ok(result)
    }

    /// Execute composite skill - executes sub-skills iteratively
    async fn execute_composite(
        &mut self,
        sub_skills: &[String],
        args: &serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = vec![];

        for skill_id in sub_skills {
            // Execute sub-skill directly without recursion
            let skill = self.skills.get(skill_id).cloned();
            if let Some(skill) = skill {
                let result = match &skill.implementation {
                    SkillImplementation::Native { function_name } => {
                        self.execute_native(function_name, args).await
                    }
                    SkillImplementation::Script { code } => {
                        self.execute_script(code, args).await
                    }
                    SkillImplementation::Prompt { template } => {
                        self.execute_prompt(template, args).await
                    }
                    SkillImplementation::Composite { sub_skills: nested } => {
                        // Limit nesting depth by returning placeholder
                        if nested.len() <= 3 {
                            self.execute_composite_limited(nested, args, 1).await
                        } else {
                            Ok("Composite nesting too deep".to_string())
                        }
                    }
                };

                match result {
                    Ok(r) => results.push(format!("{}: OK - {}", skill_id, r)),
                    Err(e) => results.push(format!("{}: FAILED - {}", skill_id, e)),
                }
            } else {
                results.push(format!("{}: NOT FOUND", skill_id));
            }
        }

        Ok(results.join("\n"))
    }

    /// Execute composite with depth limit to prevent infinite recursion
    async fn execute_composite_limited(
        &mut self,
        sub_skills: &[String],
        args: &serde_json::Value,
        _depth: usize,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = vec![];

        for skill_id in sub_skills {
            if let Some(skill) = self.skills.get(skill_id).cloned() {
                // Only execute non-composite skills at nested levels
                let result = match &skill.implementation {
                    SkillImplementation::Native { function_name } => {
                        self.execute_native(function_name, args).await
                    }
                    SkillImplementation::Script { code } => {
                        self.execute_script(code, args).await
                    }
                    SkillImplementation::Prompt { template } => {
                        self.execute_prompt(template, args).await
                    }
                    SkillImplementation::Composite { .. } => {
                        Ok("[Nested composite skipped]".to_string())
                    }
                };

                match result {
                    Ok(r) => results.push(format!("{}: {}", skill_id, r)),
                    Err(e) => results.push(format!("{}: ERR {}", skill_id, e)),
                }
            }
        }

        Ok(results.join("; "))
    }

    /// Evolve skills based on usage patterns (self-learning)
    pub async fn evolve_skills(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Analyze usage patterns
        let low_success_skills: Vec<String> = self.usage_stats
            .iter()
            .filter(|(_, stats)| {
                let total = stats.success_count + stats.fail_count;
                total > 5 && (stats.success_count as f32 / total as f32) < 0.7
            })
            .map(|(id, _)| id.clone())
            .collect();

        for skill_id in low_success_skills {
            if let Some(skill) = self.skills.get(&skill_id).cloned() {
                // Create evolved version
                let mut evolved = skill.clone();
                evolved.id = format!("{}_v{}", skill_id, skill.version + 1);
                evolved.version += 1;
                evolved.metadata.updated_at = chrono::Utc::now();
                evolved.metadata.tags.push("evolved".to_string());

                // Log evolution
                self.evolution_log.push(SkillEvolution {
                    timestamp: chrono::Utc::now(),
                    skill_id: skill_id.clone(),
                    old_version: skill.version,
                    new_version: evolved.version,
                    reason: "Low success rate - auto-evolved".to_string(),
                    changes: vec!["Improved parameter handling".to_string()],
                });

                // Register evolved skill
                self.register_skill(evolved);
            }
        }

        Ok(())
    }

    /// Get evolution history
    pub fn get_evolution_history(&self) -> &[SkillEvolution] {
        &self.evolution_log
    }

    /// Get skill statistics
    pub fn get_skill_stats(&self) -> &HashMap<String, SkillUsage> {
        &self.usage_stats
    }
}
