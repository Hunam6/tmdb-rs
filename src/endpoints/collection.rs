use serde::Deserialize;

use crate::common::{Images, Translations};
use crate::endpoints::movie::MovieShort;
use crate::{Backdrop, Language, Poster};

/// one collection in a search response
#[derive(Debug, Clone, Deserialize)]
pub struct CollectionShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub overview: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default, deserialize_with = "crate::common::opt_language")]
    pub original_language: Option<Language>,
    pub adult: bool,
}

endpoint! {
    /// a collection and its parts
    collection(id: u64): GET "/collection/{id}" => CollectionDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub original_name: String,
            pub overview: String,
            #[serde(rename = "poster_path")]
            pub poster: Option<Poster>,
            #[serde(rename = "backdrop_path")]
            pub backdrop: Option<Backdrop>,
            #[serde(default, deserialize_with = "crate::common::opt_language")]
            pub original_language: Option<Language>,
            #[serde(default)]
            pub parts: Vec<MovieShort>,
        }
        appends {
            images: Images,
            translations: Translations,
        }
    }
}

endpoint! {
    /// a collection's images
    collection_images(id: u64): GET "/collection/{id}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// a collection's translations
    collection_translations(id: u64): GET "/collection/{id}/translations" => Translations
}
