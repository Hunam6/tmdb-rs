use serde::Deserialize;
use strum::Display;
use time::Date;

use crate::common::{MediaType, StatusResponse};
use crate::datetime::opt_date;
use crate::endpoints::list::ListShort;
use crate::endpoints::movie::MovieShort;
use crate::endpoints::tv::TvShort;
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
    account(session_id: SessionId): GET "/account" => AccountDetails
}

endpoint! {
    /// the account's favorite movies
    favorite_movies(account_id: u64, session_id: SessionId): GET "/account/{account_id}/favorite/movies" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's favorite series
    favorite_tv(account_id: u64, session_id: SessionId): GET "/account/{account_id}/favorite/tv" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// mark or unmark a favorite
    mark_favorite(account_id: u64, session_id: SessionId): POST "/account/{account_id}/favorite" => StatusResponse {
        body { media_type: MediaType, media_id: u64, favorite: bool }
    }
}

endpoint! {
    /// the account's rated movies
    rated_movies(account_id: u64, session_id: SessionId): GET "/account/{account_id}/rated/movies" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's rated series
    rated_tv(account_id: u64, session_id: SessionId): GET "/account/{account_id}/rated/tv" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's rated episodes
    rated_episodes(account_id: u64, session_id: SessionId): GET "/account/{account_id}/rated/episodes" => Page<RatedEpisode> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's movie watchlist
    watchlist_movies(account_id: u64, session_id: SessionId): GET "/account/{account_id}/watchlist/movies" => Page<MovieShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// the account's series watchlist
    watchlist_tv(account_id: u64, session_id: SessionId): GET "/account/{account_id}/watchlist/tv" => Page<TvShort> {
        params { language: Language, page: u32, sort_by: AccountSort }
    }
}

endpoint! {
    /// add or remove a watchlist entry
    set_watchlist(account_id: u64, session_id: SessionId): POST "/account/{account_id}/watchlist" => StatusResponse {
        body { media_type: MediaType, media_id: u64, watchlist: bool }
    }
}

endpoint! {
    /// the account's lists
    account_lists(account_id: u64, session_id: SessionId): GET "/account/{account_id}/lists" => Page<ListShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// the account's watch providers preferences
    account_watch_providers(session_id: SessionId): GET "/account/watch/providers" => AccountWatchProviders {
        params { watch_region: CountryCode }
    }
}
