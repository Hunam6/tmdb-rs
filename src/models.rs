use std::collections::HashMap;

use serde::Deserialize;
use time::Date;

use crate::append::appendable;
use crate::datetime::opt_date;
use crate::{Backdrop, CountryCode, Language, Logo, Poster, Profile};

#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}

/// `/movie/{id}/keywords`
#[derive(Debug, Clone, Deserialize)]
pub struct MovieKeywords {
    pub keywords: Vec<Keyword>,
}

/// `/tv/{id}/keywords` — same payload, different envelope
#[derive(Debug, Clone, Deserialize)]
pub struct TvKeywords {
    pub results: Vec<Keyword>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    pub id: String,
    pub key: String,
    pub name: String,
    pub site: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: u32,
    pub official: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Videos {
    pub results: Vec<Video>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image<K> {
    #[serde(rename = "file_path")]
    pub file: K,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub vote_average: f64,
    pub vote_count: u32,
    /// textless images have no language
    #[serde(rename = "iso_639_1")]
    pub language: Option<Language>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Images {
    pub backdrops: Vec<Image<Backdrop>>,
    pub posters: Vec<Image<Poster>>,
    pub logos: Vec<Image<Logo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CastMember {
    pub name: String,
    pub character: Option<String>,
    #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
    pub order: Option<u32>,
    pub credit_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrewMember {
    pub name: String,
    pub job: String,
    pub department: String,
    #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
    pub credit_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
}

/// TMDB's release date kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(from = "u32")]
pub enum ReleaseType {
    Premiere,
    TheatricalLimited,
    Theatrical,
    Digital,
    Physical,
    Tv,
    Unknown(u32),
}

impl From<u32> for ReleaseType {
    fn from(kind: u32) -> Self {
        match kind {
            1 => Self::Premiere,
            2 => Self::TheatricalLimited,
            3 => Self::Theatrical,
            4 => Self::Digital,
            5 => Self::Physical,
            6 => Self::Tv,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDate {
    pub certification: String,
    #[serde(rename = "type")]
    pub kind: ReleaseType,
    /// ISO 8601 datetime
    pub release_date: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CountryReleaseDates {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDates {
    pub results: Vec<CountryReleaseDates>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentRating {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub rating: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentRatings {
    pub results: Vec<ContentRating>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExternalIds {
    pub imdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub tvdb_id: Option<u64>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub twitter_id: Option<String>,
}

/// one season inside a series' details
#[derive(Debug, Clone, Deserialize)]
pub struct SeasonShort {
    pub season_number: u32,
    pub name: String,
    pub episode_count: u32,
    pub overview: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
}

/// movie or series, for write-endpoint bodies
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Movie,
    Tv,
}

/// the success envelope of write endpoints
#[derive(Debug, Clone, Deserialize)]
pub struct StatusResponse {
    pub success: bool,
    pub status_code: i32,
    pub status_message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Translation {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    #[serde(rename = "iso_639_1")]
    pub language: Language,
    pub name: String,
    pub english_name: String,
    pub data: TranslationData,
}

/// movies fill `title`, series `name`
#[derive(Debug, Clone, Deserialize)]
pub struct TranslationData {
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub homepage: Option<String>,
    pub tagline: Option<String>,
}

impl TranslationData {
    /// the translated title, whichever of the movie/series keys it came in
    pub fn display_title(&self) -> Option<&str> {
        self.title.as_deref().or(self.name.as_deref())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Translations {
    pub translations: Vec<Translation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlternativeTitle {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
}

/// `/movie/{id}/alternative_titles`
#[derive(Debug, Clone, Deserialize)]
pub struct AlternativeTitles {
    pub titles: Vec<AlternativeTitle>,
}

/// `/tv/{id}/alternative_titles` — same payload, different envelope
#[derive(Debug, Clone, Deserialize)]
pub struct AlternativeTitleResults {
    pub results: Vec<AlternativeTitle>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlternativeName {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
}

/// company and network alternative names
#[derive(Debug, Clone, Deserialize)]
pub struct AlternativeNames {
    pub results: Vec<AlternativeName>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeItem {
    pub id: String,
    pub action: String,
    pub time: String,
    #[serde(rename = "iso_639_1")]
    pub language: Option<Language>,
    #[serde(rename = "iso_3166_1")]
    pub country: Option<CountryCode>,
    /// free-form: the shape depends on the changed key
    pub value: serde_json::Value,
    pub original_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Change {
    pub key: String,
    pub items: Vec<ChangeItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Changes {
    pub changes: Vec<Change>,
}

/// `rated` is `false` until the user rates the title, then an object
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub enum Rated {
    Unrated(bool),
    Rated { value: f64 },
}

impl Rated {
    pub fn value(&self) -> Option<f64> {
        match self {
            Self::Unrated(_) => None,
            Self::Rated { value } => Some(*value),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountStates {
    pub favorite: bool,
    pub watchlist: bool,
    pub rated: Rated,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchProvider {
    pub provider_id: u64,
    pub provider_name: String,
    #[serde(rename = "logo_path")]
    pub logo: Option<Logo>,
    pub display_priority: Option<u32>,
}

/// one country's watch options for a title
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryProviders {
    pub link: Option<String>,
    pub flatrate: Vec<WatchProvider>,
    pub rent: Vec<WatchProvider>,
    pub buy: Vec<WatchProvider>,
    pub ads: Vec<WatchProvider>,
    pub free: Vec<WatchProvider>,
}

/// the `watch/providers` payload, keyed by country code
#[derive(Debug, Clone, Deserialize)]
pub struct WatchProviders {
    pub results: HashMap<String, CountryProviders>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorDetails {
    pub name: String,
    pub username: String,
    pub avatar_path: Option<String>,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Review {
    pub id: String,
    pub author: String,
    pub author_details: AuthorDetails,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
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
    pub list_type: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    pub favorite: Option<bool>,
}

appendable! {
    Credits,
    ReleaseDates,
    ContentRatings,
    MovieKeywords,
    TvKeywords,
    ExternalIds,
    Videos,
    Images,
    Translations,
    AlternativeTitles,
    AlternativeTitleResults,
    Changes,
    AccountStates,
    WatchProviders,
}
