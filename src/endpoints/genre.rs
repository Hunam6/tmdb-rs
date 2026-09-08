use serde::Deserialize;

use crate::models::Genre;
use crate::Language;

/// the answer of both genre list endpoints
#[derive(Debug, Clone, Deserialize)]
pub struct GenreList {
    pub genres: Vec<Genre>,
}

endpoint! {
    /// the movie genre list
    movie_genres(): GET "/genre/movie/list" => GenreList {
        params { language: Language }
    }
}

endpoint! {
    /// the series genre list
    tv_genres(): GET "/genre/tv/list" => GenreList {
        params { language: Language }
    }
}
