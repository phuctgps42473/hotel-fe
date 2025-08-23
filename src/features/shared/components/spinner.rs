use leptos::prelude::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
      <span class="loading loading-ring loading-xl"></span>
    }
}
