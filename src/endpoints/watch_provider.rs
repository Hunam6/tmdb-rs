use serde::Deserialize;

use crate::models::WatchProvider;
use crate::{CountryCode, Language};

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderRegion {
    pub iso_3166_1: String,
    pub english_name: String,
    pub native_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderRegions {
    pub results: Vec<ProviderRegion>,
}

/// the providers available in a region
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderList {
    pub results: Vec<WatchProvider>,
}

endpoint! {
    /// the regions watch provider data covers
    watch_provider_regions(): GET "/watch/providers/regions" => ProviderRegions {
        params { language: Language }
    }
}

endpoint! {
    /// the movie watch providers available in a region
    movie_watch_provider_list(): GET "/watch/providers/movie" => ProviderList {
        params { language: Language, watch_region: CountryCode }
    }
}

endpoint! {
    /// the series watch providers available in a region
    tv_watch_provider_list(): GET "/watch/providers/tv" => ProviderList {
        params { language: Language, watch_region: CountryCode }
    }
}
