use serde::Deserialize;

use crate::endpoints::search::CompanyShort;
use crate::models::{AlternativeNames, Images};

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyDetails {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub headquarters: Option<String>,
    pub homepage: String,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
    pub parent_company: Option<CompanyShort>,
}

endpoint! {
    /// a company's details
    company(id: u64): GET "/company/{}" => CompanyDetails
}

endpoint! {
    /// a company's alternative names
    company_alternative_names(id: u64): GET "/company/{}/alternative_names" => AlternativeNames
}

endpoint! {
    /// a company's logos
    company_images(id: u64): GET "/company/{}/images" => Images
}
