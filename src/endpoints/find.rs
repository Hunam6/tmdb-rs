// the derived Display still names the dead variants so old data keeps parsing
#![allow(deprecated)]

use serde::Deserialize;
use strum::Display;
use time::Date;

use crate::datetime::opt_date;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::person::PersonShort;
use crate::endpoints::tv::TvShort;
use crate::{Language, Poster, Still};

/// the database an external id comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum ExternalSource {
    #[strum(serialize = "imdb_id")]
    Imdb,
    #[deprecated = "freebase shut down in 2016; TMDB no longer supports this source"]
    #[strum(serialize = "freebase_mid")]
    FreebaseMid,
    #[deprecated = "freebase shut down in 2016; TMDB no longer supports this source"]
    #[strum(serialize = "freebase_id")]
    Freebase,
    #[strum(serialize = "tvdb_id")]
    Tvdb,
    #[deprecated = "tvrage shut down; TMDB no longer supports this source"]
    #[strum(serialize = "tvrage_id")]
    Tvrage,
    #[strum(serialize = "facebook_id")]
    Facebook,
    #[strum(serialize = "instagram_id")]
    Instagram,
    #[strum(serialize = "threads_id")]
    Threads,
    #[strum(serialize = "tiktok_id")]
    Tiktok,
    #[strum(serialize = "twitter_id")]
    Twitter,
    #[strum(serialize = "wikidata_id")]
    Wikidata,
    #[strum(serialize = "youtube_id")]
    Youtube,
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
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
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
    #[serde(rename = "still_path")]
    pub still: Option<Still>,
    pub vote_average: f64,
    pub vote_count: u32,
}

/// everything matching one external id
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FindResults {
    pub movie_results: Vec<MovieShort>,
    pub tv_results: Vec<TvShort>,
    pub person_results: Vec<PersonShort>,
    pub tv_episode_results: Vec<FoundEpisode>,
    pub tv_season_results: Vec<FoundSeason>,
}

endpoint! {
    /// find TMDB entries by an external id (imdb, tvdb, wikidata, ...)
    find(external_id: &str, external_source: ExternalSource): GET "/find/{external_id}" => FindResults {
        params { language: Language }
    }
}
