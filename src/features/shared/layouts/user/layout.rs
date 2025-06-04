use leptos::prelude::*;

use crate::features::{
    home::components::search_bar::SearchBar,
    shared::layouts::user::{footer::Footer, navbar::Navbar},
};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
          <header class="block w-full fixed bottom-0 left-0 md:relative">
            <Navbar />
            <SearchBar />
          </header>
          {children()}
          <Footer />
    }
}
