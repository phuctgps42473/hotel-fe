use eyre::{Context, Result};
use js_sys::JSON;
use leptos::prelude::window;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::{config::env::get_base_url, ApiResponse};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

async fn refresh_token() -> Result<(), JsValue> {
    let request = Request::new_with_str(&format!("{}/api/auth/refresh-token", get_base_url()))?;
    request.headers().set("Content-Type", "application/json")?;

    let promise = window().fetch_with_request(&request);
    let response: Response = JsFuture::from(promise).await?.dyn_into()?;
    let json = JsFuture::from(response.json()?).await?;
    let stringified = JSON::stringify(&json)?;
    let rust_json = stringified.as_string().unwrap();
    let res: ApiResponse<TokenResponse> = serde_json::from_str(&rust_json)
        .context("Cannot deserialize response's json")
        .unwrap();
    if res.code == 200 {
        window()
            .local_storage()?
            .unwrap()
            .set("accessToken", &res.data.unwrap().access_token)?;
        Ok(())
    } else {
        window().location().assign("/login")
    }
}

pub async fn fetch<Req, Res>(
    endpoint: &str,
    method: &str,
    body: Option<Req>,
) -> Result<ApiResponse<Res>, JsValue>
where
    Req: Serialize + Clone,
    Res: for<'a> Deserialize<'a>,
{
    Box::pin(async move {
        let default_options = RequestInit::new();
        default_options.set_credentials(web_sys::RequestCredentials::Include);
        default_options.set_mode(RequestMode::Cors);
        default_options.set_method(method);
        if let Some(req_body) = body.clone() {
            let body = &JsValue::from_str(&serde_json::to_string(&req_body).unwrap());
            default_options.set_body(body.as_ref());
        }

        let request = Request::new_with_str_and_init(
            &format!("{}/{}", get_base_url(), endpoint),
            &default_options,
        )?;
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
        let response: ApiResponse<Res> = serde_json::from_str(&rust_json)
            .context("Cannot deserialize response's json")
            .unwrap();

        if response.code == 401 && !endpoint.contains("refresh-token") {
            match refresh_token().await {
                Ok(_) => fetch::<Req, Res>(endpoint, method, body).await,
                Err(e) => Err(e),
            };
        }

        Ok(response)
    })
    .await
}

pub async fn fetch2<Req, Res>(
    enpoint: &str,
    method: &str,
    body: Option<Req>,
) -> Result<Res, JsValue>
where
    Req: Serialize,
    Res: for<'a> Deserialize<'a>,
{
    let default_options = RequestInit::new();
    default_options.set_credentials(web_sys::RequestCredentials::Include);
    default_options.set_mode(RequestMode::Cors);
    default_options.set_method(method);
    if let Some(req_body) = body {
        let body = &JsValue::from_str(&serde_json::to_string(&req_body).unwrap());
        default_options.set_body(body);
    }

    let request = Request::new_with_str_and_init(
        &format!("{}/{}", get_base_url(), enpoint),
        &default_options,
    )?;
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
    let response: Res = serde_json::from_str(&rust_json)
        .context("Cannot deserialize response's json")
        .unwrap();

    Ok(response)
}
