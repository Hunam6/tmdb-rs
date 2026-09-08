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
//! println!("{}", movie.title);
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
mod datetime;
mod endpoints;
mod error;
mod image;
mod locale;
mod models;
mod page;

pub use append::Append;
pub use client::Client;
pub use endpoints::genre::*;
pub use endpoints::movie::*;
pub use endpoints::search::*;
pub use endpoints::tv::*;
pub use error::Error;
pub use image::{Backdrop, Logo, Poster, Profile, Still};
pub use locale::{Country, Language};
pub use models::*;
pub use page::Page;

pub type Result<T> = std::result::Result<T, Error>;
