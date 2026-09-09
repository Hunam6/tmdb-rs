use serde::Deserialize;

use crate::endpoints::search::PersonShort;
use crate::{Backdrop, Poster, Still};

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

/// the movie or series a credit points at
#[derive(Debug, Clone, Deserialize)]
pub struct CreditMedia {
    pub id: u64,
    /// movies
    pub title: Option<String>,
    pub original_title: Option<String>,
    /// series
    pub name: Option<String>,
    pub original_name: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    pub media_type: Option<String>,
    #[serde(default)]
    pub seasons: Vec<CreditSeason>,
    #[serde(default)]
    pub episodes: Vec<CreditEpisode>,
    pub character: Option<String>,
}

/// a single credit by its credit id
#[derive(Debug, Clone, Deserialize)]
pub struct CreditDetails {
    pub id: String,
    pub credit_type: String,
    pub department: String,
    pub job: String,
    pub media: CreditMedia,
    pub person: PersonShort,
}

endpoint! {
    /// a credit by its credit id
    credit(id: &str): GET "/credit/{}" => CreditDetails
}
