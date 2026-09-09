use serde::Deserialize;

use crate::models::{AlternativeNames, Images};
use crate::Logo;

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkDetails {
    pub id: u64,
    pub name: String,
    pub headquarters: Option<String>,
    pub homepage: String,
    #[serde(rename = "logo_path")]
    pub logo: Option<Logo>,
    pub origin_country: Option<String>,
}

endpoint! {
    /// a network's details
    network(id: u64): GET "/network/{id}" => NetworkDetails
}

endpoint! {
    /// a network's alternative names
    network_alternative_names(id: u64): GET "/network/{id}/alternative_names" => AlternativeNames
}

endpoint! {
    /// a network's logos
    network_images(id: u64): GET "/network/{id}/images" => Images
}
