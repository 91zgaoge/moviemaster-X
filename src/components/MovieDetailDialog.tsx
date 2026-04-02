import { useState } from "react"
import { convertFileSrc } from "@tauri-apps/api/core"
import { Play, Download, FileText, Star, Calendar, Clock, Clapperboard, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import type { Movie } from "@/lib/api"

interface MovieDetailDialogProps {
  movie: Movie | null
  isOpen: boolean
  onClose: () => void
  onPlay?: () => void
  onGenerateNfo?: () => void
  onUpdateFromTMDB?: () => void
  onDelete?: () => void
}

export function MovieDetailDialog({
  movie,
  isOpen,
  onClose,
  onPlay,
  onGenerateNfo,
  onUpdateFromTMDB,
  onDelete,
}: MovieDetailDialogProps) {
  const [activeTab, setActiveTab] = useState<"info" | "files" | "metadata">("info")

  if (!movie) return null

  const getPosterUrl = () => {
    if (movie.poster_path) {
      return convertFileSrc(movie.poster_path.replace(/\\/g, '/'))
    }
    return null
  }

  const formatFileSize = (bytes: number | null) => {
    if (!bytes) return "未知"
    const gb = bytes / (1024 * 1024 * 1024)
    if (gb >= 1) return `${gb.toFixed(2)} GB`
    const mb = bytes / (1024 * 1024)
    return `${mb.toFixed(2)} MB`
  }

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent style={{ maxWidth: "800px", maxHeight: "90vh", overflow: "auto" }}>
        <DialogHeader>
          <DialogTitle>影片详情</DialogTitle>
        </DialogHeader>

        <div style={{ display: "flex", gap: "24px", marginTop: "16px" }}>
          {/* 左侧：海报 */}
          <div style={{ width: "200px", flexShrink: 0 }}>
            <div
              style={{
                aspectRatio: "2/3",
                backgroundColor: "var(--color-muted)",
                borderRadius: "8px",
                overflow: "hidden",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              {getPosterUrl() ? (
                <img
                  src={getPosterUrl()!}
                  alt={movie.cnname || movie.filename}
                  style={{ width: "100%", height: "100%", objectFit: "cover" }}
                />
              ) : (
                <div style={{ textAlign: "center", color: "var(--color-muted-foreground)" }}>
                  <Clapperboard style={{ width: "48px", height: "48px", margin: "0 auto 8px" }} />
                  <div style={{ fontSize: "12px" }}>暂无海报</div>
                </div>
              )}
            </div>

            {/* 操作按钮 */}
            <div style={{ marginTop: "16px", display: "flex", flexDirection: "column", gap: "8px" }}>
              <Button
                variant="default"
                style={{ width: "100%", display: "flex", alignItems: "center", gap: "8px" }}
                onClick={onPlay}
              >
                <Play style={{ width: "16px", height: "16px" }} />
                播放影片
              </Button>

              <Button
                variant="outline"
                style={{ width: "100%", display: "flex", alignItems: "center", gap: "8px" }}
                onClick={onUpdateFromTMDB}
              >
                <Download style={{ width: "16px", height: "16px" }} />
                更新信息
              </Button>

              <Button
                variant="outline"
                style={{ width: "100%", display: "flex", alignItems: "center", gap: "8px" }}
                onClick={onGenerateNfo}
              >
                <FileText style={{ width: "16px", height: "16px" }} />
                生成NFO
              </Button>

              <Button
                variant="outline"
                style={{ width: "100%", display: "flex", alignItems: "center", gap: "8px", color: "var(--color-destructive)" }}
                onClick={onDelete}
              >
                <Trash2 style={{ width: "16px", height: "16px" }} />
                删除记录
              </Button>
            </div>
          </div>

          {/* 右侧：详情信息 */}
          <div style={{
            flex: 1,
            backgroundColor: "rgb(240, 242, 245)",
            borderRadius: "12px",
            padding: "20px"
          }}>
            {/* 标题 */}
            <h2 style={{ fontSize: "24px", fontWeight: "bold", marginBottom: "8px" }}>
              {movie.cnname || movie.filename}
            </h2>

            {movie.cnoname && (
              <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)", marginBottom: "16px" }}>
                {movie.cnoname}
              </p>
            )}

            {/* 评分和元数据 */}
            <div style={{ display: "flex", gap: "16px", marginBottom: "24px", flexWrap: "wrap" }}>
              {movie.douban_rating && (
                <div style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                  <Star style={{ width: "16px", height: "16px", color: "#eab308" }} />
                  <span style={{ fontWeight: 600 }}>{movie.douban_rating.toFixed(1)}</span>
                </div>
              )}

              {movie.year && (
                <div style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                  <Calendar style={{ width: "16px", height: "16px" }} />
                  <span>{movie.year}</span>
                </div>
              )}

              <div style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                <Clock style={{ width: "16px", height: "16px" }} />
                <span>{formatFileSize(movie.file_size)}</span>
              </div>

              {movie.video_type === "tv" && movie.season && (
                <div>
                  第 {movie.season} 季
                  {movie.episode && ` 第 ${movie.episode} 集`}
                </div>
              )}
            </div>

            {/* 标签页 */}
            <div style={{ display: "flex", gap: "4px", borderBottom: "1px solid var(--color-border)", marginBottom: "16px" }}>
              {[
                { key: "info", label: "基本信息" },
                { key: "files", label: "文件信息" },
                { key: "metadata", label: "元数据" },
              ].map((tab) => (
                <button
                  key={tab.key}
                  onClick={() => setActiveTab(tab.key as any)}
                  style={{
                    padding: "8px 16px",
                    border: "none",
                    borderBottom: activeTab === tab.key ? "2px solid var(--color-primary)" : "2px solid transparent",
                    backgroundColor: activeTab === tab.key ? "var(--color-background)" : "transparent",
                    color: activeTab === tab.key ? "var(--color-primary)" : "var(--color-muted-foreground)",
                    cursor: "pointer",
                    fontWeight: activeTab === tab.key ? 600 : 400,
                    borderRadius: "6px 6px 0 0",
                    transition: "all 0.2s"
                  }}
                >
                  {tab.label}
                </button>
              ))}
            </div>

            {/* 标签页内容 */}
            <div style={{
              backgroundColor: "rgb(255, 255, 255)",
              borderRadius: "8px",
              padding: "16px",
              border: "1px solid rgb(220, 223, 230)"
            }}>
              {activeTab === "info" && (
                <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                  {movie.description && (
                    <div>
                      <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>简介</label>
                      <p style={{ lineHeight: 1.6 }}>{movie.description}</p>
                    </div>
                  )}

                  {movie.countries && (
                    <div>
                      <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>国家/地区</label>
                      <div style={{ display: "flex", gap: "8px", flexWrap: "wrap" }}>
                        {movie.countries.split(",").map((country, idx) => (
                          <span
                            key={idx}
                            style={{
                              padding: "2px 8px",
                              backgroundColor: "var(--color-muted)",
                              borderRadius: "4px",
                              fontSize: "12px",
                            }}
                          >
                            {country.trim()}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "files" && (
                <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                  <div>
                    <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>文件名</label>
                    <div style={{ fontFamily: "monospace", fontSize: "12px", wordBreak: "break-all" }}>{movie.filename}</div>
                  </div>

                  <div>
                    <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>完整路径</label>
                    <div style={{ fontFamily: "monospace", fontSize: "12px", wordBreak: "break-all" }}>{movie.path}</div>
                  </div>

                  {movie.file_hash && (
                    <div>
                      <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>文件哈希</label>
                      <div style={{ fontFamily: "monospace", fontSize: "12px" }}>{movie.file_hash}</div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "metadata" && (
                <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                  {movie.imdb_id && (
                    <div>
                      <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>IMDB ID</label>
                      <a
                        href={`https://www.imdb.com/title/${movie.imdb_id}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        style={{ color: "var(--color-primary)" }}
                      >
                        {movie.imdb_id}
                      </a>
                    </div>
                  )}

                  {movie.douban_id && (
                    <div>
                      <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>豆瓣 ID</label>
                      <div>{movie.douban_id}</div>
                    </div>
                  )}

                  <div>
                    <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>数据库 ID</label>
                    <div>{movie.id}</div>
                  </div>

                  <div>
                    <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>创建时间</label>
                    <div>{movie.created_at}</div>
                  </div>

                  <div>
                    <label style={{ fontSize: "12px", color: "var(--color-muted-foreground)", display: "block", marginBottom: "4px" }}>更新时间</label>
                    <div>{movie.updated_at}</div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}