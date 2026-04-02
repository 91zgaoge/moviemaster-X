import { create } from "zustand"
import * as api from "@/lib/api"

interface DirectoryState {
  directories: api.Directory[]
  loading: boolean
  error: string | null
  
  fetchDirectories: () => Promise<void>
  addDirectory: (path: string, name?: string) => Promise<void>
  removeDirectory: (id: number) => Promise<void>
  toggleDirectory: (id: number, enabled: boolean) => Promise<void>
}

export const useDirectoryStore = create<DirectoryState>((set, get) => ({
  directories: [],
  loading: false,
  error: null,
  
  fetchDirectories: async () => {
    set({ loading: true, error: null })
    try {
      const directories = await api.getDirectories()
      set({ directories, loading: false })
    } catch (error) {
      set({ error: String(error), loading: false })
    }
  },
  
  addDirectory: async (path, name) => {
    set({ loading: true, error: null })
    try {
      await api.addDirectory({ path, name })
      await get().fetchDirectories()
    } catch (error) {
      set({ error: String(error), loading: false })
      throw error
    }
  },
  
  removeDirectory: async (id) => {
    set({ loading: true, error: null })
    try {
      await api.removeDirectory(id)
      await get().fetchDirectories()
    } catch (error) {
      set({ error: String(error), loading: false })
      throw error
    }
  },
  
  toggleDirectory: async (id, enabled) => {
    try {
      await api.toggleDirectory(id, enabled)
      await get().fetchDirectories()
    } catch (error) {
      set({ error: String(error) })
    }
  },
}))
