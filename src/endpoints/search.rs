use serde::Deserialize;

use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
use crate::models::Keyword;
use crate::{Country, Language, Page};

/// one person in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct PersonShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub profile_path: Option<String>,
    pub known_for_department: Option<String>,
    pub popularity: f64,
    pub gender: Option<u32>,
    pub adult: bool,
}

/// one collection in a search response
#[derive(Debug, Clone, Deserialize)]
pub struct CollectionShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub original_language: Option<String>,
    pub adult: bool,
}

/// one company in a search response
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyShort {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

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
    search_movies(): GET "/search/movie" => Page<MovieShort> {
            required { query: &str }
        params {
            language: Language,
            page: u32,
            include_adult: bool,
            region: Country,
            year: u32,
            primary_release_year: u32,
        }
    }
}

endpoint! {
    /// search for series by name
    search_tv(): GET "/search/tv" => Page<TvShort> {
            required { query: &str }
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
    search_multi(): GET "/search/multi" => Page<MultiResult> {
            required { query: &str }
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for people by name
    search_people(): GET "/search/person" => Page<PersonShort> {
            required { query: &str }
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for collections by name
    search_collections(): GET "/search/collection" => Page<CollectionShort> {
            required { query: &str }
        params { language: Language, page: u32, include_adult: bool }
    }
}

endpoint! {
    /// search for keywords by name
    search_keywords(): GET "/search/keyword" => Page<Keyword> {
            required { query: &str }
        params { page: u32 }
    }
}

endpoint! {
    /// search for companies by name
    search_companies(): GET "/search/company" => Page<CompanyShort> {
            required { query: &str }
        params { page: u32 }
    }
}
