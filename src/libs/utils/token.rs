use eyre::Result;
use leptos::prelude::window;
use wasm_bindgen::JsValue;

pub fn get_access_token() -> Option<String> {
    window().local_storage().unwrap().unwrap().get_item("accessToken").unwrap()
}

pub fn clear_access_token() -> Result<(), JsValue> {
    window().local_storage()?.unwrap().remove_item("accessToken")
}