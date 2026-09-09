use serde::Deserialize;

use crate::common::WatchProvider;
use crate::{CountryCode, Language};

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderRegion {
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub english_name: String,
    pub native_name: String,
}

endpoint! {
    /// the regions watch provider data covers
    watch_provider_regions(): GET "/watch/providers/regions" => Vec<ProviderRegion> [results] {
        params { language: Language }
    }
}

endpoint! {
    /// the movie watch providers available in a region
    movie_watch_provider_list(): GET "/watch/providers/movie" => Vec<WatchProvider> [results] {
        params { language: Language, watch_region: CountryCode }
    }
}

endpoint! {
    /// the series watch providers available in a region
    tv_watch_provider_list(): GET "/watch/providers/tv" => Vec<WatchProvider> [results] {
        params { language: Language, watch_region: CountryCode }
    }
}
