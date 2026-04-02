import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

interface QBittorrentSettingsProps {
  onClose?: () => void
}

export function QBittorrentSettings({ onClose }: QBittorrentSettingsProps) {
  const [config, setConfig] = useState({
    base_url: "http://10.40.31.69:8044",
    username: "",
    password: "",
  })

  const handleSave = async () => {
    console.log("Save config", config)
  }

  return (
    <div style={{ padding: "24px" }}>
      <h2>qBittorrent 配置</h2>
      <Input value={config.base_url} onChange={e => setConfig({...config, base_url: e.target.value})} placeholder="Server URL" />
      <Input value={config.username} onChange={e => setConfig({...config, username: e.target.value})} placeholder="Username" />
      <Input type="password" value={config.password} onChange={e => setConfig({...config, password: e.target.value})} placeholder="Password" />
      <Button onClick={handleSave}>保存</Button>
      <Button onClick={onClose}>关闭</Button>
    </div>
  )
}
