use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<i64>,
    pub title: String,
    pub content_md: Option<String>,
    pub content_json: Option<String>,
    pub folder: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub reminder_at: Option<String>,
    pub pinned: bool,
    pub archived: bool,
    pub deleted_at: Option<String>,
    pub brand: String,
    pub backlinks_json: String,
    pub tags_cache: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub title: String,
    pub snippet: String,
    pub rank: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteFilter {
    pub folder: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
