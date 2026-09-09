use serde::Serialize;

use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
use crate::param::serde_param;
use crate::{CountryCode, Date, Language, Page, ToParam};

/// the movie discover sort orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MovieSort {
    #[serde(rename = "original_title.asc")]
    OriginalTitleAsc,
    #[serde(rename = "original_title.desc")]
    OriginalTitleDesc,
    #[serde(rename = "popularity.asc")]
    PopularityAsc,
    #[serde(rename = "popularity.desc")]
    PopularityDesc,
    #[serde(rename = "revenue.asc")]
    RevenueAsc,
    #[serde(rename = "revenue.desc")]
    RevenueDesc,
    #[serde(rename = "primary_release_date.asc")]
    PrimaryReleaseDateAsc,
    #[serde(rename = "primary_release_date.desc")]
    PrimaryReleaseDateDesc,
    #[serde(rename = "title.asc")]
    TitleAsc,
    #[serde(rename = "title.desc")]
    TitleDesc,
    #[serde(rename = "vote_average.asc")]
    VoteAverageAsc,
    #[serde(rename = "vote_average.desc")]
    VoteAverageDesc,
    #[serde(rename = "vote_count.asc")]
    VoteCountAsc,
    #[serde(rename = "vote_count.desc")]
    VoteCountDesc,
}

impl ToParam for MovieSort {
    fn to_param(&self) -> String {
        serde_param(self)
    }
}

/// the series discover sort orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TvSort {
    #[serde(rename = "first_air_date.asc")]
    FirstAirDateAsc,
    #[serde(rename = "first_air_date.desc")]
    FirstAirDateDesc,
    #[serde(rename = "name.asc")]
    NameAsc,
    #[serde(rename = "name.desc")]
    NameDesc,
    #[serde(rename = "original_name.asc")]
    OriginalNameAsc,
    #[serde(rename = "original_name.desc")]
    OriginalNameDesc,
    #[serde(rename = "popularity.asc")]
    PopularityAsc,
    #[serde(rename = "popularity.desc")]
    PopularityDesc,
    #[serde(rename = "vote_average.asc")]
    VoteAverageAsc,
    #[serde(rename = "vote_average.desc")]
    VoteAverageDesc,
    #[serde(rename = "vote_count.asc")]
    VoteCountAsc,
    #[serde(rename = "vote_count.desc")]
    VoteCountDesc,
}

impl ToParam for TvSort {
    fn to_param(&self) -> String {
        serde_param(self)
    }
}

endpoint! {
    /// discover movies by filters: genres, dates, ratings, providers, ...
    discover_movies(): GET "/discover/movie" => Page<MovieShort> {
        params {
            language: Language,
            page: u32,
            sort_by: MovieSort,
            include_adult: bool,
            include_video: bool,
            region: CountryCode,
            watch_region: CountryCode,
            with_watch_providers: Vec<u64>,
            with_watch_monetization_types: &str,
            with_genres: Vec<u64>,
            without_genres: Vec<u64>,
            with_keywords: Vec<u64>,
            without_keywords: Vec<u64>,
            with_cast: Vec<u64>,
            with_crew: Vec<u64>,
            with_people: Vec<u64>,
            with_companies: Vec<u64>,
            with_networks: Vec<u64>,
            with_origin_country: CountryCode,
            with_original_language: Language,
            certification: &str,
            certification_country: CountryCode,
            primary_release_year: u32,
            year: u32,
            vote_average_gte: f64,
            vote_average_lte: f64,
            vote_count_gte: f64,
            vote_count_lte: f64,
            with_runtime_gte: u32,
            with_runtime_lte: u32,
        }
    }
}

endpoint! {
    /// discover series by filters: genres, dates, ratings, networks, ...
    discover_tv(): GET "/discover/tv" => Page<TvShort> {
        params {
            language: Language,
            page: u32,
            sort_by: TvSort,
            include_adult: bool,
            include_null_first_air_dates: bool,
            watch_region: CountryCode,
            with_watch_providers: Vec<u64>,
            with_watch_monetization_types: &str,
            with_genres: Vec<u64>,
            without_genres: Vec<u64>,
            with_keywords: Vec<u64>,
            without_keywords: Vec<u64>,
            with_companies: Vec<u64>,
            with_networks: Vec<u64>,
            with_origin_country: CountryCode,
            with_original_language: Language,
            with_status: &str,
            with_type: &str,
            air_date_gte: Date,
            air_date_lte: Date,
            first_air_date_year: u32,
            vote_average_gte: f64,
            vote_average_lte: f64,
            vote_count_gte: f64,
            vote_count_lte: f64,
            with_runtime_gte: u32,
            with_runtime_lte: u32,
        }
    }
}
