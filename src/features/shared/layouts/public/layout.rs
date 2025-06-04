use leptos::prelude::*;

use crate::features::shared::layouts::public::{footer::Footer, header::Header};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
      <Header />
      {children()}
      <Footer />
    }
}
