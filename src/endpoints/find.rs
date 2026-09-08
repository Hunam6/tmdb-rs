use std::fmt;

use serde::Deserialize;
use time::Date;

use crate::datetime::opt_date;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::search::PersonShort;
use crate::endpoints::tv::TvShort;
use crate::Language;

/// the database an external id comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalSource {
    Imdb,
    FreebaseMid,
    Freebase,
    Tvdb,
    Tvrage,
    Facebook,
    Instagram,
    Threads,
    Tiktok,
    Twitter,
    Wikidata,
    Youtube,
}

impl fmt::Display for ExternalSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self {
            Self::Imdb => "imdb_id",
            Self::FreebaseMid => "freebase_mid",
            Self::Freebase => "freebase_id",
            Self::Tvdb => "tvdb_id",
            Self::Tvrage => "tvrage_id",
            Self::Facebook => "facebook_id",
            Self::Instagram => "instagram_id",
            Self::Threads => "threads_id",
            Self::Tiktok => "tiktok_id",
            Self::Twitter => "twitter_id",
            Self::Wikidata => "wikidata_id",
            Self::Youtube => "youtube_id",
        };
        f.write_str(source)
    }
}

/// one season found by external id
#[derive(Debug, Clone, Deserialize)]
pub struct FoundSeason {
    pub id: u64,
    pub show_id: u64,
    pub name: String,
    pub season_number: u32,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
    pub poster_path: Option<String>,
}

/// one episode found by external id
#[derive(Debug, Clone, Deserialize)]
pub struct FoundEpisode {
    pub id: u64,
    pub show_id: u64,
    pub name: String,
    pub episode_number: u32,
    pub season_number: u32,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u32,
}

impl crate::ToParam for ExternalSource {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

/// everything matching one external id
#[derive(Debug, Clone, Deserialize)]
pub struct FindResults {
    #[serde(default)]
    pub movie_results: Vec<MovieShort>,
    #[serde(default)]
    pub tv_results: Vec<TvShort>,
    #[serde(default)]
    pub person_results: Vec<PersonShort>,
    #[serde(default)]
    pub tv_episode_results: Vec<FoundEpisode>,
    #[serde(default)]
    pub tv_season_results: Vec<FoundSeason>,
}

endpoint! {
    /// find TMDB entries by an external id (imdb, tvdb, wikidata, ...)
    find(external_id: &str): GET "/find/{}" => FindResults {
        required { external_source: ExternalSource }
        params { language: Language }
    }
}
