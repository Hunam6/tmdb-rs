use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Certification {
    pub certification: String,
    pub meaning: String,
    pub order: u32,
}

/// certifications by country code
pub type Certifications = HashMap<String, Vec<Certification>>;

endpoint! {
    /// the movie certifications by country
    movie_certifications(): GET "/certification/movie/list" => Certifications [certifications]
}

endpoint! {
    /// the series certifications by country
    tv_certifications(): GET "/certification/tv/list" => Certifications [certifications]
}
