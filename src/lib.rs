//! # tmdb-rs
//!
//! An async, fully-typed client for [TMDB](https://www.themoviedb.org)
//! (the movie database), covering API v3 and v4.
//!
//! The signature feature is compile-time `append_to_response`: each `with_*`
//! call fills one slot of the response type, so an appended payload is a plain
//! field rather than an `Option`:
//!
//! ```no_run
//! # async fn example() -> tmdb_rs::Result<()> {
//! let tmdb = tmdb_rs::Client::with_read_token(std::env::var("TMDB_READ_TOKEN").unwrap());
//!
//! let movie = tmdb.movie(550).with_credits().with_similar().send().await?;
//! println!("{}", movie.name);
//! println!("{} cast members", movie.credits.cast.len());
//! println!("{} similar movies", movie.similar.results.len());
//! // movie.images is () — it wasn't requested, and the type says so
//! # Ok(())
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

mod append;
mod client;
mod common;
mod credentials;
mod endpoints;
mod image;
mod param;
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "v4")]
mod v4;

pub use append::Append;
pub use client::Client;
pub use common::*;
#[cfg(feature = "v4")]
pub use credentials::AccessToken;
pub use credentials::{GuestSessionId, SessionId};
pub use endpoints::account::*;
pub use endpoints::authentication::*;
pub use endpoints::certification::*;
pub use endpoints::collection::*;
pub use endpoints::company::*;
pub use endpoints::configuration::*;
pub use endpoints::credit::*;
pub use endpoints::discover::*;
pub use endpoints::find::*;
pub use endpoints::genre::*;
pub use endpoints::keyword::*;
pub use endpoints::list::*;
pub use endpoints::movie::*;
pub use endpoints::network::*;
pub use endpoints::person::*;
pub use endpoints::review::*;
pub use endpoints::search::*;
pub use endpoints::trending::*;
pub use endpoints::tv::*;
pub use endpoints::watch_provider::*;
pub use image::{Backdrop, Logo, Poster, Profile, Still};
pub use isocountry::CountryCode;
pub use isolang::Language;
pub use param::ToParam;
pub use time::Date;
#[cfg(feature = "v4")]
pub use v4::*;

/// every failure the client can produce
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transport: {0}")]
    Transport(#[from] reqwest::Error),
    /// TMDB's own error body, with its status code
    #[error("tmdb error {code}: {message}")]
    Tmdb { code: i32, message: String },
    /// status code 34, or a bare http 404
    #[error("not found")]
    NotFound,
    /// a 429 that was still in effect after one retry
    #[error("rate limited; retry after {retry_after:?}")]
    RateLimited {
        retry_after: Option<std::time::Duration>,
    },
    /// a requested append_to_response payload was absent from the response
    #[error("requested append `{0}` missing from the response")]
    MissingAppend(&'static str),
    #[error("decode: {0}")]
    Decode(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
