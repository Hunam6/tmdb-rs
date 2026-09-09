use serde::Deserialize;

use crate::Profile;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorDetails {
    pub name: String,
    pub username: String,
    #[serde(rename = "avatar_path")]
    pub avatar: Option<Profile>,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Review {
    pub id: String,
    pub author: String,
    pub author_details: AuthorDetails,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
}

endpoint! {
    /// a review by id
    review(id: &str): GET "/review/{id}" => Review
}
