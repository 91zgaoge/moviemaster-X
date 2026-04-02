use crate::models::Movie;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 完整的影片NFO数据结构
#[derive(Debug, Clone, Default)]
pub struct NFOData {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub rating: Option<f64>,
    pub votes: Option<i32>,
    pub plot: Option<String>,
    pub outline: Option<String>,
    pub tagline: Option<String>,
    pub runtime: Option<i32>,
    pub premiered: Option<String>,
    pub genres: Vec<String>,
    pub countries: Vec<String>,
    pub directors: Vec<String>,
    pub credits: Vec<String>,
    pub actors: Vec<Actor>,
    pub studios: Vec<String>,
    pub imdb_id: Option<String>,
    pub tmdb_id: Option<String>,
    pub trailer: Option<String>,
    pub poster: Option<String>,
    pub fanart: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub name: String,
    pub role: Option<String>,
    pub thumb: Option<String>,
}

impl From<&Movie> for NFOData {
    fn from(movie: &Movie) -> Self {
        NFOData {
            title: movie.cnname.clone(),
            original_title: movie.cnoname.clone(),
            sort_title: None,
            year: movie.year.clone(),
            rating: movie.douban_rating,
            votes: None,
            plot: movie.description.clone(),
            outline: movie.description.clone(),
            tagline: None,
            runtime: None,
            premiered: movie.year.as_ref().map(|y| format!("{}-01-01", y)),
            genres: Vec::new(),
            countries: movie.countries.as_ref()
                .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            directors: Vec::new(),
            credits: Vec::new(),
            actors: Vec::new(),
            studios: Vec::new(),
            imdb_id: movie.imdb_id.clone(),
            tmdb_id: None,
            trailer: None,
            poster: movie.poster_path.clone(),
            fanart: movie.fanart_path.clone(),
        }
    }
}

/// 生成电影NFO（Kodi格式）
pub fn generate_movie_nfo(data: &NFOData) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    xml.push('\n');
    xml.push_str(r#"<movie xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#);
    xml.push('\n');
    
    // 标题
    if let Some(ref title) = data.title {
        xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    }
    
    // 原标题
    if let Some(ref orig) = data.original_title {
        xml.push_str(&format!("  <originaltitle>{}</originaltitle>\n", escape_xml(orig)));
    }
    
    // 排序标题
    if let Some(ref sort) = data.sort_title {
        xml.push_str(&format!("  <sorttitle>{}</sorttitle>\n", escape_xml(sort)));
    }
    
    // 年份
    if let Some(ref year) = data.year {
        xml.push_str(&format!("  <year>{}</year>\n", escape_xml(year)));
    }
    
    // 评分
    if let Some(rating) = data.rating {
        xml.push_str(&format!("  <rating>{:.1}</rating>\n", rating));
    }
    
    // 投票数
    if let Some(votes) = data.votes {
        xml.push_str(&format!("  <votes>{}</votes>\n", votes));
    }
    
    // 时长
    if let Some(runtime) = data.runtime {
        xml.push_str(&format!("  <runtime>{}</runtime>\n", runtime));
    }
    
    // 简介
    if let Some(ref plot) = data.plot {
        xml.push_str(&format!("  <plot>{}</plot>\n", escape_xml(plot)));
    }
    
    // 概要
    if let Some(ref outline) = data.outline {
        xml.push_str(&format!("  <outline>{}</outline>\n", escape_xml(outline)));
    }
    
    // 标语
    if let Some(ref tagline) = data.tagline {
        xml.push_str(&format!("  <tagline>{}</tagline>\n", escape_xml(tagline)));
    }
    
    // 首映日期
    if let Some(ref premiered) = data.premiered {
        xml.push_str(&format!("  <premiered>{}</premiered>\n", escape_xml(premiered)));
    }
    
    // 国家
    for country in &data.countries {
        xml.push_str(&format!("  <country>{}</country>\n", escape_xml(country)));
    }
    
    // 类型
    for genre in &data.genres {
        xml.push_str(&format!("  <genre>{}</genre>\n", escape_xml(genre)));
    }
    
    // 如果没有类型，添加默认类型
    if data.genres.is_empty() {
        xml.push_str("  <genre>Movie</genre>\n");
    }
    
    // 导演
    for director in &data.directors {
        xml.push_str(&format!("  <director>{}</director>\n", escape_xml(director)));
    }
    
    // 编剧
    for credit in &data.credits {
        xml.push_str(&format!("  <credits>{}</credits>\n", escape_xml(credit)));
    }
    
    // 演员
    for actor in &data.actors {
        xml.push_str("  <actor>\n");
        xml.push_str(&format!("    <name>{}</name>\n", escape_xml(&actor.name)));
        if let Some(ref role) = actor.role {
            xml.push_str(&format!("    <role>{}</role>\n", escape_xml(role)));
        }
        if let Some(ref thumb) = actor.thumb {
            xml.push_str(&format!("    <thumb>{}</thumb>\n", escape_xml(thumb)));
        }
        xml.push_str("  </actor>\n");
    }
    
    // 制作公司
    for studio in &data.studios {
        xml.push_str(&format!("  <studio>{}</studio>\n", escape_xml(studio)));
    }
    
    // IMDB ID
    if let Some(ref imdb) = data.imdb_id {
        xml.push_str(&format!("  <id>{}</id>\n", escape_xml(imdb)));
        xml.push_str(&format!(r#"  <uniqueid type="imdb" default="true">{}</uniqueid>
"#, escape_xml(imdb)));
    }
    
    // TMDB ID
    if let Some(ref tmdb) = data.tmdb_id {
        xml.push_str(&format!(r#"  <uniqueid type="tmdb">{}</uniqueid>
"#, escape_xml(tmdb)));
    }
    
    // 预告片
    if let Some(ref trailer) = data.trailer {
        xml.push_str(&format!("  <trailer>{}</trailer>\n", escape_xml(trailer)));
    }
    
    // 海报
    if let Some(ref poster) = data.poster {
        xml.push_str(&format!(r#"  <thumb aspect="poster">{}</thumb>
"#, escape_xml(poster)));
    }
    
    // 背景图
    if let Some(ref fanart) = data.fanart {
        xml.push_str("  <fanart>\n");
        xml.push_str(&format!("    <thumb>{}</thumb>\n", escape_xml(fanart)));
        xml.push_str("  </fanart>\n");
    }
    
    xml.push_str("</movie>");
    xml
}

/// 生成电视剧NFO
pub fn generate_tvshow_nfo(data: &NFOData) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    xml.push('\n');
    xml.push_str(r#"<tvshow xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#);
    xml.push('\n');
    
    if let Some(ref title) = data.title {
        xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    }
    
    if let Some(ref orig) = data.original_title {
        xml.push_str(&format!("  <originaltitle>{}</originaltitle>\n", escape_xml(orig)));
    }
    
    if let Some(ref year) = data.year {
        xml.push_str(&format!("  <year>{}</year>\n", escape_xml(year)));
    }
    
    if let Some(rating) = data.rating {
        xml.push_str(&format!("  <rating>{:.1}</rating>\n", rating));
    }
    
    if let Some(ref plot) = data.plot {
        xml.push_str(&format!("  <plot>{}</plot>\n", escape_xml(plot)));
    }
    
    for genre in &data.genres {
        xml.push_str(&format!("  <genre>{}</genre>\n", escape_xml(genre)));
    }
    
    if data.genres.is_empty() {
        xml.push_str("  <genre>TV Show</genre>\n");
    }
    
    for country in &data.countries {
        xml.push_str(&format!("  <country>{}</country>\n", escape_xml(country)));
    }
    
    if let Some(ref imdb) = data.imdb_id {
        xml.push_str(&format!("  <id>{}</id>\n", escape_xml(imdb)));
    }
    
    xml.push_str("</tvshow>");
    xml
}

/// 生成单集NFO
pub fn generate_episode_nfo(data: &NFOData, season: i32, episode: i32) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    xml.push('\n');
    xml.push_str(r#"<episodedetails xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#);
    xml.push('\n');
    
    if let Some(ref title) = data.title {
        xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    }
    
    xml.push_str(&format!("  <season>{}</season>\n", season));
    xml.push_str(&format!("  <episode>{}</episode>\n", episode));
    
    if let Some(ref year) = data.year {
        xml.push_str(&format!("  <year>{}</year>\n", escape_xml(year)));
    }
    
    if let Some(rating) = data.rating {
        xml.push_str(&format!("  <rating>{:.1}</rating>\n", rating));
    }
    
    if let Some(ref plot) = data.plot {
        xml.push_str(&format!("  <plot>{}</plot>\n", escape_xml(plot)));
    }
    
    for actor in &data.actors {
        xml.push_str("  <actor>\n");
        xml.push_str(&format!("    <name>{}</name>\n", escape_xml(&actor.name)));
        if let Some(ref role) = actor.role {
            xml.push_str(&format!("    <role>{}</role>\n", escape_xml(role)));
        }
        xml.push_str("  </actor>\n");
    }
    
    xml.push_str("</episodedetails>");
    xml
}

/// 从Movie生成NFO（兼容旧接口）
pub fn generate_nfo_from_movie(movie: &Movie) -> String {
    let data = NFOData::from(movie);
    if movie.video_type == "tv" {
        generate_tvshow_nfo(&data)
    } else {
        generate_movie_nfo(&data)
    }
}

/// 保存NFO文件
pub fn save_nfo(path: &Path, content: &str) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| format!("创建文件失败: {}", e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("写入文件失败: {}", e))?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(())
}

/// XML转义
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// 验证NFO文件
pub fn validate_nfo(content: &str) -> Result<(), String> {
    if !content.starts_with("<?xml") {
        return Err("NFO文件必须以XML声明开头".to_string());
    }
    
    if !content.contains("<movie>") && !content.contains("<tvshow>") && !content.contains("<episodedetails>") {
        return Err("NFO文件必须包含movie、tvshow或episodedetails根元素".to_string());
    }
    
    Ok(())
}

/// 解析NFO文件（简单解析）
pub fn parse_nfo(content: &str) -> Result<NFOData, String> {
    let mut data = NFOData::default();
    
    // 简单提取标签内容
    for line in content.lines() {
        let line = line.trim();
        
        if let Some(val) = extract_tag(line, "title") {
            data.title = Some(val);
        } else if let Some(val) = extract_tag(line, "originaltitle") {
            data.original_title = Some(val);
        } else if let Some(val) = extract_tag(line, "year") {
            data.year = Some(val);
        } else if let Some(val) = extract_tag(line, "plot") {
            data.plot = Some(val);
        } else if let Some(val) = extract_tag(line, "rating") {
            data.rating = val.parse().ok();
        } else if let Some(val) = extract_tag(line, "id") {
            data.imdb_id = Some(val);
        }
    }
    
    Ok(data)
}

/// 提取XML标签内容
fn extract_tag(line: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if line.contains(&start_tag) && line.contains(&end_tag) {
        let start = line.find(&start_tag)? + start_tag.len();
        let end = line.find(&end_tag)?;
        Some(line[start..end].to_string())
    } else {
        None
    }
}