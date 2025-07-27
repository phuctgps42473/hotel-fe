use leptos::prelude::window;

pub fn get_access_token() -> Option<String> {
    window().local_storage().unwrap().unwrap().get_item("accessToken").unwrap()
}
