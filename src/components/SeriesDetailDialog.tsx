import { useState, useMemo, useEffect } from "react"
import { convertFileSrc } from "@tauri-apps/api/core"
import { Play, Star, Calendar, Clapperboard, ChevronDown, ChevronRight, Tv } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import type { GroupedSeries } from "@/lib/grouping"
import type { Movie } from "@/lib/api"
import * as api from "@/lib/api"

interface SeriesDetailDialogProps {
  series: GroupedSeries | null
  isOpen: boolean
  onClose: () => void
  onUpdateMovie?: (movieId: number) => void
}

export function SeriesDetailDialog({
  series,
  isOpen,
  onClose,
  onUpdateMovie,
}: SeriesDetailDialogProps) {
  const [expandedSeasons, setExpandedSeasons] = useState<Set<string>>(new Set())
  const [selectedEpisode, setSelectedEpisode] = useState<Movie | null>(null)

  // Group episodes by season
  const episodesBySeason = useMemo(() => {
    if (!series) return new Map<string, Movie[]>()
    const grouped = new Map<string, Movie[]>()
    for (const episode of series.episodes) {
      const season = episode.season || "0"
      if (!grouped.has(season)) {
        grouped.set(season, [])
      }
      grouped.get(season)!.push(episode)
    }
    // Sort episodes within each season
    for (const [, eps] of grouped) {
      eps.sort((a, b) => {
        const epA = parseInt(a.episode || "0")
        const epB = parseInt(b.episode || "0")
        return epA - epB
      })
    }
    return grouped
  }, [series?.episodes])

  // Get sorted season keys
  const seasons = useMemo(() => {
    return Array.from(episodesBySeason.keys()).sort((a, b) => parseInt(a) - parseInt(b))
  }, [episodesBySeason])

  // Auto-expand first season
  useEffect(() => {
    if (seasons.length > 0 && expandedSeasons.size === 0) {
      setExpandedSeasons(new Set([seasons[0]]))
    }
  }, [seasons])

  // Reset selected episode when dialog opens/closes
  useEffect(() => {
    if (!isOpen) {
      setSelectedEpisode(null)
    }
  }, [isOpen])

  if (!series) return null

  const toggleSeason = (season: string) => {
    const newSet = new Set(expandedSeasons)
    if (newSet.has(season)) {
      newSet.delete(season)
    } else {
      newSet.add(season)
    }
    setExpandedSeasons(newSet)
  }

  const getPosterUrl = () => {
    if (series.poster_path) {
      return convertFileSrc(series.poster_path.replace(/\\/g, '/'))
    }
    return null
  }

  const handlePlayEpisode = async (episode: Movie) => {
    try {
      await api.openMovieFile(episode.path)
    } catch (error) {
      console.error("Failed to play episode:", error)
      alert("播放失败: " + error)
    }
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
      <DialogContent style={{ maxWidth: "900px", maxHeight: "90vh", overflow: "auto" }}>
        <DialogHeader>
          <DialogTitle>剧集详情</DialogTitle>
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
                  alt={series.title}
                  style={{ width: "100%", height: "100%", objectFit: "cover" }}
                />
              ) : (
                <div style={{ textAlign: "center", color: "var(--color-muted-foreground)" }}>
                  <Tv style={{ width: "48px", height: "48px", margin: "0 auto 8px" }} />
                  <div style={{ fontSize: "12px" }}>暂无海报</div>
                </div>
              )}
            </div>

            {/* 剧集统计 */}
            <div style={{
              marginTop: "16px",
              padding: "12px",
              backgroundColor: "var(--color-muted)",
              borderRadius: "8px"
            }}>
              <div style={{ fontSize: "14px", fontWeight: 600, marginBottom: "8px" }}>剧集统计</div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "12px", marginBottom: "4px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>总季数</span>
                <span>{series.seasonCount} 季</span>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "12px" }}>
                <span style={{ color: "var(--color-muted-foreground)" }}>总集数</span>
                <span>{series.episodeCount} 集</span>
              </div>
            </div>
          </div>

          {/* 右侧：详情信息 */}
          <div style={{ flex: 1 }}>
            {/* 标题 */}
            <h2 style={{ fontSize: "24px", fontWeight: "bold", marginBottom: "8px" }}>
              {series.title}
            </h2>

            {/* 年份和评分 */}
            <div style={{ display: "flex", gap: "16px", marginBottom: "24px", flexWrap: "wrap" }}>
              {series.year && (
                <div style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                  <Calendar style={{ width: "16px", height: "16px" }} />
                  <span>{series.year}</span>
                </div>
              )}
            </div>

            {/* 分季列表 */}
            <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
              <div style={{ fontSize: "16px", fontWeight: 600, marginBottom: "8px" }}>剧集列表</div>

              {seasons.map((season) => {
                const episodes = episodesBySeason.get(season) || []
                const isExpanded = expandedSeasons.has(season)

                return (
                  <div
                    key={season}
                    style={{
                      border: "1px solid var(--color-border)",
                      borderRadius: "8px",
                      overflow: "hidden"
                    }}
                  >
                    {/* 季标题 */}
                    <div
                      onClick={() => toggleSeason(season)}
                      style={{
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "space-between",
                        padding: "12px 16px",
                        backgroundColor: "var(--color-muted)",
                        cursor: "pointer",
                        userSelect: "none"
                      }}
                    >
                      <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                        {isExpanded ? (
                          <ChevronDown style={{ width: "18px", height: "18px" }} />
                        ) : (
                          <ChevronRight style={{ width: "18px", height: "18px" }} />
                        )}
                        <span style={{ fontWeight: 600 }}>第 {season} 季</span>
                        <span style={{ fontSize: "12px", color: "var(--color-muted-foreground)" }}>
                          ({episodes.length} 集)
                        </span>
                      </div>
                    </div>

                    {/* 剧集列表 */}
                    {isExpanded && (
                      <div style={{ padding: "8px" }}>
                        {episodes.map((episode, index) => (
                          <div
                            key={episode.id}
                            style={{
                              display: "flex",
                              alignItems: "center",
                              gap: "12px",
                              padding: "10px 12px",
                              borderRadius: "6px",
                              marginBottom: "4px",
                              backgroundColor: selectedEpisode?.id === episode.id
                                ? "rgba(59, 130, 246, 0.1)"
                                : index % 2 === 0
                                  ? "var(--color-background)"
                                  : "var(--color-muted)",
                              cursor: "pointer",
                              border: selectedEpisode?.id === episode.id
                                ? "1px solid rgb(59, 130, 246)"
                                : "1px solid transparent"
                            }}
                            onClick={() => setSelectedEpisode(episode)}
                          >
                            {/* 集数 */}
                            <div style={{
                              width: "40px",
                              textAlign: "center",
                              fontWeight: 600,
                              fontSize: "14px",
                              color: "var(--color-muted-foreground)"
                            }}>
                              {episode.episode || "?"}
                            </div>

                            {/* 文件名/标题 */}
                            <div style={{ flex: 1, minWidth: 0 }}>
                              <div style={{
                                fontSize: "14px",
                                overflow: "hidden",
                                textOverflow: "ellipsis",
                                whiteSpace: "nowrap"
                              }}>
                                {episode.cnname || episode.filename}
                              </div>
                              <div style={{
                                fontSize: "11px",
                                color: "var(--color-muted-foreground)",
                                overflow: "hidden",
                                textOverflow: "ellipsis",
                                whiteSpace: "nowrap"
                              }}>
                                {formatFileSize(episode.file_size)} · {episode.filename}
                              </div>
                            </div>

                            {/* 操作按钮 */}
                            <div style={{ display: "flex", gap: "4px" }}>
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={(e) => {
                                  e.stopPropagation()
                                  handlePlayEpisode(episode)
                                }}
                                style={{ padding: "4px 8px" }}
                              >
                                <Play style={{ width: "14px", height: "14px" }} />
                              </Button>

                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={(e) => {
                                  e.stopPropagation()
                                  onUpdateMovie?.(episode.id)
                                }}
                                style={{ padding: "4px 8px" }}
                              >
                                <Clapperboard style={{ width: "14px", height: "14px" }} />
                              </Button>
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                )
              })}
            </div>
          </div>
        </div>

        {/* 选中剧集详情 */}
        {selectedEpisode && (
          <div style={{
            marginTop: "24px",
            padding: "16px",
            backgroundColor: "var(--color-muted)",
            borderRadius: "8px"
          }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "12px" }}>
              <div style={{ fontSize: "14px", fontWeight: 600 }}>
                第 {selectedEpisode.season} 季 第 {selectedEpisode.episode} 集
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setSelectedEpisode(null)}
              >
                关闭
              </Button>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: "12px", fontSize: "12px" }}>
              <div>
                <span style={{ color: "var(--color-muted-foreground)" }}>文件名: </span>
                <span style={{ fontFamily: "monospace" }}>{selectedEpisode.filename}</span>
              </div>
              <div>
                <span style={{ color: "var(--color-muted-foreground)" }}>文件大小: </span>
                <span>{formatFileSize(selectedEpisode.file_size)}</span>
              </div>
              {selectedEpisode.douban_rating && (
                <div>
                  <span style={{ color: "var(--color-muted-foreground)" }}>评分: </span>
                  <span style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                    <Star style={{ width: "12px", height: "12px", color: "#eab308" }} />
                    {selectedEpisode.douban_rating}
                  </span>
                </div>
              )}
              <div>
                <span style={{ color: "var(--color-muted-foreground)" }}>路径: </span>
                <span style={{ fontFamily: "monospace", wordBreak: "break-all" }}>{selectedEpisode.path}</span>
              </div>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  )
}
