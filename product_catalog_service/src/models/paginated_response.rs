use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub data: Vec<T>,
}
