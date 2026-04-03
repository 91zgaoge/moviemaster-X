use crate::models::ParsedFilename;
use regex::Regex;
use std::path::Path;

pub struct FilenameParser;

/// Smart series grouping key - extracts base title for grouping episodes
pub struct SeriesGrouper;

impl SeriesGrouper {
    /// Extract series grouping key from filename
    /// This is used to group episodes from the same series together
    pub fn extract_series_key(filename: &str) -> String {
        let path = Path::new(filename);
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);

        // Remove episode patterns to get series base name
        let patterns = [
            // S01E01 patterns
            r"[.\s_-]*[Ss]\d{1,2}[Ee]\d{1,2}.*$",
            r"[.\s_-]*\d{1,2}[Xx]\d{1,2}.*$",
            // Episode 01 patterns
            r"[.\s_-]*[Ee]pisode[.\s_-]*\d{1,3}.*$",
            r"[.\s_-]*[Ee]p[.\s_-]*\d{1,3}.*$",
            // Season 1 Episode 1 patterns
            r"[.\s_-]*[Ss]eason[.\s_-]*\d{1,2}[.\s_-]*[Ee]pisode[.\s_-]*\d{1,3}.*$",
            // Pure numeric patterns like .01. or _01_ at the end
            r"[.\s_-]+\d{1,3}[.\s_-]+(?=\d{4}p|720p|1080p|2160p|4k|bluray|webrip|hdtv|$)",
            // Chinese episode patterns
            r"[第]\d{1,3}[集话].*$",
            r"[.\s_-]*\d{1,3}[集话].*$",
        ];

        let mut result = stem.to_string();
        for pattern in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        // Clean up common separators and suffixes
        let clean_patterns = [
            r"[\[\(\{].*?[\]\)\}]",  // Remove brackets and their content
            r"[-_.]+$",               // Remove trailing separators
        ];

        for pattern in &clean_patterns {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        // Normalize: lowercase, remove dots and underscores, trim
        result.to_lowercase()
            .replace('.', " ")
            .replace('_', " ")
            .replace('-', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }

    /// Check if two filenames likely belong to the same series
    pub fn same_series(filename1: &str, filename2: &str) -> bool {
        let key1 = Self::extract_series_key(filename1);
        let key2 = Self::extract_series_key(filename2);

        // Exact match
        if key1 == key2 {
            return true;
        }

        // Check if one is a substring of the other (for truncated names)
        if key1.len() > 5 && key2.len() > 5 {
            if key1.contains(&key2) || key2.contains(&key1) {
                return true;
            }
        }

        // Check similarity for minor differences
        let similarity = Self::calculate_similarity(&key1, &key2);
        similarity > 0.85
    }

    /// Calculate string similarity using Levenshtein distance
    fn calculate_similarity(s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len());
        if max_len == 0 {
            return 1.0;
        }

        let distance = Self::levenshtein_distance(s1, s2);
        1.0 - (distance as f64 / max_len as f64)
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = [
                    matrix[i][j + 1] + 1,      // deletion
                    matrix[i + 1][j] + 1,      // insertion
                    matrix[i][j] + cost,       // substitution
                ].into_iter().min().unwrap();
            }
        }

        matrix[len1][len2]
    }
}

impl FilenameParser {
    pub fn parse(filename: &str) -> ParsedFilename {
        let path = Path::new(filename);
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);
        
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Detect video type by extension
        let video_type = if is_tv_extension(&ext) {
            "tv"
        } else {
            "movie"
        };

        // Common patterns for TV shows
        // S01E01, 1x01, Season 1 Episode 1
        let tv_patterns = [
            r"(?i)(.+?)[.\s_-]+[Ss](\d{1,2})[Ee](\d{1,2})",  // S01E01
            r"(?i)(.+?)[.\s_-]+(\d{1,2})x(\d{1,2})",         // 1x01
            r"(?i)(.+?)[.\s_-]+[Ss]eason[.\s_-]*(\d{1,2})[.\s_-]*[Ee]pisode[.\s_-]*(\d{1,2})", // Season 1 Episode 1
            r"(?i)(.+?)[.\s_-]+(\d{1,2})[.\s_-]+(\d{1,2})",  // 2023.01.01 (date-like for episodes)
        ];

        for pattern in &tv_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(stem) {
                    let title = caps.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
                    let season = caps.get(2).map(|m| m.as_str().to_string());
                    let episode = caps.get(3).map(|m| m.as_str().to_string());
                    
                    return ParsedFilename {
                        title: clean_title(&title),
                        year: None,
                        season,
                        episode,
                        resolution: extract_resolution(stem),
                        video_codec: extract_video_codec(stem),
                        audio_codec: extract_audio_codec(stem),
                        release_group: extract_release_group(stem),
                        video_type: "tv".to_string(),
                    };
                }
            }
        }

        // Movie pattern: Title.YEAR.Resolution.Codec
        let movie_pattern = r"(?i)(.+?)[\.\s_-]+(\d{4})";
        if let Ok(re) = Regex::new(movie_pattern) {
            if let Some(caps) = re.captures(stem) {
                let title = caps.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
                let year = caps.get(2).map(|m| m.as_str().to_string());
                
                // Validate year is reasonable
                if let Some(ref y) = year {
                    if let Ok(year_num) = y.parse::<i32>() {
                        if year_num >= 1900 && year_num <= 2030 {
                            return ParsedFilename {
                                title: clean_title(&title),
                                year: Some(year_num.to_string()),
                                season: None,
                                episode: None,
                                resolution: extract_resolution(stem),
                                video_codec: extract_video_codec(stem),
                                audio_codec: extract_audio_codec(stem),
                                release_group: extract_release_group(stem),
                                video_type: "movie".to_string(),
                            };
                        }
                    }
                }
            }
        }

        // Fallback: just use the filename as title
        ParsedFilename {
            title: clean_title(stem),
            year: None,
            season: None,
            episode: None,
            resolution: extract_resolution(stem),
            video_codec: extract_video_codec(stem),
            audio_codec: extract_audio_codec(stem),
            release_group: extract_release_group(stem),
            video_type: video_type.to_string(),
        }
    }
}

fn is_tv_extension(ext: &str) -> bool {
    matches!(ext, "mkv" | "mp4" | "avi" | "ts" | "m4v")
}

fn clean_title(title: &str) -> String {
    // Remove common release info from title
    let patterns = [
        r"\[.*?\]",           // [xxx]
        r"\(.*?\)",           // (xxx)
        r"\{.*?\}",           // {xxx}
        r"1080p|720p|2160p|4k|4K",
        r"bluray|blu-ray|bdrip|brrip|dvdrip|webrip|hdtv",
        r"x264|x265|hevc|h\\.?264|h\\.?265",
        r"aac|ac3|dts|flac|mp3",
        r"yify|yts|rarbg|evo|juggs|monova",
        r"proper|repack|rerip",
        r"-.*$",
    ];

    let mut result = title.to_string();
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            result = re.replace_all(&result, " ").to_string();
        }
    }

    // Clean up multiple spaces and dots
    result.split(|c: char| c.is_whitespace() || c == '.' || c == '_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_resolution(text: &str) -> Option<String> {
    let patterns = [
        r"(\d{3,4})[pP]",
        r"4[Kk]",
        r"2160",
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(m) = re.find(text) {
                let resolution = m.as_str().to_uppercase();
                if resolution.contains("4K") {
                    return Some("2160p".to_string());
                }
                if resolution == "2160" {
                    return Some("2160p".to_string());
                }
                return Some(resolution);
            }
        }
    }
    None
}

fn extract_video_codec(text: &str) -> Option<String> {
    let patterns = [
        (r"x265|hevc", "H.265/HEVC"),
        (r"x264|h264|avc", "H.264/AVC"),
        (r"vp9", "VP9"),
        (r"av1", "AV1"),
    ];
    
    for (pattern, name) in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(text) {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn extract_audio_codec(text: &str) -> Option<String> {
    let patterns = [
        (r"dts[- ]?x|DTSX", "DTS:X"),
        (r"dts[- ]?hd|DTSHD", "DTS-HD"),
        (r"atmos", "Dolby Atmos"),
        (r"truehd", "Dolby TrueHD"),
        (r"ac3|dolby", "Dolby Digital"),
        (r"aac", "AAC"),
        (r"flac", "FLAC"),
    ];
    
    for (pattern, name) in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(text) {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn extract_release_group(text: &str) -> Option<String> {
    // Look for release group at the end of filename
    let pattern = r"-([a-zA-Z0-9]+)$";
    if let Ok(re) = Regex::new(pattern) {
        if let Some(caps) = re.captures(text) {
            let group = caps.get(1).map(|m| m.as_str());
            if let Some(g) = group {
                // Skip common non-group strings
                if !["1080p", "720p", "brrip", "dvdrip"].contains(&g.to_lowercase().as_str()) {
                    return Some(g.to_string());
                }
            }
        }
    }
    None
}

pub fn is_video_file(path: &Path) -> bool {
    let video_extensions = [
        "mp4", "mkv", "avi", "mov", "wmv", "flv", 
        "webm", "m4v", "ts", "mts", "m2ts"
    ];
    
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| video_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}
