use serde::Deserialize;
use time::Date;

use crate::datetime::opt_date;
use crate::models::{
    ContentRatings, Credits, ExternalIds, Genre, Images, SeasonShort, TvKeywords, Videos,
};
use crate::{Language, Page};

/// one series in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct TvShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub overview: String,
    #[serde(default, deserialize_with = "opt_date")]
    pub first_air_date: Option<Date>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub original_language: Option<String>,
    #[serde(default)]
    pub origin_country: Vec<String>,
    pub adult: bool,
}

/// a series' next aired episode
#[derive(Debug, Clone, Deserialize)]
pub struct NextEpisode {
    pub id: u64,
    pub name: String,
    pub episode_number: u32,
    pub season_number: u32,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
}

/// one episode inside a season
#[derive(Debug, Clone, Deserialize)]
pub struct Episode {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub episode_number: u32,
    pub season_number: u32,
    pub runtime: Option<u32>,
    pub still_path: Option<String>,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
    pub vote_average: f64,
    pub vote_count: u32,
}

details! {
    /// the primary details of a series
    tv(id: u64): GET "/tv/{}" => TvDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub original_name: String,
            pub overview: String,
            pub tagline: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub first_air_date: Option<Date>,
            #[serde(default, deserialize_with = "opt_date")]
            pub last_air_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
            pub popularity: f64,
            pub number_of_seasons: u32,
            pub number_of_episodes: u32,
            pub episode_run_time: Vec<u32>,
            pub original_language: Option<String>,
            pub poster_path: Option<String>,
            pub backdrop_path: Option<String>,
            pub genres: Vec<Genre>,
            pub homepage: String,
            pub status: String,
            pub in_production: bool,
            #[serde(default)]
            pub languages: Vec<String>,
            #[serde(default)]
            pub origin_country: Vec<String>,
            #[serde(default)]
            pub seasons: Vec<SeasonShort>,
            pub next_episode_to_air: Option<NextEpisode>,
            pub adult: bool,
        }
        appends {
            content_ratings: ContentRatings,
            credits: Credits,
            similar: Page<TvShort>,
            recommendations: Page<TvShort>,
            keywords: TvKeywords,
            external_ids: ExternalIds,
            videos: Videos,
            images: Images,
        }
    }
}

details! {
    /// one season of a series, episodes included
    tv_season(id: u64, season: u32): GET "/tv/{}/season/{}" => SeasonDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub season_number: u32,
            pub name: String,
            pub overview: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub air_date: Option<Date>,
            pub poster_path: Option<String>,
            #[serde(default)]
            pub episodes: Vec<Episode>,
        }
        appends {
            credits: Credits,
            external_ids: ExternalIds,
            images: Images,
            videos: Videos,
        }
    }
}

details! {
    /// one episode of a series
    tv_episode(id: u64, season: u32, episode: u32): GET "/tv/{}/season/{}/episode/{}" => EpisodeDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub overview: String,
            pub episode_number: u32,
            pub season_number: u32,
            pub runtime: Option<u32>,
            pub still_path: Option<String>,
            #[serde(default, deserialize_with = "opt_date")]
            pub air_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
        }
        appends {
            credits: Credits,
            external_ids: ExternalIds,
            images: Images,
            videos: Videos,
        }
    }
}

endpoint! {
    /// the newest series
    tv_latest(): GET "/tv/latest" => TvDetails {
        params { language: Language }
    }
}

endpoint! {
    /// series ordered by popularity
    tv_popular(): GET "/tv/popular" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series ordered by rating
    tv_top_rated(): GET "/tv/top_rated" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series with an episode airing in the next week
    tv_on_the_air(): GET "/tv/on_the_air" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series with an episode airing today
    tv_airing_today(): GET "/tv/airing_today" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a series' keywords, without the details round-trip
    tv_keywords(id: u64): GET "/tv/{}/keywords" => TvKeywords
}

endpoint! {
    /// a series' ids on other databases
    tv_external_ids(id: u64): GET "/tv/{}/external_ids" => ExternalIds
}
