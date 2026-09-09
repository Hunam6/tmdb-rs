use crate::endpoints::movie::MovieShort;
use crate::models::Keyword;
use crate::{Language, Page};

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
