use crate::endpoints::movie::MovieShort;
use crate::models::Keyword;
use crate::{Language, Page};

endpoint! {
    /// a keyword by id
    keyword(id: u64): GET "/keyword/{}" => Keyword
}

endpoint! {
    /// the movies tagged with a keyword
    keyword_movies(id: u64): GET "/keyword/{}/movies" => Page<MovieShort> {
        params { language: Language, include_adult: bool }
    }
}
