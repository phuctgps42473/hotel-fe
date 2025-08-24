use serde::Deserialize;

pub mod app;
pub mod config;
pub mod features;
pub mod libs;
pub mod pages;

pub use pages::Home;

pub use features::shared::layouts;

#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub data: Option<T>,
    pub error: Option<ErrorMessage>,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
    pub last: bool,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    #[serde(rename = "totalElements")]
    pub total_elements: i64,
    pub size: i32,
    pub number: i32,
    pub sort: Sort,
    pub first: bool,
    #[serde(rename = "numberOfElements")]
    pub number_of_elements: i32,
    pub empty: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoomType {
    pub id: u64,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "defaultPrice")]
    pub default_price: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Pageable {
    #[serde(rename = "pageNumber")]
    pub page_number: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u16,
    pub sort: Sort,
    pub offset: u64,
    pub paged: bool,
    pub unpaged: bool,
}

#[derive(Debug, Deserialize)]
pub struct Sort {
    pub empty: bool,
    pub sorted: bool,
    pub unsorted: bool,
}
