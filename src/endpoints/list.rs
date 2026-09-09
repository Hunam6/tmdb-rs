use serde::Deserialize;

use crate::endpoints::search::MultiResult;
use crate::models::StatusResponse;
use crate::{Language, Poster, SessionId};

/// the kind of items a v3 list holds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    Movie,
    Tv,
}

/// one list in a list-of-lists response
#[derive(Debug, Clone, Deserialize)]
pub struct ListShort {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub item_count: u32,
    #[serde(rename = "iso_639_1")]
    pub language: Option<Language>,
    pub list_type: Option<ListType>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    pub favorite: Option<bool>,
}

/// a v3 list and its items
#[derive(Debug, Clone, Deserialize)]
pub struct ListDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_count: u32,
    #[serde(rename = "iso_639_1")]
    pub language: Option<Language>,
    pub list_type: Option<ListType>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    pub favorite: bool,
    #[serde(default)]
    pub items: Vec<MultiResult>,
    #[serde(default)]
    pub total_pages: u32,
    #[serde(default)]
    pub total_results: u64,
    #[serde(default)]
    pub page: u32,
}

/// the answer of `item_status`
#[derive(Debug, Clone, Deserialize)]
pub struct ItemStatus {
    pub id: String,
    pub item_present: bool,
    pub media_type: Option<String>,
    pub media_id: Option<u64>,
}

/// the answer of a list creation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateListResponse {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
    pub list_id: Option<u64>,
}

endpoint! {
    /// a list and its items
    list(id: u64, session_id: SessionId): GET "/list/{id}" => ListDetails {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// whether a title is on a list
    list_item_status(id: u64, session_id: SessionId): GET "/list/{id}/item_status" => ItemStatus {
        params { movie_id: u64 }
    }
}

endpoint! {
    /// create a list
    create_list(session_id: SessionId): POST "/list" => CreateListResponse {
        body { name: &str, description: &str, language: &str }
    }
}

endpoint! {
    /// add a title to a list
    list_add_item(id: u64, session_id: SessionId): POST "/list/{id}/add_item" => StatusResponse {
        body { media_id: u64 }
    }
}

endpoint! {
    /// remove a title from a list
    list_remove_item(id: u64, session_id: SessionId): POST "/list/{id}/remove_item" => StatusResponse {
        body { media_id: u64 }
    }
}

endpoint! {
    /// remove every title from a list
    list_clear(id: u64, session_id: SessionId): POST "/list/{id}/clear" => StatusResponse {
        body { confirm: bool }
    }
}

endpoint! {
    /// delete a list
    list_delete(id: u64, session_id: SessionId): DELETE "/list/{id}" => StatusResponse
}
