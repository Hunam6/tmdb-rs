use serde::Deserialize;
use time::Date;

use crate::append::appendable;
use crate::datetime::opt_date;
use crate::models::{Changes, ExternalIds, Image};
use crate::{Backdrop, CountryCode, Language, Page, Poster, Profile};

/// one person in a list or search response
#[derive(Debug, Clone, Deserialize)]
pub struct PersonShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
    pub known_for_department: Option<String>,
    pub popularity: f64,
    pub gender: Option<u32>,
    pub adult: bool,
}

/// a movie acting credit in a person's filmography
#[derive(Debug, Clone, Deserialize)]
pub struct MovieCastCredit {
    pub id: u64,
    #[serde(rename = "title")]
    pub name: String,
    #[serde(rename = "original_title")]
    pub original_name: String,
    pub character: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default, deserialize_with = "opt_date")]
    pub release_date: Option<Date>,
    pub credit_id: String,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub overview: String,
    pub adult: bool,
}

/// a series acting credit in a person's filmography
#[derive(Debug, Clone, Deserialize)]
pub struct TvCastCredit {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub character: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default, deserialize_with = "opt_date")]
    pub first_air_date: Option<Date>,
    pub credit_id: String,
    pub episode_count: Option<u32>,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub overview: String,
    #[serde(default)]
    pub origin_country: Vec<String>,
}

/// one acting credit, discriminated by TMDB's `media_type`
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "media_type", rename_all = "snake_case")]
pub enum CastCredit {
    Movie(MovieCastCredit),
    Tv(TvCastCredit),
}

/// a movie crew credit in a person's filmography
#[derive(Debug, Clone, Deserialize)]
pub struct MovieCrewCredit {
    pub id: u64,
    #[serde(rename = "title")]
    pub name: String,
    #[serde(rename = "original_title")]
    pub original_name: String,
    pub job: String,
    pub department: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default, deserialize_with = "opt_date")]
    pub release_date: Option<Date>,
    pub credit_id: String,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub overview: String,
    pub adult: bool,
}

/// a series crew credit in a person's filmography
#[derive(Debug, Clone, Deserialize)]
pub struct TvCrewCredit {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub job: String,
    pub department: String,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    #[serde(default, deserialize_with = "opt_date")]
    pub first_air_date: Option<Date>,
    pub credit_id: String,
    pub episode_count: Option<u32>,
    pub vote_average: f64,
    pub vote_count: u32,
    pub popularity: f64,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub overview: String,
    #[serde(default)]
    pub origin_country: Vec<String>,
}

/// one crew credit, discriminated by TMDB's `media_type`
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "media_type", rename_all = "snake_case")]
pub enum CrewCredit {
    Movie(MovieCrewCredit),
    Tv(TvCrewCredit),
}

/// a person's filmography; combined, movie-only and tv-only share the shape
#[derive(Debug, Clone, Deserialize)]
pub struct PersonCredits {
    pub cast: Vec<CastCredit>,
    pub crew: Vec<CrewCredit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonImages {
    pub profiles: Vec<Image<Profile>>,
}

/// the media a tagged image belongs to; movies fill `title`, series `name`
#[derive(Debug, Clone, Deserialize)]
pub struct TaggedMedia {
    pub id: u64,
    pub title: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "poster_path")]
    pub poster: Option<Poster>,
    #[serde(rename = "backdrop_path")]
    pub backdrop: Option<Backdrop>,
    pub vote_average: Option<f64>,
}

impl TaggedMedia {
    /// the title, whichever of the movie/series keys it came in
    pub fn display_title(&self) -> Option<&str> {
        self.title.as_deref().or(self.name.as_deref())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaggedImage {
    pub id: String,
    #[serde(rename = "file_path")]
    pub file: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub vote_average: f64,
    pub vote_count: u32,
    pub media_type: String,
    pub media: Option<TaggedMedia>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonTranslationData {
    pub biography: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonTranslation {
    #[serde(rename = "iso_639_1")]
    pub language: Language,
    #[serde(rename = "iso_3166_1")]
    pub country: CountryCode,
    pub name: String,
    pub english_name: String,
    pub data: PersonTranslationData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonTranslations {
    pub translations: Vec<PersonTranslation>,
}

appendable! {
    PersonCredits,
    PersonImages,
    PersonTranslations,
}

endpoint! {
    /// the primary details of a person
    person(id: u64): GET "/person/{id}" => PersonDetails {
        params { language: Language }
        base {
            pub id: u64,
            pub name: String,
            #[serde(default)]
            pub also_known_as: Vec<String>,
            pub biography: String,
            #[serde(default, deserialize_with = "opt_date")]
            pub birthday: Option<Date>,
            #[serde(default, deserialize_with = "opt_date")]
            pub deathday: Option<Date>,
            pub gender: Option<u32>,
            #[serde(rename = "homepage")]
            pub website: Option<String>,
            pub imdb_id: Option<String>,
            pub known_for_department: Option<String>,
            pub place_of_birth: Option<String>,
            pub popularity: f64,
            #[serde(rename = "profile_path")]
    pub profile: Option<Profile>,
            pub adult: bool,
        }
        appends {
            changes: Changes,
            combined_credits: PersonCredits,
            external_ids: ExternalIds,
            images: PersonImages,
            movie_credits: PersonCredits,
            #[deprecated = "TMDB deprecated tagged_images; it no longer returns data"]
            tagged_images: Page<TaggedImage>,
            translations: PersonTranslations,
            tv_credits: PersonCredits,
        }
    }
}

endpoint! {
    /// the newest person
    person_latest(): GET "/person/latest" => PersonDetails {
        params { language: Language }
    }
}

endpoint! {
    /// people ordered by popularity
    person_popular(): GET "/person/popular" => Page<PersonShort> {
        params { language: Language, page: u32 }
    }
}

endpoint! {
    /// a person's recent changes
    person_changes(id: u64): GET "/person/{id}/changes" => Changes {
        params { start_date: Date, end_date: Date, page: u32 }
    }
}

endpoint! {
    /// a person's combined movie and series credits
    person_combined_credits(id: u64): GET "/person/{id}/combined_credits" => PersonCredits {
        params { language: Language }
    }
}

endpoint! {
    /// a person's movie credits
    person_movie_credits(id: u64): GET "/person/{id}/movie_credits" => PersonCredits {
        params { language: Language }
    }
}

endpoint! {
    /// a person's series credits
    person_tv_credits(id: u64): GET "/person/{id}/tv_credits" => PersonCredits {
        params { language: Language }
    }
}

endpoint! {
    /// a person's ids on other databases
    person_external_ids(id: u64): GET "/person/{id}/external_ids" => ExternalIds
}

endpoint! {
    /// a person's profile images
    person_images(id: u64): GET "/person/{id}/images" => PersonImages
}

endpoint! {
    /// the images a person is tagged in
    #[deprecated = "TMDB deprecated this endpoint; it no longer returns data"]
    person_tagged_images(id: u64): GET "/person/{id}/tagged_images" => Page<TaggedImage> {
        params { page: u32 }
    }
}

endpoint! {
    /// a person's translations
    person_translations(id: u64): GET "/person/{id}/translations" => PersonTranslations
}
