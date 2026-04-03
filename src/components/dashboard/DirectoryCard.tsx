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
  loading: boolean
}

export function DirectoryCard({
  dir,
  onScan,
  onToggle,
  onDelete,
  loading
}: DirectoryCardProps) {
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
      <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
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
        <div>
          <h3 style={{ fontWeight: 600, color: "var(--color-foreground)" }}>{dir.name || dir.path}</h3>
          <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)", fontFamily: "monospace", marginTop: "2px" }}>{dir.path}</p>
        </div>
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
        <Button 
          variant="outline" 
          size="sm" 
          onClick={() => {
            console.log("Scan button onClick triggered, dir.enabled:", dir.enabled, "loading:", loading)
            onScan()
          }} 
          disabled={loading} 
          style={{ display: "flex", alignItems: "center", gap: "6px", opacity: dir.enabled ? 1 : 0.5 }}
        >
          <RefreshCw style={{ width: "16px", height: "16px", animation: loading ? "spin 1s linear infinite" : "none" }} />
          扫描
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
    </div>
  )
}