import { invoke } from "@tauri-apps/api/core";

// Types
export interface Directory {
  id: number;
  path: string;
  name: string | null;
  path_type: string;
  smb_connection_id: string | null;
  enabled: boolean;
  created_at: string;
}

export interface Movie {
  id: number;
  directory_id: number;
  series_id?: number | null;
  filename: string;
  path: string;
  cnname: string | null;
  cnoname: string | null;
  year: string | null;
  countries: string | null;
  douban_id: string | null;
  imdb_id: string | null;
  poster_path: string | null;
  fanart_path: string | null;
  description: string | null;
  douban_rating: number | null;
  imdb_rating: number | null;
  video_type: string;
  season: string | null;
  episode: string | null;
  file_size: number | null;
  file_hash: string | null;
  created_at: string;
  updated_at: string;
}

export interface Series {
  id: number;
  title: string;
  cnname: string | null;
  year: string | null;
  description: string | null;
  poster_path: string | null;
  fanart_path: string | null;
  douban_id: string | null;
  imdb_id: string | null;
  douban_rating: number | null;
  imdb_rating: number | null;
  total_seasons: number;
  total_episodes: number;
  created_at: string;
  updated_at: string;
}

export interface SMBConnection {
  id: string;
  name: string;
  host: string;
  share: string;
  username: string | null;
  password: string | null;
  domain: string | null;
  created_at: string;
}

export interface Settings {
  key: string;
  value: string | null;
}

export interface AddDirectoryRequest {
  path: string;
  name?: string;
  path_type?: string;
  smb_connection_id?: string;
}

export interface DoubanSearchResult {
  id: string;
  title: string;
  year: string;
  rating: number | null;
  poster: string | null;
  genres: string[];
}

// TMDB Types
export interface TMDBSearchResult {
  id: number;
  title: string;
  original_title: string;
  year: string | null;
  overview: string | null;
  poster_url: string | null;
  vote_average: number;
  video_type: string;
}

export interface TMDBMovieDetail {
  id: number;
  title: string;
  original_title: string;
  cn_title: string | null;
  overview: string | null;
  poster_url: string | null;
  backdrop_url: string | null;
  year: string | null;
  runtime: number | null;
  vote_average: number;
  genres: string[];
  countries: string[];
  imdb_id: string | null;
}

// Directory API
export async function getDirectories(): Promise<Directory[]> {
  return invoke("get_directories");
}

export async function addDirectory(request: AddDirectoryRequest): Promise<Directory> {
  return invoke("add_directory", { request });
}

export async function removeDirectory(id: number): Promise<void> {
  return invoke("remove_directory", { id });
}

export async function toggleDirectory(id: number, enabled: boolean): Promise<void> {
  return invoke("toggle_directory", { id, enabled });
}

// Movie API
export async function getMovies(options?: {
  directory_id?: number;
  video_type?: string;
  search?: string;
  limit?: number;
  offset?: number;
}): Promise<Movie[]> {
  return invoke("get_movies", options || {});
}

export async function scanDirectory(directoryId: number): Promise<number> {
  return invoke("scan_directory", { directoryId });
}

// Series API
export async function getSeries(options?: {
  search?: string;
  limit?: number;
  offset?: number;
}): Promise<Series[]> {
  return invoke("get_series", options || {});
}

export async function getSeriesEpisodes(seriesId: number, season?: string): Promise<Movie[]> {
  return invoke("get_series_episodes", { seriesId, season });
}

export async function getMoviesUngrouped(options?: {
  directory_id?: number;
  search?: string;
  limit?: number;
  offset?: number;
}): Promise<Movie[]> {
  return invoke("get_movies_ungrouped", options || {});
}

export async function getMovieById(id: number): Promise<Movie | null> {
  return invoke("get_movie_by_id", { id });
}

export async function updateMovieInfo(
  id: number,
  data: {
    cnname?: string;
    cnoname?: string;
    year?: string;
    countries?: string;
    douban_id?: string;
    imdb_id?: string;
    description?: string;
    douban_rating?: number;
    imdb_rating?: number;
    poster_path?: string;
    fanart_path?: string;
  }
): Promise<void> {
  return invoke("update_movie_info", { id, ...data });
}

export async function searchDouban(title: string, year?: string): Promise<DoubanSearchResult[]> {
  return invoke("search_douban", { title, year });
}

export async function fetchDoubanInfo(movieId: number): Promise<Movie> {
  return invoke("fetch_douban_info", { movieId });
}

export async function downloadPoster(movieId: number, posterUrl: string): Promise<string> {
  return invoke("download_poster", { movieId, posterUrl });
}

export async function generateNfo(movieId: number): Promise<string> {
  return invoke("generate_nfo", { movieId });
}

// TMDB API
export async function searchTMDB(
  _apiKey: string,
  title: string,
  year?: number,
  videoType: string = "movie"
): Promise<TMDBSearchResult[]> {
  return invoke("search_tmdb", { title, year, videoType });
}

export async function getTMDBDetail(
  tmdbId: number,
  videoType: string = "movie"
): Promise<TMDBMovieDetail> {
  return invoke("get_tmdb_detail", { tmdbId, videoType });
}

export async function downloadTMDBPoster(
  movieId: number,
  posterUrl: string
): Promise<string> {
  return invoke("download_tmdb_poster", { movieId, posterUrl });
}

export async function updateMovieFromTMDB(
  movieId: number,
  tmdbDetail: TMDBMovieDetail
): Promise<void> {
  return invoke("update_movie_from_tmdb", { movieId, tmdbDetail });
}

export interface SmartUpdateResult {
  movie_id: number;
  movie_name: string;
  season?: string;
  episode?: string;
}

export async function smartUpdateRelatedMovies(
  sourceMovieId: number,
  tmdbDetail: TMDBMovieDetail
): Promise<SmartUpdateResult[]> {
  return invoke("smart_update_related_movies", { sourceMovieId, tmdbDetail });
}

// SMB API
export async function getSMBConnections(): Promise<SMBConnection[]> {
  return invoke("get_smb_connections");
}

export async function addSMBConnection(
  name: string,
  host: string,
  share: string,
  username?: string,
  password?: string,
  domain?: string
): Promise<SMBConnection> {
  return invoke("add_smb_connection", { name, host, share, username, password, domain });
}

export async function removeSMBConnection(id: string): Promise<void> {
  return invoke("remove_smb_connection", { id });
}

export async function testSMBConnection(
  host: string,
  share: string,
  username?: string,
  password?: string
): Promise<boolean> {
  return invoke("test_smb_connection", { host, share, username, password });
}

// Settings API
export async function getSettings(): Promise<Settings[]> {
  return invoke("get_settings");
}

export async function updateSetting(key: string, value: string | null): Promise<void> {
  return invoke("update_setting", { key, value });
}

// Config Export/Import API
export async function exportConfig(): Promise<string> {
  return invoke("export_config");
}

export interface ImportResult {
  settings_count: number;
  directories_count: number;
}

export async function importConfig(): Promise<ImportResult> {
  return invoke("import_config");
}

// Utility API
export async function openMovieFile(path: string): Promise<void> {
  return invoke("open_movie_file", { path });
}

export async function deleteMovie(movieId: number): Promise<void> {
  return invoke("delete_movie", { movieId });
}

// Import/Export API
export async function exportMoviesJson(outputPath: string): Promise<void> {
  return invoke("export_movies_json", { outputPath });
}

export async function exportMoviesCsv(outputPath: string): Promise<void> {
  return invoke("export_movies_csv", { outputPath });
}

export async function importMoviesJson(jsonContent: string): Promise<number> {
  return invoke("import_movies_json", { jsonContent });
}

export async function importMoviesCsv(csvPath: string): Promise<number> {
  return invoke("import_movies_csv", { csvPath });
}

// Subtitle API
export interface SubtitleResult {
  id: string;
  filename: string;
  language: string;
  format: string;
  download_count: number;
  source: string;
}

// Duplicate Detection Types
export interface DuplicateMovie {
  id: number;
  filename: string;
  path: string;
  cnname: string | null;
  file_size: number | null;
  file_hash: string | null;
  video_type: string;
  season: string | null;
  episode: string | null;
  poster_path: string | null;
  imdb_id: string | null;
  douban_rating: number | null;
  created_at: string;
  completeness_score: number;
}

export interface DuplicateGroup {
  key: string;
  match_type: string;
  movies: DuplicateMovie[];
  suggested_keep: number;
}

export async function searchSubtitles(movieId: number, language: string): Promise<SubtitleResult[]> {
  return invoke("search_subtitles", { movieId, language });
}

export async function downloadSubtitle(movieId: number, subtitleId: string, savePath: string): Promise<string> {
  return invoke("download_subtitle", { movieId, subtitleId, savePath });
}

// Duplicate Detection API
export async function scanDuplicates(): Promise<DuplicateGroup[]> {
  return invoke("scan_duplicates");
}

export async function deleteDuplicates(keepId: number, deleteIds: number[]): Promise<number> {
  return invoke("delete_duplicates", { keepId, deleteIds });
}

// PT-Depiler Types
export interface PTSearchRequest {
  keyword: string;
  year?: string;
  video_type?: "movie" | "tv";
  season?: string;
  episode?: string;
  imdb_id?: string;
}

export interface PTSearchResult {
  site: string;
  title: string;
  original_title?: string;
  year?: string;
  torrent_url: string;
  download_url: string;
  size: string;
  seeders: number;
  leechers: number;
  snatched: number;
  freeleech: boolean;
  double_upload: boolean;
  publish_time: string;
  category: string;
  imdb_id?: string;
  douban_id?: string;
  description?: string;
  poster_url?: string;
}

export interface PTSiteConfig {
  name: string;
  enabled: boolean;
  api_url: string;
  passkey: string;
  cookie: string;
  auth_type: "passkey" | "cookie" | "api_key";
}

// PT-Depiler API
export async function ptDepilerSearch(request: PTSearchRequest): Promise<PTSearchResult[]> {
  return invoke("pt_depiler_search", { request });
}

export async function ptDepilerGetSites(): Promise<string[]> {
  return invoke("pt_depiler_get_sites");
}

export async function ptDepilerAddSite(config: PTSiteConfig): Promise<void> {
  return invoke("pt_depiler_add_site", { config });
}

export async function ptDepilerRemoveSite(siteName: string): Promise<void> {
  return invoke("pt_depiler_remove_site", { siteName });
}

export async function ptDepilerTestSite(
  siteName: string,
  apiUrl: string,
  authType: string,
  passkey?: string,
  cookie?: string
): Promise<boolean> {
  return invoke("pt_depiler_test_site", { siteName, apiUrl, authType, passkey, cookie });
}

export async function ptDepilerDownloadTorrent(
  siteName: string,
  torrentUrl: string,
  savePath: string
): Promise<string> {
  return invoke("pt_depiler_download_torrent", { siteName, torrentUrl, savePath });
}

// qBittorrent Types
export interface QBConfig {
  base_url: string;
  username: string;
  password: string;
  default_save_path?: string;
}

export interface QBTorrent {
  hash: string;
  name: string;
  size: number;
  progress: number;
  dlspeed: number;
  upspeed: number;
  state: string;
  category: string;
  save_path: string;
  added_on: number;
  completion_on: number;
}

// qBittorrent API
export async function qbittorrentTestConnection(
  baseUrl: string,
  username: string,
  password: string
): Promise<boolean> {
  return invoke("qbittorrent_test_connection", { baseUrl, username, password });
}

export async function qbittorrentAddTorrent(
  torrentUrl: string,
  savePath?: string
): Promise<void> {
  return invoke("qbittorrent_add_torrent", { torrentUrl, savePath });
}

export async function qbittorrentGetTorrents(): Promise<QBTorrent[]> {
  return invoke("qbittorrent_get_torrents");
}

export async function qbittorrentSaveConfig(config: QBConfig): Promise<void> {
  return invoke("qbittorrent_save_config", { config });
}

export async function qbittorrentLoadConfig(): Promise<QBConfig> {
  return invoke("qbittorrent_load_config");
}

// AI Agent Types
export interface AgentResponse {
  content: string;
  tool_results: ToolResult[];
  suggested_actions: string[];
}

export interface ToolResult {
  tool_name: string;
  success: boolean;
  result: string;
}

export interface AgentMetrics {
  total_interactions: number;
  successful_tasks: number;
  failed_tasks: number;
  avg_response_time_ms: number;
  user_satisfaction_score: number;
}

export interface SkillInfo {
  id: string;
  name: string;
  description: string;
  version: number;
}

// AI Agent API
export async function agentSendMessage(message: string): Promise<AgentResponse> {
  return invoke("agent_send_message", { message });
}

export async function agentGetMetrics(): Promise<AgentMetrics> {
  return invoke("agent_get_metrics");
}

export async function agentGetAvailableSkills(): Promise<SkillInfo[]> {
  return invoke("agent_get_available_skills");
}

export async function agentExportKnowledge(): Promise<string> {
  return invoke("agent_export_knowledge");
}

export async function agentImportKnowledge(knowledgeJson: string): Promise<void> {
  return invoke("agent_import_knowledge", { knowledgeJson });
}

export async function agentTestLLMConnection(endpoint?: string, apiKey?: string): Promise<boolean> {
  return invoke("agent_test_llm_connection", { endpoint, apiKey });
}
