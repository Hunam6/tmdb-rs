use std::time::Duration;

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
    RateLimited { retry_after: Option<Duration> },
    /// a requested append_to_response payload was absent from the response
    #[error("requested append `{0}` missing from the response")]
    MissingAppend(&'static str),
    #[error("decode: {0}")]
    Decode(#[from] serde_json::Error),
}
