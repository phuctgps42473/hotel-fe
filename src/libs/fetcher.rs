use eyre::Context;
use js_sys::JSON;
use leptos::prelude::window;
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::{config::env::BASE_URL, ApiResponse};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

pub async fn fetch<T>(
    enpoint: &str,
    method: &str,
    body: &JsValue,
) -> Result<ApiResponse<T>, JsValue>
where
    T: for<'a> Deserialize<'a>,
{
    let default_options = RequestInit::new();
    default_options.set_credentials(web_sys::RequestCredentials::Include);
    default_options.set_mode(RequestMode::Cors);
    default_options.set_method(method);
    default_options.set_body(body);

    let request =
        Request::new_with_str_and_init(&format!("{}/{}", BASE_URL, enpoint), &default_options)?;
    request.headers().set("Content-Type", "application/json")?;
    if let Some(access_token) = window().local_storage()?.unwrap().get("accessToken")? {
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", access_token))?;
    }

    let promise = window().fetch_with_request(&request);
    let response: Response = JsFuture::from(promise).await?.dyn_into()?;
    let json = JsFuture::from(response.json()?).await?;
    let stringified = JSON::stringify(&json)?;
    let rust_json = stringified.as_string().unwrap();
    let response: ApiResponse<T> = serde_json::from_str(&rust_json)
        .context("Cannot deserialize response's json")
        .unwrap();

    Ok(response)
}

async fn refresh_token() {
    // default_options.set_method("GET");
    // Request::new_with_str_and_init(&format!("{}/refresh-token", BASE_URL), &default_options)?;
    // let promise = window().fetch_with_request(&request);
    // let token_response: Response = JsFuture::from(promise).await?.dyn_into()?;
    // let json = JsFuture::from(token_response.json()?).await?;
    // let stringified = JSON::stringify(&json)?;
    // let rust_json = stringified.as_string().unwrap();
    // let res: ApiResponse<TokenResponse> = serde_json::from_str(&rust_json)
    //     .context("Cannot deserialize response's json")
    //     .unwrap();
    // if (res.code == 400) {
    //     window().local_storage()?.unwrap().delete("accessToken")?;
    //     let currentUrl = format!(
    //         "{}{}",
    //         window().location().pathname(),
    //         window().location().search()
    //     );
    //     let encodedUrl = encode_uri_component(&currentUrl);
    //  window().location().assign(format!("/login?redirect=${}",encodedUrl);
    // }
    // window().local_storage()?.unwrap().set("accessToken", res);
    todo!();
}
