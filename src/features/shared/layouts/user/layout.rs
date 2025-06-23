use leptos::prelude::*;

use crate::features::shared::layouts::user::{footer::Footer, navbar::Navbar};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
          <header class="block w-full fixed bottom-0 left-0 md:relative">
            <Navbar />
          </header>
          {children()}
          <Footer />
    }
}
