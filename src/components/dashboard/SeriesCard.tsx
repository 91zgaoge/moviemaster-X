import { convertFileSrc } from "@tauri-apps/api/core"
import { Tv, Calendar, Layers } from "lucide-react"

interface SeriesCardProps {
  series: {
    key: string
    title: string
    year: string | null
    poster_path: string | null
    seasonCount: number
    episodeCount: number
  }
  onClick?: () => void
}

export function SeriesCard({
  series,
  onClick
}: SeriesCardProps) {
  return (
    <div 
      style={{
        backgroundColor: "var(--color-card)",
        borderRadius: "12px",
        overflow: "hidden",
        border: "1px solid var(--color-border)",
        boxShadow: "0 1px 3px rgba(0,0,0,0.1)",
        cursor: "pointer",
        transition: "all 0.3s",
        position: "relative"
      }}
      onClick={onClick}
    >
      {/* Poster */}
      <div style={{ aspectRatio: "2/3", backgroundColor: "var(--color-muted)", position: "relative", overflow: "hidden" }}>
        {series.poster_path ? (
          <img
            src={convertFileSrc(series.poster_path.replace(/\\/g, '/'))}
            alt={series.title}
            style={{ width: "100%", height: "100%", objectFit: "cover" }}
            onError={() => {
              console.error('Poster load error:', series.poster_path);
            }}
          />
        ) : (
          <div style={{ 
            width: "100%", 
            height: "100%", 
            display: "flex", 
            flexDirection: "column",
            alignItems: "center", 
            justifyContent: "center",
            color: "var(--color-muted-foreground)"
          }}>
            <Tv style={{ width: "48px", height: "48px", marginBottom: "8px", opacity: 0.5 }} />
            <span style={{ fontSize: "12px" }}>暂无海报</span>
          </div>
        )}

        {/* Series Badge - Episode Count */}
        <div style={{
          position: "absolute",
          top: "8px",
          right: "8px",
          display: "flex",
          alignItems: "center",
          gap: "4px",
          backgroundColor: "rgba(59, 130, 246, 0.9)",
          color: "white",
          fontSize: "11px",
          fontWeight: "bold",
          padding: "4px 8px",
          borderRadius: "12px"
        }}>
          <Layers style={{ width: "12px", height: "12px" }} />
          {series.episodeCount} 集
        </div>

        {/* Season Count Badge */}
        {series.seasonCount > 0 && (
          <div style={{
            position: "absolute",
            bottom: "8px",
            left: "8px",
            backgroundColor: "rgba(0,0,0,0.7)",
            color: "white",
            fontSize: "11px",
            padding: "4px 8px",
            borderRadius: "4px"
          }}>
            {series.seasonCount} 季
          </div>
        )}
      </div>

      {/* Info */}
      <div style={{ padding: "12px" }}>
        <h3 style={{
          fontWeight: 600,
          fontSize: "14px",
          color: "var(--color-foreground)",
          overflow: "hidden",
          textOverflow: "ellipsis",
          whiteSpace: "nowrap"
        }} title={series.title}>
          {series.title}
        </h3>
        {series.year && (
          <div style={{ display: "flex", alignItems: "center", gap: "4px", marginTop: "4px", fontSize: "12px", color: "var(--color-muted-foreground)" }}>
            <Calendar style={{ width: "12px", height: "12px" }} />
            {series.year}
          </div>
        )}
      </div>
    </div>
  )
}
