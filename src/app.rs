use crate::{
    pages::{
        about_us::AboutUs, booking::Booking, contact::Contact, landing_page::LandingPage,
        login::Login, not_found::NotFound, payment::PaymentCallBack, register::Register,
        room_details::RoomDetails, search_result::SearchResult,
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
use serde::Deserialize;

#[derive(Clone, Debug, Default, Store, Deserialize)]
pub struct UserState {
    pub email: String,
    pub fullname: String,
    pub id: u32,
    pub role: String,
}

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    pub user: Option<UserState>,
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
                <Route path=path!("booking/:id") view=Booking/>
                <Route path=path!("payment/callback") view=PaymentCallBack/>
                <Route path=StaticSegment("about-us") view=AboutUs/>
            </Routes>
        </Router>
         }
}
