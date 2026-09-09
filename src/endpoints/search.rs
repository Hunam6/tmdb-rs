use serde::Deserialize;

use crate::endpoints::collection::CollectionShort;
use crate::endpoints::company::CompanyShort;
use crate::endpoints::keyword::Keyword;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::person::PersonShort;
use crate::endpoints::tv::TvShort;
use crate::{CountryCode, Language, Page};

/// one multi-search hit, discriminated by TMDB's `media_type`
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "media_type", rename_all = "snake_case")]
pub enum MultiResult {
    Movie(MovieShort),
    Tv(TvShort),
    Person(PersonShort),
}

endpoint! {
    /// search for movies by title
    search_movies(query: &str): GET "/search/movie" => Page<MovieShort> {
        params {
            language: Language,
            page: u32,
            include_adult: bool,
            region: CountryCode,
            year: u32,
            primary_release_year: u32,
        }
    }
}

endpoint! {
    /// search for series by name
    search_tv(query: &str): GET "/search/tv" => Page<TvShort> {
        params {
            language: Language,
            page: u32,
            include_adult: bool,
            year: u32,
            first_air_date_year: u32,
        }
    }
}

endpoint! {
    /// search movies, series and people in one call
    search(query: &str): GET "/search/multi" => Page<MultiResult> {
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for people by name
    search_people(query: &str): GET "/search/person" => Page<PersonShort> {
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for collections by name
    search_collections(query: &str): GET "/search/collection" => Page<CollectionShort> {
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for keywords by name
    search_keywords(query: &str): GET "/search/keyword" => Page<Keyword> {
        params { page: u32 }
    }
}

endpoint! {
    /// search for companies by name
    search_companies(query: &str): GET "/search/company" => Page<CompanyShort> {
        params { page: u32 }
    }
}
