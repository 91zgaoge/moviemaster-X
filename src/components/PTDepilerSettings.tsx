import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Plus } from "lucide-react"
import type { PTSiteConfig } from "@/lib/api"

interface PTDepilerSettingsProps {
  onClose?: () => void
}

export function PTDepilerSettings({ onClose }: PTDepilerSettingsProps) {
  const [sites] = useState<PTSiteConfig[]>([])
  const [, setIsAdding] = useState(false)

  return (
    <div style={{ padding: "24px" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "24px" }}>
        <h2>PT-Depiler 站点配置</h2>
        <div style={{ display: "flex", gap: "8px" }}>
          <Button variant="outline" onClick={() => setIsAdding(true)}>
            <Plus style={{ width: "16px", height: "16px", marginRight: "4px" }} />
            添加站点
          </Button>
          <Button variant="outline" onClick={onClose}>关闭</Button>
        </div>
      </div>

      <div style={{ fontSize: "14px", color: "#6b7280", marginBottom: "16px" }}>
        PT-Depiler 是一个浏览器扩展，可以通过本应用搜索 PT 站点并下载种子。
      </div>

      {sites.length === 0 && (
        <Card>
          <CardContent style={{ padding: "24px", textAlign: "center" }}>
            <p>尚未配置 PT 站点</p>
            <p style={{ fontSize: "12px", color: "#6b7280", marginTop: "8px" }}>
              点击「添加站点」配置您的 PT 站点信息
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
