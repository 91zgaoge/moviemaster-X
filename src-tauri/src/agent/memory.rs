use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Memory configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub embedding_endpoint: String,
    pub embedding_model: String,
    pub max_entries: usize,
    pub similarity_threshold: f32,
}

/// Memory entry types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// Short-term: ephemeral, session-based
    ShortTerm,
    /// Long-term: persisted, semantic knowledge
    LongTerm,
    /// Skill memory: learned behaviors
    Skill,
}

/// Memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub memory_type: MemoryType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(skip)]
    pub embedding: Vec<f32>,
    pub metadata: Option<serde_json::Value>,
}

/// Dual-track memory: frozen snapshot + live state (Hermes Agent architecture)
pub struct DualMemory {
    config: MemoryConfig,
    /// Frozen snapshot - immutable system knowledge
    frozen_db: Arc<Mutex<Connection>>,
    /// Live state - mutable working memory
    live_db: Arc<Mutex<Connection>>,
    /// In-memory embedding cache for fast similarity search
    embedding_cache: Arc<Mutex<Vec<MemoryEntry>>>,
    llm_client: Arc<Mutex<super::llm::LLMClient>>,
}

impl DualMemory {
    pub async fn new(config: MemoryConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create frozen memory (in-memory for fast access)
        let frozen_db = Connection::open_in_memory()?;
        Self::init_frozen_schema(&frozen_db)?;

        // Create live memory (in-memory with periodic persistence)
        let live_db = Connection::open_in_memory()?;
        Self::init_live_schema(&live_db)?;

        let llm_config = super::llm::LLMConfig {
            endpoint: config.embedding_endpoint.clone(),
            model: config.embedding_model.clone(),
            embedding_endpoint: config.embedding_endpoint.clone(),
            embedding_model: config.embedding_model.clone(),
            temperature: 0.7,
            max_tokens: 4096,
            api_key: None,
        };

        let llm_client = super::llm::LLMClient::new(llm_config).await?;

        Ok(Self {
            config,
            frozen_db: Arc::new(Mutex::new(frozen_db)),
            live_db: Arc::new(Mutex::new(live_db)),
            embedding_cache: Arc::new(Mutex::new(vec![])),
            llm_client: Arc::new(Mutex::new(llm_client)),
        })
    }

    /// Initialize frozen memory schema
    fn init_frozen_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS frozen_memories (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                memory_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                embedding BLOB,
                metadata TEXT
            )",
            [],
        )?;

        // Create index for fast retrieval
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_frozen_type ON frozen_memories(memory_type)",
            [],
        )?;

        Ok(())
    }

    /// Initialize live memory schema
    fn init_live_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS live_memories (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                memory_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                embedding BLOB,
                metadata TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_live_timestamp ON live_memories(timestamp)",
            [],
        )?;

        Ok(())
    }

    /// Store memory entry
    pub async fn store(&self,
        mut entry: MemoryEntry,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Generate embedding
        let llm = self.llm_client.lock().await;
        let embedding = llm.embed(&entry.content).await?;
        entry.embedding = embedding.clone();

        // Serialize embedding to bytes
        let embedding_bytes = if !embedding.is_empty() {
            let bytes: Vec<u8> = embedding.iter()
                .flat_map(|f| f.to_le_bytes().to_vec())
                .collect();
            Some(bytes)
        } else {
            None
        };

        let metadata_json = entry.metadata.as_ref()
            .map(|m| m.to_string())
            .unwrap_or_default();

        // Store in appropriate database
        match entry.memory_type {
            MemoryType::ShortTerm => {
                let db = self.live_db.lock().await;
                db.execute(
                    "INSERT OR REPLACE INTO live_memories (id, content, memory_type, timestamp, embedding, metadata)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        entry.id,
                        entry.content,
                        "short_term",
                        entry.timestamp.to_rfc3339(),
                        embedding_bytes,
                        metadata_json
                    ],
                )?;
            }
            MemoryType::LongTerm | MemoryType::Skill => {
                let db = self.frozen_db.lock().await;
                db.execute(
                    "INSERT OR REPLACE INTO frozen_memories (id, content, memory_type, timestamp, embedding, metadata)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        entry.id,
                        entry.content,
                        match entry.memory_type {
                            MemoryType::LongTerm => "long_term",
                            MemoryType::Skill => "skill",
                            _ => unreachable!(),
                        },
                        entry.timestamp.to_rfc3339(),
                        embedding_bytes,
                        metadata_json
                    ],
                )?;
            }
        }

        // Update cache
        let mut cache = self.embedding_cache.lock().await;
        cache.push(entry.clone());
        
        // Trim cache if too large
        if cache.len() > self.config.max_entries {
            cache.remove(0);
        }

        Ok(())
    }

    /// Retrieve similar memories using cosine similarity
    pub async fn retrieve_similar(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>, Box<dyn std::error::Error + Send + Sync>> {
        // Generate query embedding
        let llm = self.llm_client.lock().await;
        let query_embedding = llm.embed(query).await?;

        // Search in cache first
        let cache = self.embedding_cache.lock().await;
        let mut results: Vec<(f32, MemoryEntry)> = cache
            .iter()
            .map(|entry| {
                let similarity = if entry.embedding.is_empty() {
                    0.0
                } else {
                    cosine_similarity(&query_embedding, &entry.embedding)
                };
                (similarity, entry.clone())
            })
            .filter(|(sim, _)| *sim >= self.config.similarity_threshold)
            .collect();

        // Sort by similarity descending
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Return top results
        Ok(results.into_iter().take(limit).map(|(_, entry)| entry).collect())
    }

    /// Get frozen snapshot (immutable system knowledge)
    pub async fn get_frozen_snapshot(&self,
    ) -> Result<Vec<MemoryEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let db = self.frozen_db.lock().await;
        let mut stmt = db.prepare(
            "SELECT id, content, memory_type, timestamp, metadata FROM frozen_memories
             ORDER BY timestamp DESC"
        )?;

        let entries = stmt.query_map([], |row| {
            let timestamp_str: String = row.get(3)?;
            Ok(MemoryEntry {
                id: row.get(0)?,
                content: row.get(1)?,
                memory_type: {
                    let mem_type: String = row.get(2)?;
                    match mem_type.as_str() {
                        "long_term" => MemoryType::LongTerm,
                        "skill" => MemoryType::Skill,
                        _ => MemoryType::ShortTerm,
                    }
                },
                timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now),
                embedding: vec![],
                metadata: {
                    let meta_opt: Option<String> = row.get(4)?;
                    meta_opt.and_then(|s| serde_json::from_str(&s).ok())
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Get live state (mutable working memory)
    pub async fn get_live_state(&self,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let db = self.live_db.lock().await;
        let mut stmt = db.prepare(
            "SELECT id, content, memory_type, timestamp, metadata FROM live_memories
             ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let entries = stmt.query_map([limit], |row| {
            let timestamp_str: String = row.get(3)?;
            Ok(MemoryEntry {
                id: row.get(0)?,
                content: row.get(1)?,
                memory_type: MemoryType::ShortTerm,
                timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now),
                embedding: vec![],
                metadata: {
                    let meta_opt: Option<String> = row.get(4)?;
                    meta_opt.and_then(|s| serde_json::from_str(&s).ok())
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Promote short-term to long-term memory (consolidation)
    pub async fn consolidate(&self, entry_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get entry from live memory
        let entry = {
            let db = self.live_db.lock().await;
            let mut stmt = db.prepare(
                "SELECT id, content, timestamp, metadata FROM live_memories WHERE id = ?1"
            )?;
            
            stmt.query_row([entry_id], |row| {
                let timestamp_str: String = row.get(2)?;
                Ok(MemoryEntry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    memory_type: MemoryType::LongTerm,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    embedding: vec![],
                    metadata: {
                        let meta_opt: Option<String> = row.get(3)?;
                        meta_opt.and_then(|s| serde_json::from_str(&s).ok())
                    },
                })
            })
        };

        if let Ok(entry) = entry {
            // Store in frozen memory
            self.store(entry).await?;

            // Remove from live memory
            let db = self.live_db.lock().await;
            db.execute("DELETE FROM live_memories WHERE id = ?1", [entry_id])?;
        }

        Ok(())
    }

    /// Export all memories
    pub async fn export_all(&self,
    ) -> Result<Vec<MemoryEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all = self.get_frozen_snapshot().await?;
        all.extend(self.get_live_state(10000).await?);
        Ok(all)
    }

    /// Clear live memory (garbage collection)
    pub async fn clear_live_memory(&self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let db = self.live_db.lock().await;
        db.execute("DELETE FROM live_memories", [])?;
        
        let mut cache = self.embedding_cache.lock().await;
        cache.retain(|e| matches!(e.memory_type, MemoryType::LongTerm | MemoryType::Skill));
        
        Ok(())
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}
