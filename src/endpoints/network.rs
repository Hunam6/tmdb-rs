use serde::Deserialize;

use crate::models::{AlternativeNames, Images};

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkDetails {
    pub id: u64,
    pub name: String,
    pub headquarters: Option<String>,
    pub homepage: String,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

endpoint! {
    /// a network's details
    network(id: u64): GET "/network/{}" => NetworkDetails
}

endpoint! {
    /// a network's alternative names
    network_alternative_names(id: u64): GET "/network/{}/alternative_names" => AlternativeNames
}

endpoint! {
    /// a network's logos
    network_images(id: u64): GET "/network/{}/images" => Images
}
