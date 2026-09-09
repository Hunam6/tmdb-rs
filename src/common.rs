use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use time::macros::format_description;
use time::Date;

use crate::append::appendable;
use crate::endpoints::credit::Credits;
use crate::endpoints::keyword::{MovieKeywords, TvKeywords};
use crate::endpoints::movie::ReleaseDates;
use crate::{Backdrop, CountryCode, Language, Logo, Poster};

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
pub struct ExternalIds {
    pub imdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub tvdb_id: Option<u64>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub twitter_id: Option<String>,
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
    #[serde(rename = "homepage")]
    pub website: Option<String>,
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
    #[serde(rename = "title")]
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
}

/// `/movie` or `/tv` `alternative_titles` — movies wrap it in `titles`, series in `results`
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "RawAlternativeTitles")]
pub struct AlternativeTitles {
    pub titles: Vec<AlternativeTitle>,
}

#[derive(Deserialize)]
struct RawAlternativeTitles {
    titles: Option<Vec<AlternativeTitle>>,
    results: Option<Vec<AlternativeTitle>>,
}

impl From<RawAlternativeTitles> for AlternativeTitles {
    fn from(raw: RawAlternativeTitles) -> Self {
        Self {
            titles: raw.titles.or(raw.results).unwrap_or_default(),
        }
    }
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

appendable! {
    Credits,
    ReleaseDates,
    MovieKeywords,
    TvKeywords,
    ExternalIds,
    Videos,
    Images,
    Translations,
    AlternativeTitles,
    Changes,
    AccountStates,
    WatchProviders,
}

/// one page of a paginated list endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct Page<T> {
    pub page: u32,
    pub results: Vec<T>,
    pub total_pages: u32,
    pub total_results: u32,
}

impl<T> Page<T> {
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }
}

/// TMDB writes unknown dates as "" rather than null
pub(crate) fn opt_date<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Date>, D::Error> {
    match Option::<String>::deserialize(d)?.filter(|s| !s.is_empty()) {
        Some(s) => Date::parse(&s, format_description!("[year]-[month]-[day]"))
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
