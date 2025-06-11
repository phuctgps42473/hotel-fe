use crate::{
    pages::{contact::Contact, landing_page::LandingPage, register::Register},
    Home,
};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/style/output.css"/>

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=LandingPage/>
                <Route path=StaticSegment("home") view=Home/>
                <Route path=StaticSegment("contact") view=Contact/>
                <Route path=StaticSegment("register") view=Register/>
            </Routes>
        </Router>
    }
}
