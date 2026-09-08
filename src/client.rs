use std::time::Duration;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::{Error, Result};

const BASE_V3: &str = "https://api.themoviedb.org/3";

#[derive(Clone)]
enum Auth {
    /// v3 api key, sent as a query param
    Key(String),
    /// v4 read access token, sent as a bearer token
    Token(String),
}

/// the entry point; clone freely, it shares one connection pool
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    auth: Auth,
    base: String,
}

impl Client {
    /// authenticate with a v4 read access token
    pub fn new(read_access_token: impl Into<String>) -> Self {
        Self::build(Auth::Token(read_access_token.into()))
    }

    /// authenticate with a v3 api key
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self::build(Auth::Key(api_key.into()))
    }

    fn build(auth: Auth) -> Self {
        Self {
            http: reqwest::Client::new(),
            auth,
            base: BASE_V3.into(),
        }
    }

    /// bring your own reqwest client (proxies, timeouts, ...)
    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }

    /// point at another host (a proxy, or a mock in tests)
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base = url.into();
        self
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        let mut retried = false;
        loop {
            let mut request = self.http.get(format!("{}{path}", self.base));
            match &self.auth {
                Auth::Key(key) => request = request.query(&[("api_key", key.as_str())]),
                Auth::Token(token) => request = request.bearer_auth(token),
            }
            let response = request.query(query).send().await?;
            let status = response.status();

            if status == StatusCode::TOO_MANY_REQUESTS && !retried {
                retried = true;
                let wait = response
                    .headers()
                    .get("retry-after")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(1);
                tokio::time::sleep(Duration::from_secs(wait)).await;
                continue;
            }

            let body = response.text().await?;
            if status.is_success() {
                return Ok(serde_json::from_str(&body)?);
            }

            #[derive(Deserialize)]
            struct TmdbError {
                status_code: i32,
                status_message: String,
            }
            return Err(match serde_json::from_str::<TmdbError>(&body) {
                Ok(error) if error.status_code == 34 => Error::NotFound,
                Ok(error) => Error::Tmdb {
                    code: error.status_code,
                    message: error.status_message,
                },
                Err(_) if status == StatusCode::NOT_FOUND => Error::NotFound,
                Err(_) if status == StatusCode::TOO_MANY_REQUESTS => Error::RateLimited {
                    retry_after: None,
                },
                Err(_) => Error::Tmdb {
                    code: status.as_u16().into(),
                    message: body,
                },
            });
        }
    }
}
