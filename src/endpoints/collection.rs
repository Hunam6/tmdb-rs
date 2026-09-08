use crate::endpoints::movie::MovieShort;
use crate::models::{Images, Translations};
use crate::Language;

details! {
    /// a collection and its parts
    collection(id: u64): GET "/collection/{}" => CollectionDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            pub original_name: String,
            pub overview: String,
            pub poster_path: Option<String>,
            pub backdrop_path: Option<String>,
            pub original_language: Option<String>,
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
    collection_images(id: u64): GET "/collection/{}/images" => Images {
        params { language: Language, include_image_language: Vec<Language> }
    }
}

endpoint! {
    /// a collection's translations
    collection_translations(id: u64): GET "/collection/{}/translations" => Translations
}
