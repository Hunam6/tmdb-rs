use serde::Deserialize;

use crate::endpoints::movie::MovieShort;
use crate::{Language, Page};

#[derive(Debug, Clone, Deserialize)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}

/// `/movie/{id}/keywords`
#[derive(Debug, Clone, Deserialize)]
pub struct MovieKeywords {
    pub keywords: Vec<Keyword>,
}

/// `/tv/{id}/keywords` — same payload, different envelope
#[derive(Debug, Clone, Deserialize)]
pub struct TvKeywords {
    pub results: Vec<Keyword>,
}

endpoint! {
    /// a keyword by id
    keyword(id: u64): GET "/keyword/{id}" => Keyword
}

endpoint! {
    /// the movies tagged with a keyword
    #[deprecated = "TMDB deprecated this endpoint; use discover_movies().with_keywords(...) instead"]
    keyword_movies(id: u64): GET "/keyword/{id}/movies" => Page<MovieShort> {
        params { language: Language, include_adult: bool }
    }
}
