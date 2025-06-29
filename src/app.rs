use crate::{
    pages::{
        contact::Contact, landing_page::LandingPage, login::Login, not_found::NotFound,
        register::Register, room_details::RoomDetails, search_result::SearchResult,
    },
    Home,
};

use leptoaster::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};
use reactive_stores::Store;


#[derive(Clone, Debug, Default, Store)]
pub struct UserState {
    email: Option<String>,
    fullname: Option<String>,
    id: Option<u32>,
    role: Option<String>,
}

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    user: UserState,
}

#[component]
pub fn App() -> impl IntoView {
    provide_toaster();
    provide_meta_context();
    provide_context(Store::new(GlobalState::default()));

    view! {
    // <Stylesheet id="leptos" href="/style/output.css"/>

    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Toaster stacked={true} />
    <Router>
        <Routes fallback=NotFound>
            <Route path=StaticSegment("") view=LandingPage/>
            <Route path=StaticSegment("home") view=Home/>
            <Route path=StaticSegment("contact") view=Contact/>
            <Route path=StaticSegment("register") view=Register/>
            <Route path=StaticSegment("login") view=Login/>
            <Route path=StaticSegment("search") view=SearchResult/>
            <Route path=path!("rooms/:id") view=RoomDetails/>
        </Routes>
    </Router>
     }
}
