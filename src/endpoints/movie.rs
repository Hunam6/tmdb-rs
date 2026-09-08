use serde::Deserialize;
use time::Date;

use crate::datetime::opt_date;
use crate::models::{
    Credits, ExternalIds, Genre, Images, MovieKeywords, ReleaseDates, Videos,
};
use crate::{Country, Language, Page};

/// one movie in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct MovieShort {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub overview: String,
    #[serde(default, deserialize_with = "opt_date")]
    pub release_date: Option<Date>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub original_language: Option<String>,
    pub adult: bool,
    pub video: bool,
}

details! {
    /// the primary details of a movie
    movie(id: u64): GET "/movie/{}" => MovieDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub title: String,
            pub original_title: String,
            pub overview: String,
            pub tagline: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub release_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
            pub popularity: f64,
            pub runtime: Option<u32>,
            pub original_language: Option<String>,
            pub poster_path: Option<String>,
            pub backdrop_path: Option<String>,
            pub genres: Vec<Genre>,
            pub imdb_id: Option<String>,
            pub homepage: String,
            pub status: String,
            pub budget: u64,
            pub revenue: u64,
            pub adult: bool,
        }
        appends {
            credits: Credits,
            release_dates: ReleaseDates,
            similar: Page<MovieShort>,
            recommendations: Page<MovieShort>,
            keywords: MovieKeywords,
            external_ids: ExternalIds,
            videos: Videos,
            images: Images,
        }
    }
}

endpoint! {
    /// the current movie ids, paged by insertion order
    movie_changes(): GET "/movie/changes" => Page<MovieChange> {
        params { page: u32, start_date: Date, end_date: Date }
    }
}

/// a movie touched by a change window
#[derive(Debug, Clone, Deserialize)]
pub struct MovieChange {
    pub id: u64,
    pub adult: Option<bool>,
}

endpoint! {
    /// the newest movie
    movie_latest(): GET "/movie/latest" => MovieDetails {
        params { language: Language }
    }
}

endpoint! {
    /// movies currently in theatres
    movie_now_playing(): GET "/movie/now_playing" => Page<MovieShort> {
        params { language: Language, page: u32, region: Country }
    }
}

endpoint! {
    /// movies ordered by popularity
    movie_popular(): GET "/movie/popular" => Page<MovieShort> {
        params { language: Language, page: u32, region: Country }
    }
}

endpoint! {
    /// movies ordered by rating
    movie_top_rated(): GET "/movie/top_rated" => Page<MovieShort> {
        params { language: Language, page: u32, region: Country }
    }
}

endpoint! {
    /// upcoming movies
    movie_upcoming(): GET "/movie/upcoming" => Page<MovieShort> {
        params { language: Language, page: u32, region: Country }
    }
}

endpoint! {
    /// a movie's keywords, without the details round-trip
    movie_keywords(id: u64): GET "/movie/{}/keywords" => MovieKeywords
}

endpoint! {
    /// a movie's ids on other databases
    movie_external_ids(id: u64): GET "/movie/{}/external_ids" => ExternalIds
}
