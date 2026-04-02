import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Bot, Check, X, RefreshCw, Brain, Server, Key, Thermometer, Eye, EyeOff } from "lucide-react"
import { agentTestLLMConnection, updateSetting, getSettings } from "@/lib/api"

export function LLMSettings() {
  const [config, setConfig] = useState({
    llm_endpoint: "http://localhost:8000/v1",
    model_name: "Qwen2.5-32B",
    embedding_endpoint: "http://localhost:8000/v1/embeddings",
    embedding_model: "bge-large-zh-v1.5",
    temperature: "0.7",
    max_tokens: "4096",
    api_key: "",
  })
  const [showApiKey, setShowApiKey] = useState(false)
  const [isEditing, setIsEditing] = useState(false)
  const [saved, setSaved] = useState(false)
  const [isTesting, setIsTesting] = useState(false)
  const [testResult, setTestResult] = useState<boolean | null>(null)
  const [loading, setLoading] = useState(true)

  // Load saved config
  useEffect(() => {
    const loadConfig = async () => {
      try {
        const settings = await getSettings()
        const llmConfig = {
          llm_endpoint: settings.find(s => s.key === "llm_endpoint")?.value || "http://localhost:8000/v1",
          model_name: settings.find(s => s.key === "llm_model")?.value || "Qwen2.5-32B",
          embedding_endpoint: settings.find(s => s.key === "embedding_endpoint")?.value || "http://localhost:8000/v1/embeddings",
          embedding_model: settings.find(s => s.key === "embedding_model")?.value || "bge-large-zh-v1.5",
          temperature: settings.find(s => s.key === "llm_temperature")?.value || "0.7",
          max_tokens: settings.find(s => s.key === "llm_max_tokens")?.value || "4096",
          api_key: settings.find(s => s.key === "llm_api_key")?.value || "",
        }
        setConfig(llmConfig)
      } catch (error) {
        console.error("Failed to load LLM config:", error)
      } finally {
        setLoading(false)
      }
    }
    loadConfig()
  }, [])

  const handleSave = async () => {
    try {
      await updateSetting("llm_endpoint", config.llm_endpoint)
      await updateSetting("llm_model", config.model_name)
      await updateSetting("embedding_endpoint", config.embedding_endpoint)
      await updateSetting("embedding_model", config.embedding_model)
      await updateSetting("llm_temperature", config.temperature)
      await updateSetting("llm_max_tokens", config.max_tokens)
      await updateSetting("llm_api_key", config.api_key)
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
      setIsEditing(false)
    } catch (error) {
      alert("保存失败: " + error)
    }
  }

  const handleTest = async () => {
    setIsTesting(true)
    setTestResult(null)
    try {
      const result = await agentTestLLMConnection(config.llm_endpoint, config.api_key)
      setTestResult(result)
    } catch (error) {
      setTestResult(false)
    } finally {
      setIsTesting(false)
    }
  }

  const handleChange = (field: string, value: string) => {
    setConfig(prev => ({ ...prev, [field]: value }))
  }

  if (loading) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", padding: "24px" }}>
        <RefreshCw style={{ width: "20px", height: "20px", animation: "spin 1s linear infinite" }} />
      </div>
    )
  }

  return (
    <div>
      {/* Header */}
      <div style={{ marginBottom: "16px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
          <Brain style={{ width: "18px", height: "18px", color: "var(--color-primary)" }} />
          <label style={{ fontSize: "14px", fontWeight: 500, color: "var(--color-foreground)" }}>
            AI 大语言模型配置
          </label>
        </div>
        <p style={{ fontSize: "12px", color: "var(--color-muted-foreground)", marginLeft: "26px" }}>
          配置本地 LLM 服务 (vLLM/Ollama) 以启用 AI 助手功能
        </p>
      </div>

      {/* Connection Status */}
      <div style={{ 
        display: "flex", 
        alignItems: "center", 
        gap: "12px", 
        marginBottom: "16px",
        padding: "12px",
        backgroundColor: "var(--color-muted)",
        borderRadius: "8px"
      }}>
        <div style={{ display: "flex", alignItems: "center", gap: "8px", flex: 1 }}>
          <div
            style={{
              width: "10px",
              height: "10px",
              borderRadius: "50%",
              backgroundColor: testResult === true ? "#22c55e" : testResult === false ? "#ef4444" : "#9ca3af",
            }}
          />
          <span style={{ fontSize: "13px", color: "var(--color-foreground)" }}>
            {testResult === true ? "连接正常" : testResult === false ? "连接失败" : "未测试"}
          </span>
        </div>
        <Button
          variant="outline"
          size="sm"
          onClick={handleTest}
          disabled={isTesting}
          style={{ display: "flex", alignItems: "center", gap: "6px", fontSize: "12px" }}
        >
          {isTesting ? (
            <RefreshCw style={{ width: "14px", height: "14px", animation: "spin 1s linear infinite" }} />
          ) : (
            <RefreshCw style={{ width: "14px", height: "14px" }} />
          )}
          测试连接
        </Button>
      </div>

      {isEditing ? (
        <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
          {/* LLM Endpoint */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              LLM API 地址
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Server style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="text"
                value={config.llm_endpoint}
                onChange={(e) => handleChange("llm_endpoint", e.target.value)}
                placeholder="http://localhost:8000/v1"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* API Key */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              API Key (可选)
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Key style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type={showApiKey ? "text" : "password"}
                value={config.api_key}
                onChange={(e) => handleChange("api_key", e.target.value)}
                placeholder="sk-..."
                style={{ flex: 1 }}
              />
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setShowApiKey(!showApiKey)}
                style={{ padding: "4px", height: "32px", width: "32px" }}
              >
                {showApiKey ? (
                  <EyeOff style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
                ) : (
                  <Eye style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
                )}
              </Button>
            </div>
          </div>

          {/* Model Name */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              模型名称
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Bot style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="text"
                value={config.model_name}
                onChange={(e) => handleChange("model_name", e.target.value)}
                placeholder="Qwen2.5-32B"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* Embedding Endpoint */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              嵌入模型 API 地址
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Server style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="text"
                value={config.embedding_endpoint}
                onChange={(e) => handleChange("embedding_endpoint", e.target.value)}
                placeholder="http://localhost:8000/v1/embeddings"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* Embedding Model */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              嵌入模型名称
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Key style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="text"
                value={config.embedding_model}
                onChange={(e) => handleChange("embedding_model", e.target.value)}
                placeholder="bge-large-zh-v1.5"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* Temperature */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              Temperature (创造性程度)
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Thermometer style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="number"
                min="0"
                max="2"
                step="0.1"
                value={config.temperature}
                onChange={(e) => handleChange("temperature", e.target.value)}
                placeholder="0.7"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* Max Tokens */}
          <div>
            <label style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-muted-foreground)", display: "block", marginBottom: "6px" }}>
              最大 Token 数
            </label>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Key style={{ width: "16px", height: "16px", color: "var(--color-muted-foreground)" }} />
              <Input
                type="number"
                min="256"
                max="8192"
                step="256"
                value={config.max_tokens}
                onChange={(e) => handleChange("max_tokens", e.target.value)}
                placeholder="4096"
                style={{ flex: 1 }}
              />
            </div>
          </div>

          {/* Action Buttons */}
          <div style={{ display: "flex", gap: "8px", marginTop: "8px" }}>
            <Button
              variant="default"
              size="sm"
              onClick={handleSave}
              style={{ display: "flex", alignItems: "center", gap: "4px" }}
            >
              <Check style={{ width: "16px", height: "16px" }} />
              保存
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setIsEditing(false)}
              style={{ display: "flex", alignItems: "center", gap: "4px" }}
            >
              <X style={{ width: "16px", height: "16px" }} />
              取消
            </Button>
          </div>
        </div>
      ) : (
        <div>
          {/* Display Current Config */}
          <div style={{ 
            backgroundColor: "var(--color-muted)", 
            padding: "16px", 
            borderRadius: "8px",
            marginBottom: "16px"
          }}>
            <div style={{ display: "grid", gap: "8px" }}>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "13px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>LLM API:</span>
                <span style={{ color: "var(--color-foreground)", fontFamily: "monospace" }}>{config.llm_endpoint}</span>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "13px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>API Key:</span>
                <span style={{ color: "var(--color-foreground)" }}>
                  {config.api_key ? "已设置" : "未设置"}
                </span>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "13px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>模型:</span>
                <span style={{ color: "var(--color-foreground)" }}>{config.model_name}</span>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "13px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>嵌入模型:</span>
                <span style={{ color: "var(--color-foreground)" }}>{config.embedding_model}</span>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "13px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>Temperature:</span>
                <span style={{ color: "var(--color-foreground)" }}>{config.temperature}</span>
              </div>
            </div>
          </div>

          <Button
            variant="outline"
            size="sm"
            onClick={() => setIsEditing(true)}
          >
            编辑配置
          </Button>
        </div>
      )}

      {saved && (
        <div style={{ 
          marginTop: "12px", 
          padding: "8px 12px", 
          backgroundColor: "#dcfce7", 
          color: "#166534",
          borderRadius: "6px",
          fontSize: "13px",
          display: "flex",
          alignItems: "center",
          gap: "6px"
        }}>
          <Check style={{ width: "14px", height: "14px" }} />
          配置已保存
        </div>
      )}

      <style>{`
        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  )
}
