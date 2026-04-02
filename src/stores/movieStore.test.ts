import { describe, it, expect, vi, beforeEach } from "vitest";
import { useMovieStore } from "./movieStore";
import * as api from "@/lib/api";

// Mock the API module
vi.mock("@/lib/api");

describe("movieStore", () => {
  beforeEach(() => {
    // Reset store state before each test
    useMovieStore.setState({
      movies: [],
      loading: false,
      error: null,
      selectedDirectory: null,
      searchQuery: "",
      videoType: null,
      tmdbSearchResults: [],
      tmdbLoading: false,
    });
    vi.clearAllMocks();
  });

  it("should initialize with default state", () => {
    const state = useMovieStore.getState();
    expect(state.movies).toEqual([]);
    expect(state.loading).toBe(false);
    expect(state.error).toBeNull();
    expect(state.selectedDirectory).toBeNull();
    expect(state.searchQuery).toBe("");
    expect(state.videoType).toBeNull();
  });

  it("should set selected directory", () => {
    const store = useMovieStore.getState();
    store.setSelectedDirectory(1);
    expect(useMovieStore.getState().selectedDirectory).toBe(1);
  });

  it("should set search query", () => {
    const store = useMovieStore.getState();
    store.setSearchQuery("test movie");
    expect(useMovieStore.getState().searchQuery).toBe("test movie");
  });

  it("should set video type", () => {
    const store = useMovieStore.getState();
    store.setVideoType("movie");
    expect(useMovieStore.getState().videoType).toBe("movie");
  });

  describe("smartUpdateRelatedMovies", () => {
    it("should call API and refresh movies", async () => {
      const mockResults = [
        { movie_id: 1, movie_name: "Test Movie", season: "1" },
        { movie_id: 2, movie_name: "Test Movie", season: "2" },
      ];
      
      vi.mocked(api.smartUpdateRelatedMovies).mockResolvedValue(mockResults);
      vi.mocked(api.getMovies).mockResolvedValue([
        {
          id: 1,
          directory_id: 1,
          filename: "test.mp4",
          path: "/test/test.mp4",
          cnname: "测试电影",
          cnoname: null,
          year: "2023",
          countries: null,
          douban_id: null,
          imdb_id: null,
          poster_path: null,
          fanart_path: null,
          description: null,
          douban_rating: null,
          imdb_rating: null,
          video_type: "tv",
          season: "1",
          episode: null,
          file_size: null,
          file_hash: null,
          created_at: "2024-01-01",
          updated_at: "2024-01-01",
        },
      ]);

      const store = useMovieStore.getState();
      const tmdbDetail: api.TMDBMovieDetail = {
        id: 123,
        title: "Test Movie",
        original_title: "Test Movie",
        cn_title: "测试电影",
        overview: "Test overview",
        poster_url: null,
        backdrop_url: null,
        year: "2023",
        runtime: 120,
        vote_average: 8.5,
        genres: ["Action"],
        countries: ["US"],
        imdb_id: "tt1234567",
      };

      const results = await store.smartUpdateRelatedMovies(1, tmdbDetail);

      expect(api.smartUpdateRelatedMovies).toHaveBeenCalledWith(1, tmdbDetail);
      expect(results).toEqual(mockResults);
    });

    it("should handle errors", async () => {
      vi.mocked(api.smartUpdateRelatedMovies).mockRejectedValue(new Error("API Error"));

      const store = useMovieStore.getState();
      const tmdbDetail: api.TMDBMovieDetail = {
        id: 123,
        title: "Test Movie",
        original_title: "Test Movie",
        cn_title: null,
        overview: null,
        poster_url: null,
        backdrop_url: null,
        year: null,
        runtime: null,
        vote_average: 0,
        genres: [],
        countries: [],
        imdb_id: null,
      };

      await expect(store.smartUpdateRelatedMovies(1, tmdbDetail)).rejects.toThrow("API Error");
      expect(useMovieStore.getState().error).toBe("Error: API Error");
    });
  });
});
