//! the v4 api: auth flow, list CRUD, account lists
//!
//! reached via [`Client::v4`]; every call authenticates with the v4 user
//! access token, never the read token

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::endpoints::movie::MovieShort;
use crate::endpoints::search::MultiResult;
use crate::endpoints::tv::TvShort;
use crate::models::{ListShort, MediaType, StatusResponse};
use crate::{AccessToken, Client, Language, Page, Result};

/// the v4 api surface; get one from [`Client::v4`]
#[derive(Clone)]
pub struct V4(Client);

impl V4 {
    pub(crate) fn new(client: Client) -> Self {
        Self(client)
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        self.0.get(path, query).await
    }

    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        body: &serde_json::Value,
    ) -> Result<T> {
        self.0.post(path, query, body).await
    }

    pub(crate) async fn put<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        body: &serde_json::Value,
    ) -> Result<T> {
        self.0.put(path, query, body).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        body: &serde_json::Value,
    ) -> Result<T> {
        self.0.delete(path, query, body).await
    }
}

/// the v4 list sort orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum V4Sort {
    #[serde(rename = "original_order.asc")]
    OriginalOrderAsc,
    #[serde(rename = "original_order.desc")]
    OriginalOrderDesc,
    #[serde(rename = "title.asc")]
    TitleAsc,
    #[serde(rename = "title.desc")]
    TitleDesc,
    #[serde(rename = "release_date.asc")]
    ReleaseDateAsc,
    #[serde(rename = "release_date.desc")]
    ReleaseDateDesc,
    #[serde(rename = "vote_average.asc")]
    VoteAverageAsc,
    #[serde(rename = "vote_average.desc")]
    VoteAverageDesc,
    #[serde(rename = "created_at.asc")]
    CreatedAtAsc,
    #[serde(rename = "created_at.desc")]
    CreatedAtDesc,
}

impl crate::ToParam for V4Sort {
    fn to_param(&self) -> String {
        crate::param::serde_param(self)
    }
}

/// step 1's answer: the request token to send the user to approve
#[derive(Debug, Clone, Deserialize)]
pub struct V4RequestToken {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
    pub request_token: String,
}

impl V4RequestToken {
    /// the url to send the user to so they can approve the token
    pub fn redirect_url(&self) -> String {
        format!(
            "https://www.themoviedb.org/auth/access?request_token={}",
            self.request_token
        )
    }
}

/// step 2's answer: the user access token every other v4 call needs
#[derive(Debug, Clone, Deserialize)]
pub struct V4AccessToken {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
    pub account_id: String,
    pub access_token: String,
}

impl V4AccessToken {
    pub fn token(&self) -> AccessToken {
        AccessToken::new(self.access_token.clone())
    }
}

/// a v4 list and one page of its items
#[derive(Debug, Clone, Deserialize)]
pub struct V4List {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub iso_639_1: Option<String>,
    pub iso_3166_1: Option<String>,
    pub item_count: u32,
    pub average_rating: Option<f64>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub created_by: Option<String>,
    pub sort_by: Option<String>,
    pub page: u32,
    pub total_pages: u32,
    pub total_results: u64,
    #[serde(default)]
    pub results: Vec<MultiResult>,
}

/// one item in an add/remove/update items call
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ListItem {
    pub media_type: MediaType,
    pub media_id: u64,
}

/// per-item outcome of an items call
#[derive(Debug, Clone, Deserialize)]
pub struct ItemResult {
    pub media_id: u64,
    pub media_type: String,
    pub success: bool,
    #[serde(default)]
    pub error: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemsResponse {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
    #[serde(default)]
    pub results: Vec<ItemResult>,
}

/// the answer of a v4 list creation
#[derive(Debug, Clone, Deserialize)]
pub struct V4CreateListResponse {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
    pub id: Option<u64>,
}

endpoint! {
    @gen V4,
    /// v4 auth, step 1: a request token for the user to approve
    v4_request_token(): POST "/auth/request_token" => V4RequestToken {
        body { redirect_to: &str }
    }
}

endpoint! {
    @gen V4,
    /// v4 auth, step 2: exchange an approved request token for an access token
    v4_access_token(): POST "/auth/access_token" => V4AccessToken {
        body { request_token: &str }
    }
}

endpoint! {
    @gen V4,
    /// log out: delete an access token
    v4_delete_access_token(): DELETE "/auth/access_token" => StatusResponse {
        body { access_token: &str }
    }
}

endpoint! {
    @gen V4,
    /// a list and one page of its items
    v4_list(id: u64): GET "/list/{}" => V4List {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// create a list
    v4_create_list(): POST "/list" => V4CreateListResponse {
        body { name: &str, iso_639_1: &str, description: &str, public: bool }
    }
}

endpoint! {
    @gen V4,
    /// rename or re-describe a list
    v4_update_list(id: u64): PUT "/list/{}" => StatusResponse {
        body { name: &str, description: &str, public: bool }
    }
}

endpoint! {
    @gen V4,
    /// delete a list
    v4_delete_list(id: u64): DELETE "/list/{}" => StatusResponse {
        body {}
    }
}

endpoint! {
    @gen V4,
    /// add items to a list
    v4_list_add_items(id: u64): POST "/list/{}/items" => ItemsResponse {
        body { items: Vec<ListItem> }
    }
}

endpoint! {
    @gen V4,
    /// remove items from a list
    v4_list_remove_items(id: u64): DELETE "/list/{}/items" => ItemsResponse {
        body { items: Vec<ListItem> }
    }
}

endpoint! {
    @gen V4,
    /// remove every item from a list
    v4_list_clear(id: u64): GET "/list/{}/clear" => StatusResponse
}

endpoint! {
    @gen V4,
    /// the account's own lists
    v4_account_lists(account_id: &str): GET "/account/{}/lists" => Page<ListShort> {
        params { page: u32 }
    }
}

endpoint! {
    @gen V4,
    /// the account's favorite movies
    v4_favorite_movies(account_id: &str): GET "/account/{}/movie/favorites" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's favorite series
    v4_favorite_tv(account_id: &str): GET "/account/{}/tv/favorites" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's movie recommendations
    v4_movie_recommendations(account_id: &str): GET "/account/{}/movie/recommendations" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's series recommendations
    v4_tv_recommendations(account_id: &str): GET "/account/{}/tv/recommendations" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's movie watchlist
    v4_watchlist_movies(account_id: &str): GET "/account/{}/movie/watchlist" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's series watchlist
    v4_watchlist_tv(account_id: &str): GET "/account/{}/tv/watchlist" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's rated movies
    v4_rated_movies(account_id: &str): GET "/account/{}/movie/rated" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// the account's rated series
    v4_rated_tv(account_id: &str): GET "/account/{}/tv/rated" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: V4Sort }
    }
}

endpoint! {
    @gen V4,
    /// mark or unmark a favorite
    v4_mark_favorite(account_id: &str): POST "/account/{}/favorites" => StatusResponse {
        body { media_type: MediaType, media_id: u64, favorite: bool }
    }
}

endpoint! {
    @gen V4,
    /// add or remove a watchlist entry
    v4_set_watchlist(account_id: &str): POST "/account/{}/watchlist" => StatusResponse {
        body { media_type: MediaType, media_id: u64, watchlist: bool }
    }
}
