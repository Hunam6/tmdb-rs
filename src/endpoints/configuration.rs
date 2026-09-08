use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ImageConfig {
    pub base_url: String,
    pub secure_base_url: String,
    pub backdrop_sizes: Vec<String>,
    pub logo_sizes: Vec<String>,
    pub poster_sizes: Vec<String>,
    pub profile_sizes: Vec<String>,
    pub still_sizes: Vec<String>,
}

/// the API-wide configuration: image base urls and sizes, change keys
#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub images: ImageConfig,
    pub change_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CountryInfo {
    pub iso_3166_1: String,
    pub english_name: String,
    pub native_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DepartmentJobs {
    pub department: String,
    pub jobs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageInfo {
    pub iso_639_1: String,
    pub english_name: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timezone {
    pub iso_3166_1: String,
    pub zones: Vec<String>,
}

endpoint! {
    /// the API-wide configuration
    configuration(): GET "/configuration" => Configuration
}

endpoint! {
    /// the countries TMDB knows
    countries(): GET "/configuration/countries" => Vec<CountryInfo>
}

endpoint! {
    /// the departments and jobs credits use
    jobs(): GET "/configuration/jobs" => Vec<DepartmentJobs>
}

endpoint! {
    /// the languages TMDB knows
    languages(): GET "/configuration/languages" => Vec<LanguageInfo>
}

endpoint! {
    /// the languages translations can be requested in
    primary_translations(): GET "/configuration/primary_translations" => Vec<String>
}

endpoint! {
    /// the timezones TMDB knows, by country
    timezones(): GET "/configuration/timezones" => Vec<Timezone>
}
