use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub username: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub order_by: Option<String>,
}
