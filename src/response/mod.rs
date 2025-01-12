use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize, Default)]
pub struct EmptyResponse {}

#[derive(Debug, Deserialize, Default)]
pub struct PaginatedResponse<T> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Debug, Deserialize, Default)]
pub struct Meta {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub total: u64,
    pub count: u64,
    pub per_page: u64,
    pub current_page: u64,
    pub total_pages: u64,
}

pub trait Paginated {
    fn set_page(&mut self, page: u64);
    fn get_page(&self) -> u64;
    fn max_page(&self) -> u64;
}

