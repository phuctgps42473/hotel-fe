use eyre::Result;
use leptos::prelude::window;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

pub fn fetcher() -> Client {
    let mut defautl_headers = HeaderMap::new();
    defautl_headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    let client_builder = reqwest::ClientBuilder::new();

    if let Some(storage) = window().local_storage().unwrap() {
        if let Some(access_token) = storage.get("accessToken").unwrap() {
            defautl_headers.append(
                header::AUTHORIZATION,
                HeaderValue::from_bytes(format!("Bearer {}", access_token).as_bytes()).unwrap(),
            );
        }
    }
    client_builder
        .default_headers(defautl_headers)
        .build()
        .unwrap()
}
