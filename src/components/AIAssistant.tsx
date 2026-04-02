import { useState, useRef, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  MessageSquare,
  Send,
  X,
  Bot,
  User,
  Loader2,
  Sparkles,
  Wrench,
} from "lucide-react"
import {
  agentSendMessage,
  agentGetAvailableSkills,
  agentTestLLMConnection,
  type AgentResponse,
  type SkillInfo,
} from "@/lib/api"

interface Message {
  id: string
  role: "user" | "assistant" | "system"
  content: string
  timestamp: Date
  toolResults?: { tool_name: string; success: boolean; result: string }[]
}

interface AIAssistantProps {
  onClose?: () => void
}

export function AIAssistant({ onClose }: AIAssistantProps) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: "welcome",
      role: "assistant",
      content: "你好！我是MovieMaster AI助手。我可以帮你：\n\n• 搜索和管理电影\n• 通过PT站点查找下载\n• 控制qBittorrent下载\n• 检测重复文件\n• 智能更新元数据\n\n有什么可以帮你的吗？",
      timestamp: new Date(),
    },
  ])
  const [input, setInput] = useState("")
  const [isLoading, setIsLoading] = useState(false)
  const [isConnected, setIsConnected] = useState<boolean | null>(null)
  const [skills, setSkills] = useState<SkillInfo[]>([])
  const [showSkills, setShowSkills] = useState(false)
  const scrollRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)

  // Auto-scroll to bottom
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
    }
  }, [messages])

  // Test LLM connection on mount
  useEffect(() => {
    testConnection()
    loadSkills()
  }, [])

  const testConnection = async () => {
    try {
      const connected = await agentTestLLMConnection()
      setIsConnected(connected)
    } catch {
      setIsConnected(false)
    }
  }

  const loadSkills = async () => {
    try {
      const availableSkills = await agentGetAvailableSkills()
      setSkills(availableSkills)
    } catch (error) {
      console.error("Failed to load skills:", error)
    }
  }

  const handleSend = async () => {
    if (!input.trim() || isLoading) return

    const userMessage: Message = {
      id: Date.now().toString(),
      role: "user",
      content: input.trim(),
      timestamp: new Date(),
    }

    setMessages((prev) => [...prev, userMessage])
    setInput("")
    setIsLoading(true)

    try {
      const response: AgentResponse = await agentSendMessage(userMessage.content)

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: response.content,
        timestamp: new Date(),
        toolResults: response.tool_results,
      }

      setMessages((prev) => [...prev, assistantMessage])
    } catch (error) {
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: `抱歉，发生了错误：${error}`,
        timestamp: new Date(),
      }
      setMessages((prev) => [...prev, errorMessage])
    } finally {
      setIsLoading(false)
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const quickActions = [
    { label: "搜索电影", prompt: "帮我搜索电影" },
    { label: "PT下载", prompt: "搜索PT站点下载" },
    { label: "检测重复", prompt: "检测重复文件" },
    { label: "QB状态", prompt: "查看qBittorrent下载状态" },
  ]

  return (
    <Card
      style={{
        position: "fixed",
        bottom: "24px",
        right: "24px",
        width: "400px",
        height: "600px",
        zIndex: 50,
        display: "flex",
        flexDirection: "column",
        boxShadow: "0 25px 50px -12px rgba(249, 115, 22, 0.4)",
        border: "2px solid #f97316",
        background: "linear-gradient(180deg, #fff7ed 0%, #ffedd5 100%)",
      }}
    >
      {/* Header */}
      <CardHeader
        style={{
          padding: "16px",
          borderBottom: "2px solid #f97316",
          background: "linear-gradient(135deg, #f97316 0%, #ea580c 50%, #dc2626 100%)",
        }}
      >
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
            <div
              style={{
                width: "36px",
                height: "36px",
                borderRadius: "10px",
                backgroundColor: "rgba(255, 255, 255, 0.2)",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <Bot style={{ width: "20px", height: "20px", color: "white" }} />
            </div>
            <div>
              <CardTitle style={{ fontSize: "16px", color: "white", marginBottom: "2px" }}>
                AI 助手
              </CardTitle>
              <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
                <div
                  style={{
                    width: "8px",
                    height: "8px",
                    borderRadius: "50%",
                    backgroundColor: isConnected === true ? "#22c55e" : isConnected === false ? "#ef4444" : "#f59e0b",
                  }}
                />
                <span style={{ fontSize: "12px", color: "rgba(255, 255, 255, 0.8)" }}>
                  {isConnected === true ? "已连接" : isConnected === false ? "未连接" : "检测中..."}
                </span>
              </div>
            </div>
          </div>
          <div style={{ display: "flex", gap: "8px" }}>
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setShowSkills(!showSkills)}
              style={{ color: "rgba(255, 255, 255, 0.8)", backgroundColor: "rgba(255, 255, 255, 0.1)" }}
            >
              <Wrench style={{ width: "16px", height: "16px" }} />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              onClick={onClose}
              style={{ color: "rgba(255, 255, 255, 0.8)", backgroundColor: "rgba(255, 255, 255, 0.1)" }}
            >
              <X style={{ width: "16px", height: "16px" }} />
            </Button>
          </div>
        </div>
      </CardHeader>

      {/* Skills Panel */}
      {showSkills && (
        <div
          style={{
            padding: "12px",
            background: "linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)",
            borderBottom: "2px solid #f97316",
          }}
        >
          <div style={{ fontSize: "12px", fontWeight: 600, color: "#9a3412", marginBottom: "8px" }}>
            可用技能
          </div>
          <div style={{ display: "flex", flexWrap: "wrap", gap: "6px" }}>
            {skills.map((skill) => (
              <span
                key={skill.id}
                style={{
                  fontSize: "11px",
                  padding: "4px 10px",
                  borderRadius: "12px",
                  backgroundColor: "#f97316",
                  color: "white",
                  fontWeight: 500,
                }}
              >
                {skill.name}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Messages */}
      <CardContent
        ref={scrollRef}
        style={{
          flex: 1,
          overflow: "auto",
          padding: "16px",
          display: "flex",
          flexDirection: "column",
          gap: "12px",
        }}
      >
        {messages.map((message) => (
          <div
            key={message.id}
            style={{
              display: "flex",
              gap: "10px",
              alignItems: "flex-start",
              flexDirection: message.role === "user" ? "row-reverse" : "row",
            }}
          >
            <div
              style={{
                width: "28px",
                height: "28px",
                borderRadius: "8px",
                background: message.role === "user"
                  ? "linear-gradient(135deg, #f97316 0%, #ea580c 100%)"
                  : "linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                flexShrink: 0,
                boxShadow: message.role === "user" ? "0 2px 8px rgba(249, 115, 22, 0.4)" : "none",
              }}
            >
              {message.role === "user" ? (
                <User style={{ width: "14px", height: "14px", color: "white" }} />
              ) : (
                <Sparkles style={{ width: "14px", height: "14px", color: "#ea580c" }} />
              )}
            </div>
            <div
              style={{
                maxWidth: "calc(100% - 50px)",
                padding: "10px 14px",
                borderRadius: "12px",
                background: message.role === "user"
                  ? "linear-gradient(135deg, #f97316 0%, #ea580c 100%)"
                  : "linear-gradient(135deg, #fff7ed 0%, #ffedd5 100%)",
                color: message.role === "user" ? "white" : "#7c2d12",
                fontSize: "14px",
                lineHeight: "1.5",
                whiteSpace: "pre-wrap",
                boxShadow: message.role === "user" ? "0 2px 8px rgba(249, 115, 22, 0.3)" : "0 1px 3px rgba(0,0,0,0.1)",
                border: message.role === "user" ? "none" : "1px solid #fed7aa",
              }}
            >
              {message.content}
              {message.toolResults && message.toolResults.length > 0 && (
                <div style={{ marginTop: "8px", paddingTop: "8px", borderTop: "1px solid rgba(255,255,255,0.2)" }}>
                  {message.toolResults.map((tool, idx) => (
                    <div key={idx} style={{ fontSize: "11px", opacity: 0.8 }}>
                      {tool.success ? "✓" : "✗"} {tool.tool_name}
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
        {isLoading && (
          <div style={{ display: "flex", gap: "10px", alignItems: "flex-start" }}>
            <div
              style={{
                width: "28px",
                height: "28px",
                borderRadius: "8px",
                background: "linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <Sparkles style={{ width: "14px", height: "14px", color: "#ea580c" }} />
            </div>
            <div
              style={{
                padding: "10px 14px",
                borderRadius: "12px",
                background: "linear-gradient(135deg, #fff7ed 0%, #ffedd5 100%)",
                display: "flex",
                alignItems: "center",
                gap: "8px",
                border: "1px solid #fed7aa",
                color: "#7c2d12",
              }}
            >
              <Loader2 style={{ width: "14px", height: "14px", animation: "spin 1s linear infinite", color: "#f97316" }} />
              <span style={{ fontSize: "14px" }}>思考中...</span>
            </div>
          </div>
        )}
      </CardContent>

      {/* Quick Actions */}
      <div
        style={{
          padding: "8px 16px",
          borderTop: "2px solid #fed7aa",
          background: "linear-gradient(180deg, #fff7ed 0%, #ffedd5 100%)",
          display: "flex",
          gap: "8px",
          overflowX: "auto",
        }}
      >
        {quickActions.map((action) => (
          <Button
            key={action.label}
            variant="outline"
            size="sm"
            onClick={() => {
              setInput(action.prompt)
              inputRef.current?.focus()
            }}
            style={{
              fontSize: "12px",
              whiteSpace: "nowrap",
              borderColor: "#f97316",
              color: "#ea580c",
              backgroundColor: "white",
            }}
          >
            {action.label}
          </Button>
        ))}
      </div>

      {/* Input */}
      <div
        style={{
          padding: "16px",
          borderTop: "2px solid #fed7aa",
          background: "linear-gradient(180deg, #ffedd5 0%, #fed7aa 100%)",
          display: "flex",
          gap: "10px",
        }}
      >
        <Input
          ref={inputRef}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="输入消息..."
          disabled={isLoading}
          style={{
            flex: 1,
            borderColor: "#f97316",
            backgroundColor: "white",
          }}
        />
        <Button
          onClick={handleSend}
          disabled={isLoading || !input.trim()}
          size="icon"
          style={{
            background: "linear-gradient(135deg, #f97316 0%, #ea580c 100%)",
            border: "none",
          }}
        >
          {isLoading ? (
            <Loader2 style={{ width: "16px", height: "16px", animation: "spin 1s linear infinite" }} />
          ) : (
            <Send style={{ width: "16px", height: "16px" }} />
          )}
        </Button>
      </div>

      <style>{`
        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
    </Card>
  )
}

// Floating button to open AI Assistant
export function AIAssistantButton({ onClick }: { onClick: () => void }) {
  return (
    <Button
      onClick={onClick}
      style={{
        position: "fixed",
        bottom: "24px",
        right: "24px",
        width: "56px",
        height: "56px",
        borderRadius: "50%",
        background: "linear-gradient(135deg, #f97316 0%, #ea580c 50%, #dc2626 100%)",
        boxShadow: "0 10px 25px -5px rgba(249, 115, 22, 0.5)",
        zIndex: 50,
        padding: 0,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        border: "2px solid #fff7ed",
        transition: "all 0.3s ease",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.transform = "scale(1.1)"
        e.currentTarget.style.boxShadow = "0 15px 35px -5px rgba(249, 115, 22, 0.6)"
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.transform = "scale(1)"
        e.currentTarget.style.boxShadow = "0 10px 25px -5px rgba(249, 115, 22, 0.5)"
      }}
    >
      <MessageSquare style={{ width: "24px", height: "24px", color: "white" }} />
    </Button>
  )
}
