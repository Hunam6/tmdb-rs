use serde::Deserialize;

use crate::models::{AlternativeNames, Images};
use crate::Logo;

/// one company in a search response
#[derive(Debug, Clone, Deserialize)]
pub struct CompanyShort {
    pub id: u64,
    pub name: String,
    #[serde(rename = "logo_path")]
    pub logo: Option<Logo>,
    pub origin_country: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyDetails {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub headquarters: Option<String>,
    #[serde(rename = "homepage")]
    pub website: String,
    #[serde(rename = "logo_path")]
    pub logo: Option<Logo>,
    pub origin_country: Option<String>,
    pub parent_company: Option<CompanyShort>,
}

endpoint! {
    /// a company's details
    company(id: u64): GET "/company/{id}" => CompanyDetails
}

endpoint! {
    /// a company's alternative names
    company_alternative_names(id: u64): GET "/company/{id}/alternative_names" => AlternativeNames
}

endpoint! {
    /// a company's logos
    company_images(id: u64): GET "/company/{id}/images" => Images
}
