use serde::Deserialize;

/// one page of a paginated list endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct Page<T> {
    pub page: u32,
    pub results: Vec<T>,
    pub total_pages: u32,
    pub total_results: u32,
}

impl<T> Page<T> {
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }
}
