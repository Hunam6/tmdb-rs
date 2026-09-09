use serde::Deserialize;
use strum::Display;
use time::Date;

use crate::datetime::opt_date;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
use crate::models::{ListShort, MediaType, StatusResponse};
use crate::{CountryCode, Language, Page, Profile, SessionId, Still};

/// the sort order of account lists
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum AccountSort {
    #[strum(serialize = "created_at.asc")]
    CreatedAtAsc,
    #[strum(serialize = "created_at.desc")]
    CreatedAtDesc,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "RawAvatar")]
pub struct Avatar {
    pub gravatar_hash: String,
    pub path: Option<Profile>,
}

#[derive(Deserialize)]
struct RawAvatar {
    gravatar: RawGravatar,
    tmdb: RawTmdbAvatar,
}

#[derive(Deserialize)]
struct RawGravatar {
    hash: String,
}

#[derive(Deserialize)]
struct RawTmdbAvatar {
    avatar_path: Option<Profile>,
}

impl From<RawAvatar> for Avatar {
    fn from(raw: RawAvatar) -> Self {
        Avatar {
            gravatar_hash: raw.gravatar.hash,
            path: raw.tmdb.avatar_path,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountDetails {
    pub id: u64,
    pub name: String,
    pub username: String,
    #[serde(rename = "iso_639_1")]
    pub language: Language,
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub include_adult: bool,
    pub avatar: Avatar,
}

/// one episode in the rated-episodes list
#[derive(Debug, Clone, Deserialize)]
pub struct RatedEpisode {
    pub id: u64,
    pub name: String,
    pub episode_number: u32,
    pub season_number: u32,
    pub show_id: Option<u64>,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
    #[serde(rename = "still_path")]
    pub still: Option<Still>,
    pub vote_average: f64,
    pub rating: f64,
}

/// the account's watch provider preferences
#[derive(Debug, Clone, Deserialize)]
pub struct AccountWatchProviders {
    pub id: u64,
    pub watch_region: Option<String>,
    pub watch_provider_ids: Vec<u64>,
}

endpoint! {
    /// the account the session belongs to
    account(): GET "/account" => AccountDetails {
        required { session_id: SessionId }
    }
}

endpoint! {
    /// the account's favorite movies
    favorite_movies(account_id: u64): GET "/account/{}/favorite/movies" => Page<MovieShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's favorite series
    favorite_tv(account_id: u64): GET "/account/{}/favorite/tv" => Page<TvShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// mark or unmark a favorite
    mark_favorite(account_id: u64): POST "/account/{}/favorite" => StatusResponse {
        required { session_id: SessionId }
        body { media_type: MediaType, media_id: u64, favorite: bool }
    }
}

endpoint! {
    /// the account's rated movies
    rated_movies(account_id: u64): GET "/account/{}/rated/movies" => Page<MovieShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's rated series
    rated_tv(account_id: u64): GET "/account/{}/rated/tv" => Page<TvShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's rated episodes
    rated_episodes(account_id: u64): GET "/account/{}/rated/episodes" => Page<RatedEpisode> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's movie watchlist
    watchlist_movies(account_id: u64): GET "/account/{}/watchlist/movies" => Page<MovieShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's series watchlist
    watchlist_tv(account_id: u64): GET "/account/{}/watchlist/tv" => Page<TvShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// add or remove a watchlist entry
    set_watchlist(account_id: u64): POST "/account/{}/watchlist" => StatusResponse {
        required { session_id: SessionId }
        body { media_type: MediaType, media_id: u64, watchlist: bool }
    }
}

endpoint! {
    /// the account's lists
    account_lists(account_id: u64): GET "/account/{}/lists" => Page<ListShort> {
        required { session_id: SessionId }
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// the account's watch providers preferences
    account_watch_providers(): GET "/account/watch/providers" => AccountWatchProviders {
        required { session_id: SessionId }
        params { watch_region: CountryCode }
    }
}
