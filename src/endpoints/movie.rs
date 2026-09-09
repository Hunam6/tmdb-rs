use serde::Deserialize;
use time::Date;

use crate::common::{
    AccountStates, AlternativeTitles, Changes, ExternalIds, Images, StatusResponse, Translations,
    Videos, WatchProviders,
};
use crate::datetime::opt_date;
use crate::endpoints::credit::Credits;
use crate::endpoints::genre::Genre;
use crate::endpoints::keyword::MovieKeywords;
use crate::endpoints::list::ListShort;
use crate::endpoints::review::Review;
use crate::{Backdrop, CountryCode, GuestSessionId, Language, Page, Poster, SessionId};

/// TMDB's release date kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, num_enum::FromPrimitive)]
#[serde(from = "u32")]
#[repr(u32)]
pub enum ReleaseType {
    Premiere = 1,
    TheatricalLimited = 2,
    Theatrical = 3,
    Digital = 4,
    Physical = 5,
    Tv = 6,
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDate {
    pub certification: String,
    #[serde(rename = "type")]
    pub kind: ReleaseType,
    /// ISO 8601 datetime
    pub release_date: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CountryReleaseDates {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDates {
    pub results: Vec<CountryReleaseDates>,
}

/// one movie in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct MovieShort {
    pub id: u64,
    #[serde(rename = "title")]
    pub name: String,
    #[serde(rename = "original_title")]
    pub original_name: String,
    pub overview: String,
    #[serde(default, deserialize_with = "opt_date")]
    pub release_date: Option<Date>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub original_language: Option<String>,
    pub adult: bool,
    pub video: bool,
}

endpoint! {
    /// the primary details of a movie
    movie(id: u64): GET "/movie/{id}" => MovieDetails {
        params { language: Language }
        base {
            pub id: u64,
            #[serde(rename = "title")]
            pub name: String,
            #[serde(rename = "original_title")]
            pub original_name: String,
            pub overview: String,
            pub tagline: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub release_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
            pub popularity: f64,
            pub runtime: Option<u32>,
            pub original_language: Option<String>,
            #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
            #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
            pub genres: Vec<Genre>,
            pub imdb_id: Option<String>,
            #[serde(rename = "homepage")]
            pub website: String,
            pub status: String,
            pub budget: u64,
            pub revenue: u64,
            pub adult: bool,
        }
        appends {
            account_states: AccountStates,
            alternative_titles: AlternativeTitles,
            changes: Changes,
            credits: Credits,
            external_ids: ExternalIds,
            images: Images,
            keywords: MovieKeywords,
            lists: Page<ListShort>,
            recommendations: Page<MovieShort>,
            release_dates: ReleaseDates,
            reviews: Page<Review>,
            similar: Page<MovieShort>,
            translations: Translations,
            videos: Videos,
            watch_providers: WatchProviders as "watch/providers",
        }
    }
}

endpoint! {
    /// a movie's account states (favorite, rated, watchlist)
    movie_account_states(id: u64): GET "/movie/{id}/account_states" => AccountStates {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

endpoint! {
    /// a movie's alternative titles
    movie_alternative_titles(id: u64): GET "/movie/{id}/alternative_titles" => AlternativeTitles {
        params { language: Language, country: CountryCode }
    }
}

endpoint! {
    /// a movie's recent changes
    movie_changes_by_id(id: u64): GET "/movie/{id}/changes" => Changes {
        params { start_date: Date, end_date: Date, page: u32 }
    }
}

endpoint! {
    /// a movie's credits
    movie_credits(id: u64): GET "/movie/{id}/credits" => Credits {
        params { language: Language }
    }
}

endpoint! {
    /// a movie's images
    movie_images(id: u64): GET "/movie/{id}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// the lists a movie appears in
    movie_lists(id: u64): GET "/movie/{id}/lists" => Page<ListShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// movies similar to a movie, from TMDB's collaborative filtering
    movie_similar(id: u64): GET "/movie/{id}/similar" => Page<MovieShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a movie's recommendations
    movie_recommendations(id: u64): GET "/movie/{id}/recommendations" => Page<MovieShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a movie's release dates by country
    movie_release_dates(id: u64): GET "/movie/{id}/release_dates" => ReleaseDates
}

endpoint! {
    /// a movie's reviews
    movie_reviews(id: u64): GET "/movie/{id}/reviews" => Page<Review> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a movie's translations
    movie_translations(id: u64): GET "/movie/{id}/translations" => Translations
}

endpoint! {
    /// a movie's videos (trailers, teasers, ...)
    movie_videos(id: u64): GET "/movie/{id}/videos" => Videos {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// where a movie can be streamed, rented or bought, by country
    movie_watch_providers(id: u64): GET "/movie/{id}/watch/providers" => WatchProviders
}

endpoint! {
    /// rate a movie; pass a session or guest session
    rate_movie(id: u64): POST "/movie/{id}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
        body { value: f64 }
    }
}

endpoint! {
    /// delete a movie rating
    unrate_movie(id: u64): DELETE "/movie/{id}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

endpoint! {
    /// the current movie ids, paged by insertion order
    movie_changes(): GET "/movie/changes" => Page<MovieChange> {
        params { page: u32, start_date: Date, end_date: Date }
    }
}

/// a movie touched by a change window
#[derive(Debug, Clone, Deserialize)]
pub struct MovieChange {
    pub id: u64,
    pub adult: Option<bool>,
}

endpoint! {
    /// the newest movie
    movie_latest(): GET "/movie/latest" => MovieDetails {
        params { language: Language }
    }
}

endpoint! {
    /// movies currently in theatres
    movie_now_playing(): GET "/movie/now_playing" => Page<MovieShort> {
        params { language: Language, page: u32, region: CountryCode }
    }
}

endpoint! {
    /// movies ordered by popularity
    movie_popular(): GET "/movie/popular" => Page<MovieShort> {
        params { language: Language, page: u32, region: CountryCode }
    }
}

endpoint! {
    /// movies ordered by rating
    movie_top_rated(): GET "/movie/top_rated" => Page<MovieShort> {
        params { language: Language, page: u32, region: CountryCode }
    }
}

endpoint! {
    /// upcoming movies
    movie_upcoming(): GET "/movie/upcoming" => Page<MovieShort> {
        params { language: Language, page: u32, region: CountryCode }
    }
}

endpoint! {
    /// a movie's keywords, without the details round-trip
    movie_keywords(id: u64): GET "/movie/{id}/keywords" => MovieKeywords
}

endpoint! {
    /// a movie's ids on other databases
    movie_external_ids(id: u64): GET "/movie/{id}/external_ids" => ExternalIds
}
