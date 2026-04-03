import { useState } from "react"
import { convertFileSrc } from "@tauri-apps/api/core"
import { Film, Star, Calendar, Clapperboard } from "lucide-react"

interface MovieCardProps {
  movie: {
    id: number
    poster_path: string | null
    cnname: string | null
    filename: string
    douban_rating: number | null
    year: string | null
    video_type: string
    season: string | null
    episode: string | null
  }
  isSelected?: boolean
  isBatchMode?: boolean
  onToggleSelect?: () => void
  onClick?: () => void
  onSearchTMDB: () => void
}

export function MovieCard({
  movie,
  isSelected,
  isBatchMode,
  onToggleSelect,
  onClick,
  onSearchTMDB
}: MovieCardProps) {
  const [showActions, setShowActions] = useState(false)

  return (
    <div 
      style={{
        backgroundColor: isSelected ? "rgba(59, 130, 246, 0.1)" : "var(--color-card)",
        borderRadius: "12px",
        overflow: "hidden",
        border: isSelected ? "2px solid rgb(59, 130, 246)" : "1px solid var(--color-border)",
        boxShadow: "0 1px 3px rgba(0,0,0,0.1)",
        cursor: isBatchMode ? "default" : "pointer",
        transition: "all 0.3s",
        position: "relative"
      }}
      onClick={() => {
        if (isBatchMode) {
          onToggleSelect?.()
        } else {
          onClick?.()
        }
      }}
      onMouseEnter={() => setShowActions(true)}
      onMouseLeave={() => setShowActions(false)}
    >
      {/* Poster */}
      <div style={{ aspectRatio: "2/3", backgroundColor: "var(--color-muted)", position: "relative", overflow: "hidden" }}>
        {/* Batch Selection Checkbox */}
        {isBatchMode && (
          <div
            style={{
              position: "absolute",
              top: "8px",
              left: "8px",
              zIndex: 10,
              backgroundColor: "rgba(255,255,255,0.9)",
              borderRadius: "4px",
              padding: "4px"
            }}
            onClick={(e) => e.stopPropagation()}
          >
            <input
              type="checkbox"
              checked={isSelected}
              onChange={onToggleSelect}
              style={{ width: "20px", height: "20px", cursor: "pointer" }}
            />
          </div>
        )}
        {movie.poster_path ? (
          <img
            src={convertFileSrc(movie.poster_path.replace(/\\/g, '/'))}
            alt={movie.cnname || movie.filename}
            style={{ width: "100%", height: "100%", objectFit: "cover" }}
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
            <Film style={{ width: "48px", height: "48px", marginBottom: "8px", opacity: 0.5 }} />
            <span style={{ fontSize: "12px" }}>暂无海报</span>
          </div>
        )}

        {/* Rating Badge */}
        {movie.douban_rating && (
          <div style={{
            position: "absolute",
            top: "8px",
            right: "8px",
            display: "flex",
            alignItems: "center",
            gap: "2px",
            backgroundColor: "#eab308",
            color: "white",
            fontSize: "12px",
            fontWeight: "bold",
            padding: "4px 8px",
            borderRadius: "12px"
          }}>
            <Star style={{ width: "12px", height: "12px" }} />
            {movie.douban_rating.toFixed(1)}
          </div>
        )}

        {/* Episode Badge */}
        {movie.video_type === "tv" && movie.season && (
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
            S{movie.season}E{movie.episode}
          </div>
        )}

        {/* TMDB Action Button */}
        {showActions && (
          <div style={{
            position: "absolute",
            top: "8px",
            left: "8px",
            display: "flex",
            gap: "8px"
          }}>
            <button
              onClick={(e) => {
                e.stopPropagation()
                onSearchTMDB()
              }}
              style={{
                backgroundColor: "rgba(0,0,0,0.7)",
                color: "white",
                border: "none",
                borderRadius: "6px",
                padding: "6px 10px",
                fontSize: "11px",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
                gap: "4px",
                backdropFilter: "blur(4px)"
              }}
              title="从 TMDB 获取信息"
            >
              <Clapperboard style={{ width: "12px", height: "12px" }} />
              TMDB
            </button>
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
        }} title={movie.cnname || movie.filename}>
          {movie.cnname || movie.filename}
        </h3>
        {movie.year && (
          <div style={{ display: "flex", alignItems: "center", gap: "4px", marginTop: "4px", fontSize: "12px", color: "var(--color-muted-foreground)" }}>
            <Calendar style={{ width: "12px", height: "12px" }} />
            {movie.year}
          </div>
        )}
      </div>
    </div>
  )
}