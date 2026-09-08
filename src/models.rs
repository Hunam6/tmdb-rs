use serde::Deserialize;
use time::Date;

use crate::append::appendable;
use crate::datetime::opt_date;

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
    pub id: u64,
    pub keywords: Vec<Keyword>,
}

/// `/tv/{id}/keywords` — same payload, different envelope
#[derive(Debug, Clone, Deserialize)]
pub struct TvKeywords {
    pub id: u64,
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
    pub id: u64,
    pub results: Vec<Video>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub vote_average: f64,
    pub vote_count: u32,
    pub iso_639_1: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Images {
    pub id: u64,
    #[serde(default)]
    pub backdrops: Vec<Image>,
    #[serde(default)]
    pub posters: Vec<Image>,
    #[serde(default)]
    pub logos: Vec<Image>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CastMember {
    pub id: u64,
    pub name: String,
    pub character: Option<String>,
    pub profile_path: Option<String>,
    pub order: Option<u32>,
    pub credit_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrewMember {
    pub id: u64,
    pub name: String,
    pub job: String,
    pub department: String,
    pub profile_path: Option<String>,
    pub credit_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credits {
    pub id: u64,
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
    pub iso_3166_1: String,
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDates {
    pub id: u64,
    pub results: Vec<CountryReleaseDates>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentRating {
    pub iso_3166_1: String,
    pub rating: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentRatings {
    pub id: u64,
    pub results: Vec<ContentRating>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExternalIds {
    pub id: u64,
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
    pub id: u64,
    pub season_number: u32,
    pub name: String,
    pub episode_count: u32,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
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
}
