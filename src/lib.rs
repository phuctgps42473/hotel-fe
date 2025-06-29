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
  pub message: String
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
  pub status: String,
  pub code: u16,
  pub data: Option<T>,
  pub error: Option<ErrorMessage>
}