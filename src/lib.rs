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
//! let tmdb = tmdb_rs::Client::new(std::env::var("TMDB_READ_TOKEN").unwrap());
//!
//! let movie = tmdb.movie(550).with_credits().with_similar().send().await?;
//! println!("{}", movie.name);
//! println!("{} cast members", movie.credits.cast.len());
//! println!("{} similar movies", movie.similar.results.len());
//! // movie.images is () — it wasn't requested, and the type says so
//! # Ok(())
//! # }
//! ```

#[macro_use]
mod macros;

mod append;
mod client;
mod common;
mod credentials;
mod datetime;
mod endpoints;
mod error;
mod image;
mod page;
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
pub use error::Error;
pub use image::{Backdrop, Logo, Poster, Profile, Still};
pub use isocountry::CountryCode;
pub use isolang::Language;
pub use page::Page;
pub use param::ToParam;
pub use time::Date;
#[cfg(feature = "v4")]
pub use v4::*;

pub type Result<T> = std::result::Result<T, Error>;
