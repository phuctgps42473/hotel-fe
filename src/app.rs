use crate::{
    pages::{
        contact::Contact, landing_page::LandingPage, login::Login, not_found::NotFound,
        register::Register, room_details::RoomDetails,
    },
    Home,
};

use leptoaster::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_toaster();
    provide_meta_context();

    view! {
    // <Stylesheet id="leptos" href="/style/output.css"/>

    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Toaster stacked={true} />
    <Router>
        <Routes fallback=|| "Page not found.">
            <Route path=StaticSegment("") view=LandingPage/>
            <Route path=StaticSegment("home") view=Home/>
            <Route path=StaticSegment("contact") view=Contact/>
            <Route path=StaticSegment("register") view=Register/>
            <Route path=StaticSegment("login") view=Login/>
            <ParentRoute path=path!("rooms") view=NotFound>
              <Route path=path!(":id") view=RoomDetails/>
              <Route path=path!("") view=|| "Not Found"/>
            </ParentRoute>
        </Routes>
    </Router>
     }
}
