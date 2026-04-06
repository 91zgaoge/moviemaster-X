import { create } from "zustand"
import * as api from "@/lib/api"
import { groupMoviesBySeries, removeDuplicateMovies, type GroupedSeries } from "@/lib/grouping"

interface MovieState {
  movies: api.Movie[]
  groupedSeries: GroupedSeries[]
  individualMovies: api.Movie[]
  loading: boolean
  error: string | null
  selectedDirectory: number | null
  searchQuery: string
  videoType: string | null
  tmdbSearchResults: api.TMDBSearchResult[]
  tmdbLoading: boolean
  groupBySeries: boolean
  // Cache for performance
  lastFetchTime: number
  isFetching: boolean
  // Scan status
  scanningDirectories: Set<number>
  scanProgress: Record<number, api.ScanProgress>
  lastScanResults: Record<number, { totalFound: number; timestamp: number }>

  fetchMovies: (force?: boolean) => Promise<void>
  scanDirectory: (directoryId: number) => Promise<number>
  startBackgroundScan: (directoryId: number) => Promise<void>
  fetchScanStatus: (directoryId: number) => Promise<void>
  isDirectoryScanning: (directoryId: number) => boolean
  setSelectedDirectory: (id: number | null) => void
  setSearchQuery: (query: string) => void
  setVideoType: (type: string | null) => void
  setGroupBySeries: (group: boolean) => void
  refreshGrouping: () => void
  // TMDB actions
  searchTMDB: (title: string, year?: number, videoType?: string) => Promise<api.TMDBSearchResult[]>
  fetchTMDBDetail: (tmdbId: number, videoType: string) => Promise<api.TMDBMovieDetail>
  downloadTMDBPoster: (movieId: number, posterUrl: string) => Promise<string>
  updateMovieFromTMDB: (movieId: number, tmdbDetail: api.TMDBMovieDetail) => Promise<void>
  smartUpdateRelatedMovies: (sourceMovieId: number, tmdbDetail: api.TMDBMovieDetail) => Promise<api.SmartUpdateResult[]>
}

export const useMovieStore = create<MovieState>((set, get) => ({
  movies: [],
  groupedSeries: [],
  individualMovies: [],
  loading: false,
  error: null,
  selectedDirectory: null,
  searchQuery: "",
  videoType: null,
  tmdbSearchResults: [],
  tmdbLoading: false,
  groupBySeries: true,
  lastFetchTime: 0,
  isFetching: false,
  scanningDirectories: new Set(),
  scanProgress: {},
  lastScanResults: {},

  fetchMovies: async (force = false) => {
    const { isFetching, lastFetchTime, selectedDirectory, searchQuery, videoType } = get()

    // Prevent concurrent fetches
    if (isFetching) {
      console.log("[MovieStore] Fetch already in progress, skipping")
      return
    }

    // Cache for 5 seconds unless force refresh
    const now = Date.now()
    if (!force && now - lastFetchTime < 5000) {
      console.log("[MovieStore] Using cached data")
      return
    }

    set({ isFetching: true, loading: true, error: null })

    try {
      console.log("[MovieStore] Fetching movies...")
      const startTime = performance.now()

      let movies = await api.getMovies({
        directory_id: selectedDirectory || undefined,
        video_type: videoType || undefined,
        search: searchQuery || undefined,
      })

      const fetchTime = performance.now() - startTime
      console.log(`[MovieStore] Fetched ${movies.length} movies in ${fetchTime.toFixed(0)}ms`)

      // Process in chunks to avoid blocking UI
      const processStart = performance.now()

      // Remove duplicates by movie identity
      movies = removeDuplicateMovies(movies)
      console.log(`[MovieStore] After dedupe: ${movies.length} movies`)

      // Group TV series
      const { series, individualMovies } = groupMoviesBySeries(movies)
      console.log(`[MovieStore] Grouped: ${series.length} series, ${individualMovies.length} movies`)

      const processTime = performance.now() - processStart
      console.log(`[MovieStore] Processing took ${processTime.toFixed(0)}ms`)

      set({
        movies,
        groupedSeries: series,
        individualMovies,
        loading: false,
        isFetching: false,
        lastFetchTime: Date.now()
      })
    } catch (error) {
      console.error("[MovieStore] Fetch failed:", error)
      set({ error: String(error), loading: false, isFetching: false })
    }
  },

  scanDirectory: async (directoryId: number) => {
    set({ loading: true, error: null })
    try {
      const count = await api.scanDirectory(directoryId)
      await get().fetchMovies()
      set({ loading: false })
      return count
    } catch (error) {
      set({ error: String(error), loading: false })
      throw error
    }
  },

  startBackgroundScan: async (directoryId: number) => {
    console.log("[MovieStore] startBackgroundScan called with:", directoryId)
    try {
      await api.startScan(directoryId)
      const scanningDirs = new Set(get().scanningDirectories)
      scanningDirs.add(directoryId)
      set({ scanningDirectories: scanningDirs })

      // Start polling scan status
      const pollInterval = setInterval(async () => {
        try {
          const status = await api.getScanStatus(directoryId)
          const state = get()

          if (status.status === "scanning") {
            set({
              scanProgress: {
                ...state.scanProgress,
                [directoryId]: status
              }
            })
          } else if (status.status === "completed") {
            clearInterval(pollInterval)
            const scanningDirs = new Set(state.scanningDirectories)
            scanningDirs.delete(directoryId)
            set({
              scanningDirectories: scanningDirs,
              lastScanResults: {
                ...state.lastScanResults,
                [directoryId]: {
                  totalFound: status.total_found || 0,
                  timestamp: Date.now()
                }
              }
            })
            // Refresh movies after scan completes
            await get().fetchMovies()
            // Clear status after 5 seconds
            setTimeout(() => {
              api.clearScanStatus(directoryId)
            }, 5000)
          } else if (status.status === "error") {
            clearInterval(pollInterval)
            const scanningDirs = new Set(state.scanningDirectories)
            scanningDirs.delete(directoryId)
            set({
              scanningDirectories: scanningDirs,
              error: `扫描失败: ${status.message}`
            })
          }
        } catch (e) {
          console.error("Failed to fetch scan status:", e)
        }
      }, 500) // Poll every 500ms

      // Stop polling after 5 minutes (timeout)
      setTimeout(() => {
        clearInterval(pollInterval)
        const scanningDirs = new Set(get().scanningDirectories)
        if (scanningDirs.has(directoryId)) {
          scanningDirs.delete(directoryId)
          set({ scanningDirectories: scanningDirs })
        }
      }, 5 * 60 * 1000)

    } catch (error) {
      set({ error: String(error) })
      throw error
    }
  },

  fetchScanStatus: async (directoryId: number) => {
    try {
      const status = await api.getScanStatus(directoryId)
      if (status.status === "scanning") {
        set({
          scanProgress: {
            ...get().scanProgress,
            [directoryId]: status
          }
        })
      }
    } catch (error) {
      console.error("Failed to fetch scan status:", error)
    }
  },

  isDirectoryScanning: (directoryId: number) => {
    return get().scanningDirectories.has(directoryId)
  },
  
  setSelectedDirectory: (id) => {
    set({ selectedDirectory: id })
    get().fetchMovies()
  },
  
  setSearchQuery: (query) => {
    set({ searchQuery: query })
    get().fetchMovies()
  },
  
  setVideoType: (type) => {
    set({ videoType: type })
    get().fetchMovies()
  },

  setGroupBySeries: (group) => {
    set({ groupBySeries: group })
  },

  refreshGrouping: () => {
    const { movies } = get()
    const uniqueMovies = removeDuplicateMovies(movies)
    const { series, individualMovies } = groupMoviesBySeries(uniqueMovies)
    set({ movies: uniqueMovies, groupedSeries: series, individualMovies })
  },

  // TMDB actions
  searchTMDB: async (title, year, videoType = "movie") => {
    set({ tmdbLoading: true })
    try {
      // 从数据库获取API key
      const settings = await api.getSettings()
      const tmdbKey = settings.find(s => s.key === "tmdb_api_key")?.value
      
      const results = await api.searchTMDB(tmdbKey || "", title, year, videoType)
      set({ tmdbSearchResults: results, tmdbLoading: false })
      return results
    } catch (error) {
      set({ error: String(error), tmdbLoading: false })
      throw error
    }
  },
  fetchTMDBDetail: async (tmdbId, videoType) => {
    try {
      return await api.getTMDBDetail(tmdbId, videoType)
    } catch (error) {
      set({ error: String(error) })
      throw error
    }
  },
  downloadTMDBPoster: async (movieId, posterUrl) => {
    try {
      return await api.downloadTMDBPoster(movieId, posterUrl)
    } catch (error) {
      set({ error: String(error) })
      throw error
    }
  },
  updateMovieFromTMDB: async (movieId, tmdbDetail) => {
    try {
      await api.updateMovieFromTMDB(movieId, tmdbDetail)
      await get().fetchMovies()
    } catch (error) {
      set({ error: String(error) })
      throw error
    }
  },
  smartUpdateRelatedMovies: async (sourceMovieId, tmdbDetail) => {
    try {
      const results = await api.smartUpdateRelatedMovies(sourceMovieId, tmdbDetail)
      await get().fetchMovies()
      return results
    } catch (error) {
      set({ error: String(error) })
      throw error
    }
  },
}))
