use serde::Deserialize;

use crate::endpoints::person::PersonShort;
use crate::{Backdrop, Poster, Profile, Still};

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

/// one season credited on an episode-level credit
#[derive(Debug, Clone, Deserialize)]
pub struct CreditSeason {
    pub season_number: u32,
    pub name: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    pub air_date: Option<String>,
}

/// one episode credited on an episode-level credit
#[derive(Debug, Clone, Deserialize)]
pub struct CreditEpisode {
    pub id: u64,
    pub name: String,
    pub episode_number: u32,
    pub season_number: u32,
    pub air_date: Option<String>,
    #[serde(rename = "still_path")]
    pub still: Option<Still>,
    pub overview: Option<String>,
}

/// the movie a credit points at
#[derive(Debug, Clone, Deserialize)]
pub struct CreditMovie {
    pub id: u64,
    #[serde(rename = "title")]
    pub name: String,
    #[serde(rename = "original_title")]
    pub original_name: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
}

/// the series a credit points at
#[derive(Debug, Clone, Deserialize)]
pub struct CreditSeries {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default)]
    pub seasons: Vec<CreditSeason>,
    #[serde(default)]
    pub episodes: Vec<CreditEpisode>,
    pub character: Option<String>,
}

/// the movie or series a credit points at, discriminated by TMDB's `media_type`
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "media_type", rename_all = "snake_case")]
pub enum CreditMedia {
    Movie(CreditMovie),
    Tv(CreditSeries),
}

/// cast (on screen) or crew (behind it)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditType {
    Cast,
    Crew,
}

/// a single credit by its credit id
#[derive(Debug, Clone, Deserialize)]
pub struct CreditDetails {
    pub id: String,
    pub credit_type: CreditType,
    pub department: String,
    pub job: String,
    pub media: CreditMedia,
    pub person: PersonShort,
}

endpoint! {
    /// a credit by its credit id
    credit(id: &str): GET "/credit/{id}" => CreditDetails
}
