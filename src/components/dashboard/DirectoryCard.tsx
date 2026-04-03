import { HardDrive, RefreshCw, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"

interface DirectoryCardProps {
  dir: {
    id: number
    name: string | null
    path: string
    enabled: boolean
  }
  onScan: () => void
  onToggle: () => void
  onDelete: () => void
  loading?: boolean
  scanning?: boolean
  scanProgress?: {
    status: string
    current_file?: string
    processed?: number
    found?: number
  }
}

export function DirectoryCard({
  dir,
  onScan,
  onToggle,
  onDelete,
  loading,
  scanning,
  scanProgress
}: DirectoryCardProps) {
  const isScanning = scanning || false

  return (
    <div style={{
      backgroundColor: "var(--color-card)",
      borderRadius: "12px",
      border: "1px solid var(--color-border)",
      padding: "16px",
      display: "flex",
      alignItems: "center",
      justifyContent: "space-between"
    }}>
      <div style={{ display: "flex", alignItems: "center", gap: "16px", flex: 1, minWidth: 0 }}>
        <div style={{
          width: "48px",
          height: "48px",
          borderRadius: "12px",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          backgroundColor: dir.enabled ? "hsl(221.2 83.2% 53.3% / 0.1)" : "var(--color-muted)"
        }}>
          <HardDrive style={{
            width: "24px",
            height: "24px",
            color: dir.enabled ? "var(--color-primary)" : "var(--color-muted-foreground)"
          }} />
        </div>
        <div style={{ flex: 1, minWidth: 0 }}>
          <h3 style={{ fontWeight: 600, color: "var(--color-foreground)" }}>{dir.name || dir.path}</h3>
          <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)", fontFamily: "monospace", marginTop: "2px" }}>{dir.path}</p>
          {isScanning && scanProgress && scanProgress.status === "scanning" && (
            <div style={{
              marginTop: "8px",
              fontSize: "12px",
              color: "var(--color-primary)",
              display: "flex",
              alignItems: "center",
              gap: "8px"
            }}>
              <span style={{
                display: "inline-block",
                width: "12px",
                height: "12px",
                border: "2px solid var(--color-primary)",
                borderTopColor: "transparent",
                borderRadius: "50%",
                animation: "spin 1s linear infinite"
              }} />
              <span style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                扫描中: {scanProgress.processed || 0} 个文件, 新增 {scanProgress.found || 0} 个
                {scanProgress.current_file && ` - ${scanProgress.current_file}`}
              </span>
            </div>
          )}
        </div>
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: "8px", marginLeft: "16px" }}>
        <Button
          variant="outline"
          size="sm"
          onClick={() => {
            console.log("Scan button onClick triggered, dir.enabled:", dir.enabled, "scanning:", isScanning)
            onScan()
          }}
          disabled={isScanning || loading}
          style={{
            display: "flex",
            alignItems: "center",
            gap: "6px",
            opacity: dir.enabled ? 1 : 0.5,
            position: "relative"
          }}
        >
          <span style={{
            display: "inline-flex",
            animation: isScanning ? "none" : "none"
          }}>
            <RefreshCw style={{
              width: "16px",
              height: "16px",
              animation: isScanning ? "spin 1s linear infinite" : "none"
            }} />
          </span>
          {isScanning ? "扫描中..." : "扫描"}
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onToggle}
          style={{ color: dir.enabled ? "#16a34a" : "#d97706" }}
        >
          {dir.enabled ? "启用" : "禁用"}
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={onDelete}
          style={{ color: "var(--color-destructive)" }}
        >
          <Trash2 style={{ width: "16px", height: "16px" }} />
        </Button>
      </div>
      <style>{`
        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  )
}