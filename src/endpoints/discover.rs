use std::fmt;

use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
use crate::{Country, Date, Language, Page};

macro_rules! sort {
    ($(#[$meta:meta])* $name:ident { $($variant:ident = $key:literal),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant,)*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $key,)*
                })
            }
        }

        impl crate::ToParam for $name {
            fn to_param(&self) -> String {
                self.to_string()
            }
        }
    };
}

sort! {
    /// the movie discover sort orders
    MovieSort {
        OriginalTitleAsc = "original_title.asc",
        OriginalTitleDesc = "original_title.desc",
        PopularityAsc = "popularity.asc",
        PopularityDesc = "popularity.desc",
        RevenueAsc = "revenue.asc",
        RevenueDesc = "revenue.desc",
        PrimaryReleaseDateAsc = "primary_release_date.asc",
        PrimaryReleaseDateDesc = "primary_release_date.desc",
        TitleAsc = "title.asc",
        TitleDesc = "title.desc",
        VoteAverageAsc = "vote_average.asc",
        VoteAverageDesc = "vote_average.desc",
        VoteCountAsc = "vote_count.asc",
        VoteCountDesc = "vote_count.desc",
    }
}

sort! {
    /// the series discover sort orders
    TvSort {
        FirstAirDateAsc = "first_air_date.asc",
        FirstAirDateDesc = "first_air_date.desc",
        NameAsc = "name.asc",
        NameDesc = "name.desc",
        OriginalNameAsc = "original_name.asc",
        OriginalNameDesc = "original_name.desc",
        PopularityAsc = "popularity.asc",
        PopularityDesc = "popularity.desc",
        VoteAverageAsc = "vote_average.asc",
        VoteAverageDesc = "vote_average.desc",
        VoteCountAsc = "vote_count.asc",
        VoteCountDesc = "vote_count.desc",
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
            region: Country,
            watch_region: Country,
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
            with_origin_country: Country,
            with_original_language: Language,
            certification: &str,
            certification_country: Country,
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
            watch_region: Country,
            with_watch_providers: Vec<u64>,
            with_watch_monetization_types: &str,
            with_genres: Vec<u64>,
            without_genres: Vec<u64>,
            with_keywords: Vec<u64>,
            without_keywords: Vec<u64>,
            with_companies: Vec<u64>,
            with_networks: Vec<u64>,
            with_origin_country: Country,
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
