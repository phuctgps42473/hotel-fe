pub const BASE_URL: &'static str = "http://localhost:8080/api";

pub fn get_base_url() -> &'static str {
    if cfg!(debug_assertions) {
        "http://localhost:8080/api"
    } else {
        "https://datn-bug-six.onrender.com/api"
    }
}
