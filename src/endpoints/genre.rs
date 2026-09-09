use crate::models::Genre;
use crate::Language;

endpoint! {
    /// the movie genre list
    movie_genres(): GET "/genre/movie/list" => Vec<Genre> [genres] {
        params { language: Language }
    }
}

endpoint! {
    /// the series genre list
    tv_genres(): GET "/genre/tv/list" => Vec<Genre> [genres] {
        params { language: Language }
    }
}
