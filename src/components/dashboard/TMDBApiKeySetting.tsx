import { useState, useEffect } from "react"
import { Check, X, ExternalLink } from "lucide-react"
import { Button } from "@/components/ui/button"
import { getSettings, updateSetting } from "@/lib/api"

export function TMDBApiKeySetting() {
  const [apiKey, setApiKey] = useState("")
  const [isEditing, setIsEditing] = useState(false)
  const [saved, setSaved] = useState(false)

  useEffect(() => {
    const loadApiKey = async () => {
      try {
        const settings = await getSettings()
        const tmdbKey = settings.find(s => s.key === "tmdb_api_key")
        if (tmdbKey?.value) setApiKey(tmdbKey.value)
      } catch (error) { console.error("Failed to load TMDB API key:", error) }
    }
    loadApiKey()
  }, [])

  const handleSave = async () => {
    try {
      await updateSetting("tmdb_api_key", apiKey || null)
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
      setIsEditing(false)
    } catch (error) { alert("保存失败: " + error) }
  }

  return (
    <div>
      <div style={{ marginBottom: "12px" }}>
        <label style={{ fontSize: "14px", fontWeight: 500, color: "var(--color-foreground)", display: "block", marginBottom: "4px" }}>TMDB API Key</label>
        <p style={{ fontSize: "12px", color: "var(--color-muted-foreground)" }}>
          用于从 The Movie Database 获取影片信息。请到 <a href="https://www.themoviedb.org/settings/api" target="_blank" rel="noopener noreferrer" style={{ color: "var(--color-primary)", textDecoration: "underline" }}>TMDB 网站</a> 申请 API Key
        </p>
      </div>
      {isEditing ? (
        <div style={{ display: "flex", gap: "8px" }}>
          <input type="text" value={apiKey} onChange={(e) => setApiKey(e.target.value)} placeholder="输入您的 TMDB API Key" style={{ flex: 1, height: "40px", padding: "0 12px", borderRadius: "6px", border: "1px solid var(--color-border)", backgroundColor: "var(--color-background)", fontSize: "14px" }} />
          <Button variant="default" size="sm" onClick={handleSave}><Check style={{ width: "16px", height: "16px" }} />保存</Button>
          <Button variant="outline" size="sm" onClick={() => setIsEditing(false)}><X style={{ width: "16px", height: "16px" }} />取消</Button>
        </div>
      ) : (
        <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
          <div style={{ flex: 1, height: "40px", padding: "0 12px", borderRadius: "6px", border: "1px solid var(--color-border)", backgroundColor: "var(--color-muted)", display: "flex", alignItems: "center", fontSize: "14px", color: apiKey ? "var(--color-foreground)" : "var(--color-muted-foreground)", fontFamily: "monospace" }}>{apiKey ? `${apiKey.substring(0, 20)}...` : "使用默认 API Key"}</div>
          <Button variant="outline" size="sm" onClick={() => setIsEditing(true)}><ExternalLink style={{ width: "16px", height: "16px" }} />{apiKey ? "修改" : "设置"}</Button>
          {saved && <span style={{ color: "#16a34a", fontSize: "14px" }}><Check style={{ width: "16px", height: "16px" }} />已保存</span>}
        </div>
      )}
    </div>
  )
}
