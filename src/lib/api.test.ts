import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  getDirectories,
  getMovies,
  scanDirectory,
  smartUpdateRelatedMovies,
  type TMDBMovieDetail,
} from "./api";
import { invoke } from "@tauri-apps/api/core";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("API Functions", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("getDirectories", () => {
    it("should call invoke with correct parameters", async () => {
      const mockDirectories = [
        {
          id: 1,
          path: "/test/movies",
          name: "Test Movies",
          path_type: "local",
          smb_connection_id: null,
          enabled: true,
          created_at: "2024-01-01",
        },
      ];
      vi.mocked(invoke).mockResolvedValue(mockDirectories);

      const result = await getDirectories();

      expect(invoke).toHaveBeenCalledWith("get_directories");
      expect(result).toEqual(mockDirectories);
    });
  });

  describe("getMovies", () => {
    it("should call invoke with filter options", async () => {
      const mockMovies = [
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
          video_type: "movie",
          season: null,
          episode: null,
          file_size: null,
          file_hash: null,
          created_at: "2024-01-01",
          updated_at: "2024-01-01",
        },
      ];
      vi.mocked(invoke).mockResolvedValue(mockMovies);

      const result = await getMovies({ directory_id: 1, video_type: "movie" });

      expect(invoke).toHaveBeenCalledWith("get_movies", {
        directory_id: 1,
        video_type: "movie",
      });
      expect(result).toEqual(mockMovies);
    });

    it("should call invoke with empty options when no filters", async () => {
      vi.mocked(invoke).mockResolvedValue([]);

      await getMovies();

      expect(invoke).toHaveBeenCalledWith("get_movies", {});
    });
  });

  describe("scanDirectory", () => {
    it("should call invoke with directoryId", async () => {
      vi.mocked(invoke).mockResolvedValue(5);

      const result = await scanDirectory(1);

      expect(invoke).toHaveBeenCalledWith("scan_directory", { directoryId: 1 });
      expect(result).toBe(5);
    });
  });

  describe("smartUpdateRelatedMovies", () => {
    it("should call invoke with correct parameters", async () => {
      const mockResults = [
        { movie_id: 1, movie_name: "Test Show S01", season: "1", episode: null },
        { movie_id: 2, movie_name: "Test Show S02", season: "2", episode: null },
      ];
      vi.mocked(invoke).mockResolvedValue(mockResults);

      const tmdbDetail: TMDBMovieDetail = {
        id: 12345,
        title: "Test Show",
        original_title: "Test Show",
        cn_title: "测试剧集",
        overview: "Test overview",
        poster_url: "https://example.com/poster.jpg",
        backdrop_url: null,
        year: "2023",
        runtime: null,
        vote_average: 8.5,
        genres: ["Drama"],
        countries: ["US"],
        imdb_id: "tt1234567",
      };

      const result = await smartUpdateRelatedMovies(1, tmdbDetail);

      expect(invoke).toHaveBeenCalledWith("smart_update_related_movies", {
        sourceMovieId: 1,
        tmdbDetail,
      });
      expect(result).toEqual(mockResults);
    });
  });
});
