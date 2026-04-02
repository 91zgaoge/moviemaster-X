import { create } from "zustand"
import * as api from "@/lib/api"

interface MovieState {
  movies: api.Movie[]
  loading: boolean
  error: string | null
  selectedDirectory: number | null
  searchQuery: string
  videoType: string | null
  tmdbSearchResults: api.TMDBSearchResult[]
  tmdbLoading: boolean
  
  fetchMovies: () => Promise<void>
  scanDirectory: (directoryId: number) => Promise<number>
  setSelectedDirectory: (id: number | null) => void
  setSearchQuery: (query: string) => void
  setVideoType: (type: string | null) => void
  // TMDB actions
  searchTMDB: (title: string, year?: number, videoType?: string) => Promise<api.TMDBSearchResult[]>
  fetchTMDBDetail: (tmdbId: number, videoType: string) => Promise<api.TMDBMovieDetail>
  downloadTMDBPoster: (movieId: number, posterUrl: string) => Promise<string>
  updateMovieFromTMDB: (movieId: number, tmdbDetail: api.TMDBMovieDetail) => Promise<void>
  smartUpdateRelatedMovies: (sourceMovieId: number, tmdbDetail: api.TMDBMovieDetail) => Promise<api.SmartUpdateResult[]>
}

export const useMovieStore = create<MovieState>((set, get) => ({
  movies: [],
  loading: false,
  error: null,
  selectedDirectory: null,
  searchQuery: "",
  videoType: null,
  tmdbSearchResults: [],
  tmdbLoading: false,
  
  fetchMovies: async () => {
    set({ loading: true, error: null })
    try {
      const { selectedDirectory, searchQuery, videoType } = get()
      const movies = await api.getMovies({
        directory_id: selectedDirectory || undefined,
        video_type: videoType || undefined,
        search: searchQuery || undefined,
      })
      set({ movies, loading: false })
    } catch (error) {
      set({ error: String(error), loading: false })
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
