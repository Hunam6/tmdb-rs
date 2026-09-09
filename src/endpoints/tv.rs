use serde::Deserialize;
use time::Date;

use crate::append::appendable;
use crate::datetime::opt_date;
use crate::endpoints::genre::Genre;
use crate::endpoints::keyword::TvKeywords;
use crate::endpoints::list::ListShort;
use crate::models::{
    AccountStates, AlternativeTitleResults, Changes, ContentRatings, Credits, ExternalIds, Images,
    Rated, Review, SeasonShort, StatusResponse, Translations, Videos, WatchProviders,
};
use crate::{Backdrop, GuestSessionId, Language, Logo, Page, Poster, Profile, SessionId, Still};

/// one series in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct TvShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub overview: String,
    #[serde(default, deserialize_with = "opt_date")]
    pub first_air_date: Option<Date>,
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
    #[serde(default)]
    pub origin_country: Vec<String>,
    pub adult: bool,
}

/// a series' next aired episode
#[derive(Debug, Clone, Deserialize)]
pub struct NextEpisode {
    pub id: u64,
    pub name: String,
    pub episode_number: u32,
    pub season_number: u32,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
}

/// one episode inside a season
#[derive(Debug, Clone, Deserialize)]
pub struct Episode {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub episode_number: u32,
    pub season_number: u32,
    pub runtime: Option<u32>,
    #[serde(rename = "still_path")]
    pub still: Option<Still>,
    #[serde(default, deserialize_with = "opt_date")]
    pub air_date: Option<Date>,
    pub vote_average: f64,
    pub vote_count: u32,
}

/// one role in an aggregate credits entry
#[derive(Debug, Clone, Deserialize)]
pub struct Role {
    pub credit_id: String,
    pub character: String,
    pub episode_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggregateCastMember {
    pub id: u64,
    pub name: String,
    #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
    pub roles: Vec<Role>,
    pub total_episode_count: u32,
    pub order: Option<u32>,
}

/// one job in an aggregate credits entry
#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    pub credit_id: String,
    pub job: String,
    pub episode_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggregateCrewMember {
    pub id: u64,
    pub name: String,
    #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
    pub jobs: Vec<Job>,
    pub total_episode_count: u32,
    pub department: String,
}

/// credits rolled up across all episodes of a series
#[derive(Debug, Clone, Deserialize)]
pub struct AggregateCredits {
    pub cast: Vec<AggregateCastMember>,
    pub crew: Vec<AggregateCrewMember>,
}

/// one network on an episode group
#[derive(Debug, Clone, Deserialize)]
pub struct NetworkShort {
    pub id: u64,
    pub name: String,
    #[serde(rename = "logo_path")]
    pub logo: Option<Logo>,
    pub origin_country: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeGroupShort {
    pub id: String,
    pub name: String,
    pub description: String,
    pub episode_count: u32,
    pub group_count: u32,
    #[serde(rename = "type")]
    pub kind: u32,
    pub network: Option<NetworkShort>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeGroups {
    pub results: Vec<EpisodeGroupShort>,
}

/// one group inside an episode group
#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeGroupEntry {
    pub id: String,
    pub name: String,
    pub order: u32,
    #[serde(default)]
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub episode_count: u32,
    pub group_count: u32,
    #[serde(rename = "type")]
    pub kind: u32,
    pub network: Option<NetworkShort>,
    pub groups: Vec<EpisodeGroupEntry>,
}

/// one episode screened in theatres
#[derive(Debug, Clone, Deserialize)]
pub struct ScreenedEntry {
    pub id: u64,
    pub episode_number: u32,
    pub season_number: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScreenedTheatrically {
    pub results: Vec<ScreenedEntry>,
}

appendable! {
    AggregateCredits,
    EpisodeGroups,
    ScreenedTheatrically,
}

endpoint! {
    /// the primary details of a series
    tv(id: u64): GET "/tv/{id}" => TvDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub original_name: String,
            pub overview: String,
            pub tagline: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub first_air_date: Option<Date>,
            #[serde(default, deserialize_with = "opt_date")]
            pub last_air_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
            pub popularity: f64,
            pub number_of_seasons: u32,
            pub number_of_episodes: u32,
            pub episode_run_time: Vec<u32>,
            pub original_language: Option<String>,
            #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
            #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
            pub genres: Vec<Genre>,
            pub homepage: String,
            pub status: String,
            pub in_production: bool,
            #[serde(default)]
            pub languages: Vec<String>,
            #[serde(default)]
            pub origin_country: Vec<String>,
            #[serde(default)]
            pub seasons: Vec<SeasonShort>,
            pub next_episode_to_air: Option<NextEpisode>,
            pub adult: bool,
        }
        appends {
            account_states: AccountStates,
            aggregate_credits: AggregateCredits,
            alternative_titles: AlternativeTitleResults,
            changes: Changes,
            content_ratings: ContentRatings,
            credits: Credits,
            episode_groups: EpisodeGroups,
            external_ids: ExternalIds,
            images: Images,
            keywords: TvKeywords,
            lists: Page<ListShort>,
            recommendations: Page<TvShort>,
            reviews: Page<Review>,
            similar: Page<TvShort>,
            translations: Translations,
            videos: Videos,
            watch_providers: WatchProviders as "watch/providers",
        }
    }
}

endpoint! {
    /// one season of a series, episodes included
    tv_season(id: u64, season: u32): GET "/tv/{id}/season/{season}" => SeasonDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub season_number: u32,
            pub name: String,
            pub overview: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub air_date: Option<Date>,
            #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
            #[serde(default)]
            pub episodes: Vec<Episode>,
        }
        appends {
            account_states: AccountStates,
            aggregate_credits: AggregateCredits,
            changes: Changes,
            credits: Credits,
            external_ids: ExternalIds,
            images: Images,
            translations: Translations,
            videos: Videos,
            watch_providers: WatchProviders as "watch/providers",
        }
    }
}

endpoint! {
    /// one episode of a series
    tv_episode(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}" => EpisodeDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub overview: String,
            pub episode_number: u32,
            pub season_number: u32,
            pub runtime: Option<u32>,
            #[serde(rename = "still_path")]
    pub still: Option<Still>,
            #[serde(default, deserialize_with = "opt_date")]
            pub air_date: Option<Date>,
            pub vote_average: f64,
            pub vote_count: u32,
        }
        appends {
            account_states: AccountStates,
            changes: Changes,
            credits: Credits,
            external_ids: ExternalIds,
            images: Images,
            translations: Translations,
            videos: Videos,
        }
    }
}

endpoint! {
    /// the newest series
    tv_latest(): GET "/tv/latest" => TvDetails {
        params { language: Language }
    }
}

endpoint! {
    /// series ordered by popularity
    tv_popular(): GET "/tv/popular" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series ordered by rating
    tv_top_rated(): GET "/tv/top_rated" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series with an episode airing in the next week
    tv_on_the_air(): GET "/tv/on_the_air" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series with an episode airing today
    tv_airing_today(): GET "/tv/airing_today" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a series' keywords, without the details round-trip
    tv_keywords(id: u64): GET "/tv/{id}/keywords" => TvKeywords
}

endpoint! {
    /// a series' ids on other databases
    tv_external_ids(id: u64): GET "/tv/{id}/external_ids" => ExternalIds
}

endpoint! {
    /// a series' account states (favorite, rated, watchlist)
    tv_account_states(id: u64): GET "/tv/{id}/account_states" => AccountStates {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

endpoint! {
    /// a series' aggregate credits
    tv_aggregate_credits(id: u64): GET "/tv/{id}/aggregate_credits" => AggregateCredits {
        params { language: Language }
    }
}

endpoint! {
    /// a series' alternative titles
    tv_alternative_titles(id: u64): GET "/tv/{id}/alternative_titles" => AlternativeTitleResults
}

endpoint! {
    /// a series' recent changes
    tv_changes_by_id(id: u64): GET "/tv/{id}/changes" => Changes {
        params { start_date: Date, end_date: Date, page: u32 }
    }
}

endpoint! {
    /// a series' content ratings by country
    tv_content_ratings(id: u64): GET "/tv/{id}/content_ratings" => ContentRatings
}

endpoint! {
    /// a series' credits
    tv_credits(id: u64): GET "/tv/{id}/credits" => Credits {
        params { language: Language }
    }
}

endpoint! {
    /// a series' episode groups
    tv_episode_groups(id: u64): GET "/tv/{id}/episode_groups" => EpisodeGroups
}

endpoint! {
    /// a series' images
    tv_images(id: u64): GET "/tv/{id}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// the lists a series appears in
    tv_lists(id: u64): GET "/tv/{id}/lists" => Page<ListShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// series similar to a series
    tv_similar(id: u64): GET "/tv/{id}/similar" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a series' recommendations
    tv_recommendations(id: u64): GET "/tv/{id}/recommendations" => Page<TvShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a series' reviews
    tv_reviews(id: u64): GET "/tv/{id}/reviews" => Page<Review> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// the episodes of a series that were screened in theatres
    tv_screened_theatrically(id: u64): GET "/tv/{id}/screened_theatrically" => ScreenedTheatrically
}

endpoint! {
    /// a series' translations
    tv_translations(id: u64): GET "/tv/{id}/translations" => Translations
}

endpoint! {
    /// a series' videos (trailers, teasers, ...)
    tv_videos(id: u64): GET "/tv/{id}/videos" => Videos {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// where a series can be streamed, rented or bought, by country
    tv_watch_providers(id: u64): GET "/tv/{id}/watch/providers" => WatchProviders
}

endpoint! {
    /// rate a series; pass a session or guest session
    rate_tv(id: u64): POST "/tv/{id}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
        body { value: f64 }
    }
}

endpoint! {
    /// delete a series rating
    unrate_tv(id: u64): DELETE "/tv/{id}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

endpoint! {
    /// a season's account states (per-episode ratings)
    tv_season_account_states(id: u64, season: u32): GET "/tv/{id}/season/{season}/account_states" => SeasonAccountStates {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

/// per-episode ratings within a season
#[derive(Debug, Clone, Deserialize)]
pub struct SeasonAccountStates {
    pub results: Vec<EpisodeAccountState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeAccountState {
    pub id: u64,
    pub episode_number: u32,
    pub rated: Rated,
}

endpoint! {
    /// a season's aggregate credits
    tv_season_aggregate_credits(id: u64, season: u32): GET "/tv/{id}/season/{season}/aggregate_credits" => AggregateCredits {
        params { language: Language }
    }
}

endpoint! {
    /// a season's changes
    tv_season_changes(id: u64): GET "/tv/season/{id}/changes" => Changes {
        params { start_date: Date, end_date: Date, page: u32 }
    }
}

endpoint! {
    /// a season's credits
    tv_season_credits(id: u64, season: u32): GET "/tv/{id}/season/{season}/credits" => Credits {
        params { language: Language }
    }
}

endpoint! {
    /// a season's ids on other databases
    tv_season_external_ids(id: u64, season: u32): GET "/tv/{id}/season/{season}/external_ids" => ExternalIds
}

endpoint! {
    /// a season's images
    tv_season_images(id: u64, season: u32): GET "/tv/{id}/season/{season}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// a season's translations
    tv_season_translations(id: u64, season: u32): GET "/tv/{id}/season/{season}/translations" => Translations
}

endpoint! {
    /// a season's videos
    tv_season_videos(id: u64, season: u32): GET "/tv/{id}/season/{season}/videos" => Videos {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// where a season can be watched, by country
    tv_season_watch_providers(id: u64, season: u32): GET "/tv/{id}/season/{season}/watch/providers" => WatchProviders
}

endpoint! {
    /// an episode's account states
    tv_episode_account_states(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/account_states" => EpisodeAccountStates {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

/// an episode's own account state
#[derive(Debug, Clone, Deserialize)]
pub struct EpisodeAccountStates {
    pub id: u64,
    pub rated: Rated,
}

endpoint! {
    /// an episode's changes
    tv_episode_changes(id: u64): GET "/tv/episode/{id}/changes" => Changes {
        params { start_date: Date, end_date: Date, page: u32 }
    }
}

endpoint! {
    /// an episode's credits
    tv_episode_credits(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/credits" => Credits {
        params { language: Language }
    }
}

endpoint! {
    /// an episode's ids on other databases
    tv_episode_external_ids(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/external_ids" => ExternalIds
}

endpoint! {
    /// an episode's images
    tv_episode_images(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// an episode's translations
    tv_episode_translations(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/translations" => Translations
}

endpoint! {
    /// an episode's videos
    tv_episode_videos(id: u64, season: u32, episode: u32): GET "/tv/{id}/season/{season}/episode/{episode}/videos" => Videos {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// rate an episode; pass a session or guest session
    rate_tv_episode(id: u64, season: u32, episode: u32): POST "/tv/{id}/season/{season}/episode/{episode}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
        body { value: f64 }
    }
}

endpoint! {
    /// delete an episode rating
    unrate_tv_episode(id: u64, season: u32, episode: u32): DELETE "/tv/{id}/season/{season}/episode/{episode}/rating" => StatusResponse {
        params { session_id: SessionId, guest_session_id: GuestSessionId }
    }
}

endpoint! {
    /// an episode group
    tv_episode_group(id: &str): GET "/tv/episode_group/{id}" => EpisodeGroup {
        params { language: Language }
    }
}
