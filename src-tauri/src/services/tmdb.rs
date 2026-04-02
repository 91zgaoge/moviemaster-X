use reqwest;
use serde::{Deserialize, Serialize};
use std::path::Path;

const TMDB_API_BASE: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p";

// TMDB API 响应结构
#[derive(Debug, Deserialize)]
pub struct TMDBSearchResponse {
    pub results: Vec<TMDBMovieResult>,
    pub total_results: i32,
    pub page: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TMDBMovieResult {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub vote_average: f64,
    pub vote_count: i32,
    pub genre_ids: Vec<i32>,
    pub original_language: String,
}

#[derive(Debug, Deserialize)]
pub struct TMDBTVSearchResponse {
    pub results: Vec<TMDBTVResult>,
    pub total_results: i32,
    pub page: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TMDBTVResult {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: f64,
    pub vote_count: i32,
    pub genre_ids: Vec<i32>,
    pub original_language: String,
}

#[derive(Debug, Deserialize)]
pub struct TMDBMovieDetail {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: f64,
    pub vote_count: i32,
    pub genres: Vec<TMDBGenre>,
    pub production_countries: Vec<TMDBCountry>,
    pub imdb_id: Option<String>,
    pub original_language: String,
}

#[derive(Debug, Deserialize)]
pub struct TMDBTVDetail {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub episode_run_time: Vec<i32>,
    pub vote_average: f64,
    pub vote_count: i32,
    pub genres: Vec<TMDBGenre>,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub number_of_seasons: i32,
    pub number_of_episodes: i32,
    pub external_ids: Option<TMDBExternalIds>,
}

#[derive(Debug, Deserialize)]
pub struct TMDBGenre {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TMDBCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TMDBExternalIds {
    pub imdb_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TMDBTranslations {
    pub translations: Vec<TMDBTranslation>,
}

#[derive(Debug, Deserialize)]
pub struct TMDBTranslation {
    pub iso_639_1: String,
    pub iso_3166_1: String,
    pub name: String,
    pub english_name: String,
    pub data: TMDBTranslationData,
}

#[derive(Debug, Deserialize)]
pub struct TMDBTranslationData {
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub homepage: Option<String>,
    pub tagline: Option<String>,
}

// TMDB API 客户端
pub struct TMDBClient {
    api_key: String,
    client: reqwest::Client,
    language: String,
}

impl TMDBClient {
    /// 创建新的 TMDB 客户端
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            language: "zh-CN".to_string(), // 默认中文
        }
    }

    /// 设置语言
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    /// 搜索电影
    pub async fn search_movie(
        &self,
        query: &str,
        year: Option<i32>,
    ) -> Result<Vec<TMDBMovieResult>, String> {
        let url = format!("{}/search/movie", TMDB_API_BASE);
        
        let year_str = year.map(|y| y.to_string());
        
        let mut request = self.client.get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("query", query),
                ("language", self.language.as_str()),
                ("include_adult", "false"),
            ]);
        
        if let Some(ref y) = year_str {
            request = request.query(&[("year", y.as_str())]);
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| format!("TMDB search request failed: {}", e))?
            .json::<TMDBSearchResponse>()
            .await
            .map_err(|e| format!("TMDB search parse failed: {}", e))?;
        
        Ok(response.results)
    }

    /// 搜索电视剧
    pub async fn search_tv(
        &self,
        query: &str,
        year: Option<i32>,
    ) -> Result<Vec<TMDBTVResult>, String> {
        let url = format!("{}/search/tv", TMDB_API_BASE);
        
        let year_str = year.map(|y| y.to_string());
        
        let mut request = self.client.get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("query", query),
                ("language", self.language.as_str()),
                ("include_adult", "false"),
            ]);
        
        if let Some(ref y) = year_str {
            request = request.query(&[("first_air_date_year", y.as_str())]);
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| format!("TMDB TV search request failed: {}", e))?
            .json::<TMDBTVSearchResponse>()
            .await
            .map_err(|e| format!("TMDB TV search parse failed: {}", e))?;
        
        Ok(response.results)
    }

    /// 获取电影详情
    pub async fn get_movie_detail(&self,
        movie_id: i64,
    ) -> Result<TMDBMovieDetail, String> {
        let url = format!("{}/movie/{}", TMDB_API_BASE, movie_id);
        
        let params = [
            ("api_key", self.api_key.as_str()),
            ("language", &self.language),
        ];
        
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("TMDB movie detail request failed: {}", e))?
            .json::<TMDBMovieDetail>()
            .await
            .map_err(|e| format!("TMDB movie detail parse failed: {}", e))?;
        
        Ok(response)
    }

    /// 获取电视剧详情
    pub async fn get_tv_detail(&self,
        tv_id: i64,
    ) -> Result<TMDBTVDetail, String> {
        let url = format!("{}/tv/{}", TMDB_API_BASE, tv_id);
        
        let params = [
            ("api_key", self.api_key.as_str()),
            ("language", &self.language),
            ("append_to_response", "external_ids"),
        ];
        
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("TMDB TV detail request failed: {}", e))?
            .json::<TMDBTVDetail>()
            .await
            .map_err(|e| format!("TMDB TV detail parse failed: {}", e))?;
        
        Ok(response)
    }

    /// 获取翻译信息（获取中文标题）
    pub async fn get_movie_translations(
        &self,
        movie_id: i64,
    ) -> Result<TMDBTranslations, String> {
        let url = format!("{}/movie/{}/translations", TMDB_API_BASE, movie_id);
        
        let params = [("api_key", self.api_key.as_str())];
        
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("TMDB translations request failed: {}", e))?
            .json::<TMDBTranslations>()
            .await
            .map_err(|e| format!("TMDB translations parse failed: {}", e))?;
        
        Ok(response)
    }

    /// 获取电视剧翻译
    pub async fn get_tv_translations(
        &self,
        tv_id: i64,
    ) -> Result<TMDBTranslations, String> {
        let url = format!("{}/tv/{}/translations", TMDB_API_BASE, tv_id);
        
        let params = [("api_key", self.api_key.as_str())];
        
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("TMDB TV translations request failed: {}", e))?
            .json::<TMDBTranslations>()
            .await
            .map_err(|e| format!("TMDB TV translations parse failed: {}", e))?;
        
        Ok(response)
    }

    /// 获取中文标题
    pub async fn get_chinese_title(
        &self,
        movie_id: i64,
        is_tv: bool,
    ) -> Result<Option<String>, String> {
        let translations = if is_tv {
            self.get_tv_translations(movie_id).await?
        } else {
            self.get_movie_translations(movie_id).await?
        };
        
        // 查找中文翻译
        for translation in translations.translations {
            if translation.iso_639_1 == "zh" {
                return Ok(translation.data.title.or(translation.data.name));
            }
        }
        
        Ok(None)
    }

    /// 下载海报图片
    pub async fn download_poster(
        &self,
        poster_path: &str,
        save_path: &Path,
    ) -> Result<(), String> {
        let url = format!("{}{}{}", TMDB_IMAGE_BASE, "/w500", poster_path);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Download poster request failed: {}", e))?;
        
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Download poster bytes failed: {}", e))?;
        
        std::fs::write(save_path, bytes)
            .map_err(|e| format!("Save poster failed: {}", e))?;
        
        Ok(())
    }

    /// 获取完整的海报 URL
    pub fn get_poster_url(&self, poster_path: Option<&String>, size: &str) -> Option<String> {
        poster_path.map(|path| format!("{}{}{}", TMDB_IMAGE_BASE, size, path))
    }
}

/// 获取 TMDB API Key（从设置中读取）
pub async fn get_tmdb_api_key(db: &crate::db::Database) -> Result<Option<String>, String> {
    // 这里可以从数据库的设置表中读取 API Key
    // 现在先返回一个默认值用于测试
    // 用户需要到 https://www.themoviedb.org/settings/api 申请 API Key
    Ok(Some("YOUR_TMDB_API_KEY".to_string()))
}