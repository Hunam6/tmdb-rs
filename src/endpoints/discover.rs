use strum::Display;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
use crate::{CountryCode, Date, Language, Page};

/// the movie discover sort orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum MovieSort {
    #[strum(serialize = "original_title.asc")]
    OriginalTitleAsc,
    #[strum(serialize = "original_title.desc")]
    OriginalTitleDesc,
    #[strum(serialize = "popularity.asc")]
    PopularityAsc,
    #[strum(serialize = "popularity.desc")]
    PopularityDesc,
    #[strum(serialize = "revenue.asc")]
    RevenueAsc,
    #[strum(serialize = "revenue.desc")]
    RevenueDesc,
    #[strum(serialize = "primary_release_date.asc")]
    PrimaryReleaseDateAsc,
    #[strum(serialize = "primary_release_date.desc")]
    PrimaryReleaseDateDesc,
    #[strum(serialize = "title.asc")]
    TitleAsc,
    #[strum(serialize = "title.desc")]
    TitleDesc,
    #[strum(serialize = "vote_average.asc")]
    VoteAverageAsc,
    #[strum(serialize = "vote_average.desc")]
    VoteAverageDesc,
    #[strum(serialize = "vote_count.asc")]
    VoteCountAsc,
    #[strum(serialize = "vote_count.desc")]
    VoteCountDesc,
}

/// the series discover sort orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum TvSort {
    #[strum(serialize = "first_air_date.asc")]
    FirstAirDateAsc,
    #[strum(serialize = "first_air_date.desc")]
    FirstAirDateDesc,
    #[strum(serialize = "name.asc")]
    NameAsc,
    #[strum(serialize = "name.desc")]
    NameDesc,
    #[strum(serialize = "original_name.asc")]
    OriginalNameAsc,
    #[strum(serialize = "original_name.desc")]
    OriginalNameDesc,
    #[strum(serialize = "popularity.asc")]
    PopularityAsc,
    #[strum(serialize = "popularity.desc")]
    PopularityDesc,
    #[strum(serialize = "vote_average.asc")]
    VoteAverageAsc,
    #[strum(serialize = "vote_average.desc")]
    VoteAverageDesc,
    #[strum(serialize = "vote_count.asc")]
    VoteCountAsc,
    #[strum(serialize = "vote_count.desc")]
    VoteCountDesc,
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
