import { useEffect, useState, useCallback } from "react"
import {
  Film,
  FolderOpen,
  RefreshCw,
  Search,
  Settings,
  
  Plus,
  
  
  
  
  Check,
  X,
  ExternalLink,
  Info,
  AlertCircle,
  Download,
  Upload,
  Copy,
  Server,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { useDirectoryStore } from "@/stores/directoryStore"
import { useMovieStore } from "@/stores/movieStore"
import { MovieDetailDialog } from "@/components/MovieDetailDialog"
import { DuplicateManager } from "@/components/DuplicateManager"
import { PTDepilerSettings } from "@/components/PTDepilerSettings"
import { QBittorrentSettings } from "@/components/QBittorrentSettings"
import { AIAssistant, AIAssistantButton } from "@/components/AIAssistant"
import { ThemeSwitcher } from "@/components/ThemeSwitcher"
import { LLMSettings } from "@/components/LLMSettings"
import type { Movie } from "@/lib/api"
import {
  MovieCard,
  DirectoryCard,
  SettingItem,
  EmptyState,
  NavButton,
  StatItem,
  FilterButton,
  TMDBApiKeySetting
} from "@/components/dashboard"

export default function Dashboard() {
  const { directories, fetchDirectories, addDirectory, removeDirectory, toggleDirectory } = useDirectoryStore()
  const { 
    movies, 
    fetchMovies, 
    scanDirectory, 
    loading, 
    searchQuery, 
    setSearchQuery, 
    setVideoType,
    searchTMDB,
    fetchTMDBDetail,
    downloadTMDBPoster,
    updateMovieFromTMDB,
    smartUpdateRelatedMovies,
  } = useMovieStore()
  
  const [activeTab, setActiveTab] = useState("movies")
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false)
  const [newPath, setNewPath] = useState("")
  const [newName, setNewName] = useState("")
  const [videoFilter, setVideoFilter] = useState<"all" | "movie" | "tv">("all")
  const [selectedMovie, setSelectedMovie] = useState<Movie | null>(null)
  const [isDetailOpen, setIsDetailOpen] = useState(false)
  const [selectedMovies, setSelectedMovies] = useState<Set<number>>(new Set())
  const [isBatchMode, setIsBatchMode] = useState(false)
  const [sortBy, setSortBy] = useState<"default" | "name" | "year" | "rating" | "created" | "updated" | "size">("updated")
  const [smartUpdateEnabled, setSmartUpdateEnabled] = useState(true)
  const [isDuplicateDialogOpen, setIsDuplicateDialogOpen] = useState(false)
  const [isPTDepilerDialogOpen, setIsPTDepilerDialogOpen] = useState(false)
  const [isQBDialogOpen, setIsQBDialogOpen] = useState(false)
  const [isAIAssistantOpen, setIsAIAssistantOpen] = useState(false)

  // Toast notification state
  const [toast, setToast] = useState<{
    show: boolean
    message: string
    type: "success" | "error" | "info"
  }>({ show: false, message: "", type: "info" })

  // Auto-hide toast after 3 seconds
  useEffect(() => {
    if (toast.show) {
      const timer = setTimeout(() => {
        setToast(prev => ({ ...prev, show: false }))
      }, 3000)
      return () => clearTimeout(timer)
    }
  }, [toast.show, toast.message])

  const showToast = useCallback((message: string, type: "success" | "error" | "info" = "info") => {
    setToast({ show: true, message, type })
  }, [])

  useEffect(() => {
    fetchDirectories()
    fetchMovies()
  }, [])

  const handleAddDirectory = async () => {
    if (!newPath) return
    try {
      await addDirectory(newPath, newName || undefined)
      setNewPath("")
      setNewName("")
      setIsAddDialogOpen(false)
    } catch (error) {
      console.error("Failed to add directory:", error)
    }
  }

  const handleScan = async (dirId: number) => {
    console.log("Scan button clicked for directory:", dirId)
    try {
      const count = await scanDirectory(dirId)
      console.log("Scan completed, found", count, "movies")
      alert(`扫描完成，新增 ${count} 个影片`)
    } catch (error) {
      console.error("Scan failed:", error)
      alert("扫描失败: " + error)
    }
  }

  const handleFilterChange = (type: "all" | "movie" | "tv") => {
    setVideoFilter(type)
    setVideoType(type === "all" ? null : type)
  }

  const filteredMovies = movies
    .filter((movie) => {
      if (videoFilter !== "all" && movie.video_type !== videoFilter) return false
      if (searchQuery) {
        const query = searchQuery.toLowerCase()
        return (
          movie.filename?.toLowerCase().includes(query) ||
          movie.cnname?.toLowerCase().includes(query)
        )
      }
      return true
    })
    .sort((a, b) => {
      switch (sortBy) {
        case "name":
          return (a.cnname || a.filename).localeCompare(b.cnname || b.filename)
        case "year":
          return (parseInt(b.year || "0") || 0) - (parseInt(a.year || "0") || 0)
        case "rating":
          return (b.douban_rating || 0) - (a.douban_rating || 0)
        case "created":
          return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        case "updated":
          return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
        case "size":
          return (b.file_size || 0) - (a.file_size || 0)
        default:
          return 0
      }
    })

  const stats = {
    total: movies.length,
    movies: movies.filter((m) => m.video_type === "movie").length,
    tv: movies.filter((m) => m.video_type === "tv").length,
  }

  return (
    <div style={{ display: "flex", height: "100vh", backgroundColor: "var(--color-background)" }}>
      {/* Toast Notification */}
      {toast.show && (
        <div
          style={{
            position: "fixed",
            top: "20px",
            right: "20px",
            zIndex: 100,
            padding: "12px 16px",
            borderRadius: "8px",
            display: "flex",
            alignItems: "center",
            gap: "8px",
            boxShadow: "0 4px 12px rgba(0, 0, 0, 0.15)",
            animation: "slideIn 0.3s ease-out",
            backgroundColor:
              toast.type === "success"
                ? "rgb(34, 197, 94)"
                : toast.type === "error"
                ? "rgb(239, 68, 68)"
                : "rgb(59, 130, 246)",
            color: "white",
          }}
        >
          {toast.type === "success" ? (
            <Check style={{ width: "18px", height: "18px" }} />
          ) : toast.type === "error" ? (
            <AlertCircle style={{ width: "18px", height: "18px" }} />
          ) : (
            <Info style={{ width: "18px", height: "18px" }} />
          )}
          <span style={{ fontSize: "14px", fontWeight: 500 }}>{toast.message}</span>
          <button
            onClick={() => setToast(prev => ({ ...prev, show: false }))}
            style={{
              marginLeft: "8px",
              padding: "2px",
              background: "transparent",
              border: "none",
              cursor: "pointer",
              color: "white",
              opacity: 0.8,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <X style={{ width: "16px", height: "16px" }} />
          </button>
        </div>
      )}

      <style>{`
        @keyframes slideIn {
          from {
            transform: translateX(100%);
            opacity: 0;
          }
          to {
            transform: translateX(0);
            opacity: 1;
          }
        }
      `}</style>

      {/* Sidebar */}
      <aside style={{ 
        width: "256px", 
        borderRight: "1px solid var(--color-border)", 
        backgroundColor: "var(--color-card)",
        display: "flex",
        flexDirection: "column"
      }}>
        {/* Logo */}
        <div style={{ padding: "24px", borderBottom: "1px solid var(--color-border)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
            <div style={{ 
              width: "40px", 
              height: "40px", 
              borderRadius: "12px",
              background: "linear-gradient(135deg, var(--color-primary) 0%, #6347eb 100%)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center"
            }}>
              <Film style={{ width: "20px", height: "20px", color: "white" }} />
            </div>
            <div>
              <h1 style={{ fontSize: "18px", fontWeight: "bold", color: "var(--color-foreground)" }}>影视管家</h1>
              <p style={{ fontSize: "12px", color: "var(--color-muted-foreground)" }}>MovieMaster</p>
            </div>
          </div>
        </div>

        {/* Navigation */}
        <nav style={{ flex: 1, padding: "16px", display: "flex", flexDirection: "column", gap: "4px" }}>
          <NavButton
            icon={<Film style={{ width: "20px", height: "20px" }} />}
            label="媒体库"
            active={activeTab === "movies"}
            onClick={() => setActiveTab("movies")}
          />
          <NavButton
            icon={<FolderOpen style={{ width: "20px", height: "20px" }} />}
            label="目录管理"
            active={activeTab === "directories"}
            onClick={() => setActiveTab("directories")}
          />
          <NavButton
            icon={<Settings style={{ width: "20px", height: "20px" }} />}
            label="设置"
            active={activeTab === "settings"}
            onClick={() => setActiveTab("settings")}
          />
          <NavButton
            icon={<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/><circle cx="12" cy="12" r="5"/></svg>}
            label="AI 助手"
            active={isAIAssistantOpen}
            onClick={() => setIsAIAssistantOpen(!isAIAssistantOpen)}
            variant="orange"
          />
        </nav>

        {/* Stats */}
        <div style={{ 
          padding: "16px", 
          borderTop: "1px solid var(--color-border)",
          backgroundColor: "var(--color-muted)"
        }}>
          <h3 style={{ 
            fontSize: "11px", 
            fontWeight: 600, 
            color: "var(--color-muted-foreground)", 
            textTransform: "uppercase",
            letterSpacing: "0.5px",
            marginBottom: "12px"
          }}>统计信息</h3>
          <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
            <StatItem label="影片总数" value={stats.total} />
            <StatItem label="电影" value={stats.movies} />
            <StatItem label="电视剧" value={stats.tv} />
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main style={{ flex: 1, display: "flex", flexDirection: "column", overflow: "hidden" }}>
        {/* Header */}
        <header style={{ 
          height: "64px", 
          borderBottom: "1px solid var(--color-border)",
          backgroundColor: "var(--color-card)",
          display: "flex",
          alignItems: "center",
          padding: "0 24px",
          gap: "16px"
        }}>
          <div style={{ position: "relative", flex: 1, maxWidth: "400px" }}>
            <Search style={{ 
              position: "absolute", 
              left: "12px", 
              top: "50%", 
              transform: "translateY(-50%)",
              width: "16px", 
              height: "16px", 
              color: "var(--color-muted-foreground)" 
            }} />
            <Input
              placeholder="搜索影片..."
              style={{ paddingLeft: "40px", height: "40px" }}
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>

          {/* 排序下拉框 */}
          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as any)}
            style={{
              height: "40px",
              padding: "0 12px",
              borderRadius: "6px",
              border: "1px solid var(--color-border)",
              backgroundColor: "var(--color-background)",
              fontSize: "14px",
              color: "var(--color-foreground)",
              cursor: "pointer"
            }}
          >
            <option value="default">默认排序</option>
            <option value="name">名称</option>
            <option value="year">年份</option>
            <option value="rating">评分</option>
            <option value="created">添加时间</option>
            <option value="updated">更新时间</option>
            <option value="size">文件大小</option>
          </select>

          <div style={{
            display: "flex",
            gap: "4px",
            backgroundColor: "var(--color-muted)",
            padding: "4px",
            borderRadius: "8px"
          }}>
            <FilterButton active={videoFilter === "all"} onClick={() => handleFilterChange("all")}>
              全部
            </FilterButton>
            <FilterButton active={videoFilter === "movie"} onClick={() => handleFilterChange("movie")}>
              电影
            </FilterButton>
            <FilterButton active={videoFilter === "tv"} onClick={() => handleFilterChange("tv")}>
              电视剧
            </FilterButton>
          </div>

          <Button
            variant="outline"
            size="icon"
            onClick={() => fetchMovies()}
            style={loading ? { animation: "spin 1s linear infinite" } : {}}
          >
            <RefreshCw style={{ width: "16px", height: "16px" }} />
          </Button>

          {/* 主题切换器 */}
          <ThemeSwitcher />
        </header>

        {/* Content */}
        <div style={{ flex: 1, overflow: "auto", padding: "24px" }}>
          {activeTab === "movies" && (
            <>
              {/* 批量操作工具栏 */}
              {isBatchMode && (
                <div style={{
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                  padding: "12px 16px",
                  backgroundColor: "var(--color-muted)",
                  borderRadius: "8px",
                  marginBottom: "16px"
                }}>
                  <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                    <input
                      type="checkbox"
                      checked={selectedMovies.size === filteredMovies.length && filteredMovies.length > 0}
                      onChange={(e) => {
                        if (e.target.checked) {
                          setSelectedMovies(new Set(filteredMovies.map(m => m.id)))
                        } else {
                          setSelectedMovies(new Set())
                        }
                      }}
                    />
                    <span>已选择 {selectedMovies.size} 个影片</span>
                  </div>
                  <div style={{ display: "flex", gap: "8px" }}>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={async () => {
                        if (selectedMovies.size === 0) {
                          alert("请先选择影片")
                          return
                        }
                        let success = 0
                        let smartUpdated = 0
                        for (const movieId of selectedMovies) {
                          const movie = movies.find(m => m.id === movieId)
                          if (movie) {
                            try {
                              const title = movie.cnname || movie.filename
                              const year = movie.year ? parseInt(movie.year) : undefined
                              const results = await searchTMDB(title, year, movie.video_type)
                              if (results.length > 0) {
                                const detail = await fetchTMDBDetail(results[0].id, movie.video_type)
                                await updateMovieFromTMDB(movie.id, detail)
                                if (detail.poster_url) {
                                  await downloadTMDBPoster(movie.id, detail.poster_url)
                                }
                                // 智能更新相关影片
                                if (smartUpdateEnabled) {
                                  try {
                                    const relatedResults = await smartUpdateRelatedMovies(movie.id, detail)
                                    smartUpdated += relatedResults.length
                                  } catch (e) {
                                    console.error("Smart update failed for movie", movie.id, e)
                                  }
                                }
                                success++
                              }
                            } catch (error) {
                              console.error("批量更新失败:", error)
                            }
                          }
                        }
                        let message = `批量更新完成: ${success}/${selectedMovies.size} 个影片`
                        if (smartUpdated > 0) {
                          message += ` (连带更新 ${smartUpdated} 个相关影片)`
                        }
                        showToast(message, "success")
                        setSelectedMovies(new Set())
                      }}
                    >
                      批量更新信息
                    </Button>
                    <Button variant="outline" size="sm" onClick={() => setIsBatchMode(false)}>
                      退出批量模式
                    </Button>
                  </div>
                </div>
              )}

              {!isBatchMode && filteredMovies.length > 0 && (
                <div style={{ marginBottom: "16px" }}>
                  <Button variant="outline" size="sm" onClick={() => setIsBatchMode(true)}>
                    进入批量模式
                  </Button>
                </div>
              )}

              {filteredMovies.length > 0 ? (
                <div style={{ 
                  display: "grid", 
                  gridTemplateColumns: "repeat(auto-fill, minmax(180px, 1fr))",
                  gap: "20px"
                }}>
                  {filteredMovies.map((movie) => (
                    <MovieCard 
                      key={movie.id} 
                      movie={movie}
                      isSelected={selectedMovies.has(movie.id)}
                      isBatchMode={isBatchMode}
                      onToggleSelect={() => {
                        const newSet = new Set(selectedMovies)
                        if (newSet.has(movie.id)) {
                          newSet.delete(movie.id)
                        } else {
                          newSet.add(movie.id)
                        }
                        setSelectedMovies(newSet)
                      }}
                      onClick={() => {
                        setSelectedMovie(movie)
                        setIsDetailOpen(true)
                      }}
                      onSearchTMDB={async () => {
                        try {
                          const title = movie.cnname || movie.filename
                          const year = movie.year ? parseInt(movie.year) : undefined
                          const results = await searchTMDB(title, year, movie.video_type)

                          if (results.length > 0) {
                            const bestMatch = results[0]
                            const detail = await fetchTMDBDetail(bestMatch.id, movie.video_type)
                            await updateMovieFromTMDB(movie.id, detail)

                            if (detail.poster_url) {
                              await downloadTMDBPoster(movie.id, detail.poster_url)
                            }

                            // 智能更新相关影片
                            let updatedCount = 0
                            if (smartUpdateEnabled) {
                              try {
                                const relatedResults = await smartUpdateRelatedMovies(movie.id, detail)
                                updatedCount = relatedResults.length
                              } catch (e) {
                                console.error("Smart update failed:", e)
                              }
                            }

                            const message = updatedCount > 0
                              ? `已更新: ${detail.cn_title || detail.title} (连带更新 ${updatedCount} 个相关影片)`
                              : `已更新: ${detail.cn_title || detail.title}`
                            showToast(message, "success")
                          } else {
                            showToast("未找到匹配的影片", "error")
                          }
                        } catch (error) {
                          showToast("获取 TMDB 信息失败", "error")
                        }
                      }}
                    />
                  ))}
                </div>
              ) : (
                <EmptyState
                  icon={<Film style={{ width: "64px", height: "64px" }} />}
                  title="暂无影片"
                  description={searchQuery ? "没有找到匹配的影片" : "请先添加目录并扫描"}
                />
              )}
            </>
          )}

          {activeTab === "directories" && (
            <div style={{ maxWidth: "800px", margin: "0 auto" }}>
              <div style={{ 
                display: "flex", 
                justifyContent: "space-between", 
                alignItems: "center",
                marginBottom: "24px"
              }}>
                <div>
                  <h2 style={{ fontSize: "24px", fontWeight: "bold", color: "var(--color-foreground)" }}>影视目录</h2>
                  <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)", marginTop: "4px" }}>管理您的影视文件夹</p>
                </div>
                <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
                  <DialogTrigger asChild>
                    <Button style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                      <Plus style={{ width: "16px", height: "16px" }} />
                      添加目录
                    </Button>
                  </DialogTrigger>
                  <DialogContent style={{ maxWidth: "450px" }}>
                    <DialogHeader>
                      <DialogTitle>添加影视目录</DialogTitle>
                      <DialogDescription>添加包含视频文件的目录路径</DialogDescription>
                    </DialogHeader>
                    <div style={{ display: "flex", flexDirection: "column", gap: "16px", padding: "16px 0" }}>
                      <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
                        <label style={{ fontSize: "14px", fontWeight: 500 }}>显示名称</label>
                        <Input placeholder="我的电影" value={newName} onChange={(e) => setNewName(e.target.value)} />
                      </div>
                      <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
                        <label style={{ fontSize: "14px", fontWeight: 500 }}>目录路径</label>
                        <Input placeholder="D:\Movies" value={newPath} onChange={(e) => setNewPath(e.target.value)} />
                      </div>
                    </div>
                    <DialogFooter>
                      <Button variant="outline" onClick={() => setIsAddDialogOpen(false)}>取消</Button>
                      <Button onClick={handleAddDirectory}>添加</Button>
                    </DialogFooter>
                  </DialogContent>
                </Dialog>
              </div>

              <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                {directories.map((dir) => (
                  <DirectoryCard
                    key={dir.id}
                    dir={dir}
                    onScan={() => handleScan(dir.id)}
                    onToggle={() => toggleDirectory(dir.id, !dir.enabled)}
                    onDelete={() => removeDirectory(dir.id)}
                    loading={loading}
                  />
                ))}
                {directories.length === 0 && (
                  <EmptyState
                    icon={<FolderOpen style={{ width: "64px", height: "64px" }} />}
                    title="暂无目录"
                    description="点击上方按钮添加影视目录"
                  />
                )}
              </div>
            </div>
          )}

          {activeTab === "settings" && (
            <div style={{ maxWidth: "600px", margin: "0 auto" }}>
              <div style={{ marginBottom: "24px" }}>
                <h2 style={{ fontSize: "24px", fontWeight: "bold", color: "var(--color-foreground)" }}>设置</h2>
                <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)", marginTop: "4px" }}>配置应用偏好</p>
              </div>

              <Card>
                <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                  <SettingItem label="NFO存储策略" description="选择NFO文件的存储位置">
                    <select style={{ 
                      width: "100%", 
                      height: "40px", 
                      padding: "0 12px",
                      borderRadius: "6px",
                      border: "1px solid var(--color-border)",
                      backgroundColor: "var(--color-background)"
                    }}>
                      <option value="same">与视频文件同目录</option>
                      <option value="cache">本地缓存</option>
                      <option value="mixed">混合模式</option>
                    </select>
                  </SettingItem>

                  <div style={{ borderTop: "1px solid var(--color-border)" }} />

                  <SettingItem label="海报图片源" description="选择获取海报图片的数据源">
                    <select style={{ 
                      width: "100%", 
                      height: "40px", 
                      padding: "0 12px",
                      borderRadius: "6px",
                      border: "1px solid var(--color-border)",
                      backgroundColor: "var(--color-background)"
                    }}>
                      <option value="douban">豆瓣</option>
                      <option value="tmdb">TMDB</option>
                    </select>
                  </SettingItem>

                  <div style={{ borderTop: "1px solid var(--color-border)" }} />

                  <SettingItem label="字幕网站" description="选择首选的字幕下载源">
                    <select style={{ 
                      width: "100%", 
                      height: "40px", 
                      padding: "0 12px",
                      borderRadius: "6px",
                      border: "1px solid var(--color-border)",
                      backgroundColor: "var(--color-background)"
                    }}>
                      <option value="subhd">字幕库</option>
                      <option value="opensubtitles">OpenSubtitles</option>
                    </select>
                  </SettingItem>
                </CardContent>
              </Card>

              {/* qBittorrent 配置 */}
              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        qBittorrent 远程下载
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        配置远程 qBittorrent 服务器 (http://10.40.31.69:8044)
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <Button
                      variant="outline"
                      onClick={() => setIsQBDialogOpen(true)}
                      style={{ display: "flex", alignItems: "center", gap: "8px", width: "fit-content" }}
                    >
                      <Server style={{ width: "16px", height: "16px" }} />
                      配置远程下载
                    </Button>
                  </CardContent>
                </Card>
              </div>

              {/* PT-Depiler 配置 */}
              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        PT-Depiler 配置
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        配置 PT 站点信息，通过浏览器插件搜索下载
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <Button
                      variant="outline"
                      onClick={() => setIsPTDepilerDialogOpen(true)}
                      style={{ display: "flex", alignItems: "center", gap: "8px", width: "fit-content" }}
                    >
                      <ExternalLink style={{ width: "16px", height: "16px" }} />
                      配置 PT 站点
                    </Button>
                  </CardContent>
                </Card>
              </div>

              {/* LLM 配置 */}
              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        AI 模型配置
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        配置本地 LLM 服务以启用 AI 助手功能
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <LLMSettings />
                  </CardContent>
                </Card>
              </div>

              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        API 配置
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        配置第三方 API 密钥以获取影片信息
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <TMDBApiKeySetting />

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    {/* 智能更新选项 */}
                    <div>
                      <label style={{ fontSize: "14px", fontWeight: 500, color: "var(--color-foreground)", display: "block", marginBottom: "8px" }}>
                        智能更新
                      </label>
                      <label style={{ display: "flex", alignItems: "center", gap: "8px", cursor: "pointer" }}>
                        <input
                          type="checkbox"
                          checked={smartUpdateEnabled}
                          onChange={(e) => setSmartUpdateEnabled(e.target.checked)}
                          style={{ width: "16px", height: "16px" }}
                        />
                        <span style={{ fontSize: "14px" }}>
                          更新影片时同时更新同名其他季/版本
                        </span>
                      </label>
                      <p style={{ fontSize: "12px", color: "var(--color-muted-foreground)", marginTop: "4px", marginLeft: "24px" }}>
                        当为一个电视剧或电影更新 TMDB 信息时，自动查找并更新库中同名但不同季/集或不同版本的影片
                      </p>
                    </div>
                  </CardContent>
                </Card>
              </div>

              {/* 重复文件清理 */}
              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        重复文件清理
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        扫描并清理媒体库中的重复影片文件
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <Button
                      variant="outline"
                      onClick={() => setIsDuplicateDialogOpen(true)}
                      style={{ display: "flex", alignItems: "center", gap: "8px", width: "fit-content" }}
                    >
                      <Copy style={{ width: "16px", height: "16px" }} />
                      扫描重复文件
                    </Button>
                  </CardContent>
                </Card>
              </div>

              {/* 配置导入导出 */}
              <div style={{ marginTop: "24px" }}>
                <Card>
                  <CardContent style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "24px" }}>
                    <div>
                      <h3 style={{ fontSize: "18px", fontWeight: 600, color: "var(--color-foreground)", marginBottom: "4px" }}>
                        配置管理
                      </h3>
                      <p style={{ fontSize: "14px", color: "var(--color-muted-foreground)" }}>
                        导出或导入应用配置，方便备份和迁移
                      </p>
                    </div>

                    <div style={{ borderTop: "1px solid var(--color-border)" }} />

                    <div style={{ display: "flex", gap: "12px" }}>
                      <Button
                        variant="outline"
                        onClick={async () => {
                          try {
                            const path = await import("@/lib/api").then(m => m.exportConfig());
                            showToast(`配置已导出到: ${path}`, "success");
                          } catch (error) {
                            if (String(error) !== "User cancelled") {
                              showToast("导出失败: " + error, "error");
                            }
                          }
                        }}
                        style={{ display: "flex", alignItems: "center", gap: "8px" }}
                      >
                        <Download style={{ width: "16px", height: "16px" }} />
                        导出配置
                      </Button>

                      <Button
                        variant="outline"
                        onClick={async () => {
                          try {
                            const result = await import("@/lib/api").then(m => m.importConfig());
                            showToast(`配置导入成功: ${result.settings_count} 个设置, ${result.directories_count} 个目录`, "success");
                            // 刷新目录列表
                            fetchDirectories();
                          } catch (error) {
                            if (String(error) !== "User cancelled") {
                              showToast("导入失败: " + error, "error");
                            }
                          }
                        }}
                        style={{ display: "flex", alignItems: "center", gap: "8px" }}
                      >
                        <Upload style={{ width: "16px", height: "16px" }} />
                        导入配置
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              </div>
            </div>
          )}
        </div>
      </main>

      {/* 影片详情对话框 */}
      <MovieDetailDialog
        movie={selectedMovie}
        isOpen={isDetailOpen}
        onClose={() => setIsDetailOpen(false)}
        onPlay={async () => {
          if (selectedMovie) {
            try {
              await import("@/lib/api").then(m => m.openMovieFile(selectedMovie.path))
            } catch (error) {
              alert("无法播放影片: " + error)
            }
          }
        }}
        onGenerateNfo={async () => {
          if (selectedMovie) {
            try {
              await import("@/lib/api").then(m => m.generateNfo(selectedMovie.id))
              alert("NFO文件已生成")
            } catch (error) {
              alert("生成NFO失败: " + error)
            }
          }
        }}
        onUpdateFromTMDB={async () => {
          if (selectedMovie) {
            try {
              const title = selectedMovie.cnname || selectedMovie.filename
              const year = selectedMovie.year ? parseInt(selectedMovie.year) : undefined
              const results = await searchTMDB(title, year, selectedMovie.video_type)

              if (results.length > 0) {
                const detail = await fetchTMDBDetail(results[0].id, selectedMovie.video_type)
                await updateMovieFromTMDB(selectedMovie.id, detail)

                if (detail.poster_url) {
                  await downloadTMDBPoster(selectedMovie.id, detail.poster_url)
                }

                // 智能更新相关影片
                let updatedCount = 0
                if (smartUpdateEnabled) {
                  try {
                    const relatedResults = await smartUpdateRelatedMovies(selectedMovie.id, detail)
                    updatedCount = relatedResults.length
                  } catch (e) {
                    console.error("Smart update failed:", e)
                  }
                }

                // 刷新影片数据
                await fetchMovies()
                const message = updatedCount > 0
                  ? `已更新: ${detail.cn_title || detail.title} (连带更新 ${updatedCount} 个相关影片)`
                  : `已更新: ${detail.cn_title || detail.title}`
                showToast(message, "success")
              } else {
                showToast("未找到匹配的影片", "error")
              }
            } catch (error) {
              showToast("更新失败", "error")
            }
          }
        }}
        onDelete={async () => {
          if (selectedMovie && confirm("确定要删除这个影片记录吗？")) {
            try {
              await import("@/lib/api").then(m => m.deleteMovie(selectedMovie.id))
              await fetchMovies()
              setIsDetailOpen(false)
            } catch (error) {
              alert("删除失败: " + error)
            }
          }
        }}
      />

      {/* 重复文件管理对话框 */}
      <Dialog open={isDuplicateDialogOpen} onOpenChange={setIsDuplicateDialogOpen}>
        <DialogContent style={{ maxWidth: "900px", maxHeight: "90vh" }}>
          <DialogHeader>
            <DialogTitle>重复文件管理</DialogTitle>
            <DialogDescription>扫描并清理媒体库中的重复影片</DialogDescription>
          </DialogHeader>
          <DuplicateManager onClose={() => setIsDuplicateDialogOpen(false)} />
        </DialogContent>
      </Dialog>

      {/* PT-Depiler 配置对话框 */}
      <Dialog open={isPTDepilerDialogOpen} onOpenChange={setIsPTDepilerDialogOpen}>
        <DialogContent style={{ maxWidth: "700px", maxHeight: "90vh" }}>
          <DialogHeader>
            <DialogTitle>PT-Depiler 配置</DialogTitle>
            <DialogDescription>配置 PT 站点信息</DialogDescription>
          </DialogHeader>
          <PTDepilerSettings onClose={() => setIsPTDepilerDialogOpen(false)} />
        </DialogContent>
      </Dialog>

      {/* qBittorrent 配置对话框 */}
      <Dialog open={isQBDialogOpen} onOpenChange={setIsQBDialogOpen}>
        <DialogContent style={{ maxWidth: "600px", maxHeight: "90vh" }}>
          <DialogHeader>
            <DialogTitle>qBittorrent 配置</DialogTitle>
            <DialogDescription>配置远程 qBittorrent 服务器</DialogDescription>
          </DialogHeader>
          <QBittorrentSettings onClose={() => setIsQBDialogOpen(false)} />
        </DialogContent>
      </Dialog>

      {/* AI 助手 */}
      {isAIAssistantOpen ? (
        <AIAssistant onClose={() => setIsAIAssistantOpen(false)} />
      ) : (
        <AIAssistantButton onClick={() => setIsAIAssistantOpen(true)} />
      )}
    </div>
  )
}

