use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use reactive_stores::Store;

use crate::{app::GlobalState, features::profile::{BookingHistory, Favorites, ProfileInformation, Settings}, layouts::public::Layout};

#[derive(Clone, PartialEq, Copy)]
enum ProfileTab {
    Information,
    Bookings,
    Favorites,
    Settings,
}

// === CÁC COMPONENT ICON ĐỂ TÁI SỬ DỤNG ===
#[component]
fn UserIcon() -> impl IntoView {
    /* SVG code */
    view! { <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" /></svg> }
}
#[component]
fn ListIcon() -> impl IntoView {
    /* SVG code */
    view! { <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h7.5M8.25 12h7.5m-7.5 5.25h7.5M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" /></svg> }
}
#[component]
fn HeartIcon() -> impl IntoView {
    /* SVG code */
    view! { <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" /></svg> }
}
#[component]
fn SettingsIcon() -> impl IntoView {
    /* SVG code */
    view! { <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.438 1.001s.145.761.438 1.001l1.003.827c.424.35.534.954.26 1.431l-1.296 2.247a1.125 1.125 0 01-1.37.49l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.063-.374-.313-.686-.645-.87a6.52 6.52 0 01-.22-.127c-.324-.196-.72-.257-1.075-.124l-1.217.456a1.125 1.125 0 01-1.37-.49l-1.296-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.437-1.001s-.145-.761-.437-1.001l-1.004-.827a1.125 1.125 0 01-.26-1.431l1.296-2.247a1.125 1.125 0 011.37-.49l1.217.456c.355.133.75.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg> }
}

// === COMPONENT CHÍNH CỦA TRANG ===
#[component]
pub fn UserProfile() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let (active_tab, set_active_tab) = signal(ProfileTab::Information);

    Effect::new(move || {
      // if state.read().user.is_none() {
      //   use_navigate()("/login", NavigateOptions::default());
      //   return;
      // }
    });

    view! {
      <Layout>
        <div class="min-h-screen bg-base-200 py-8 md:py-12">
            <div class="container mx-auto max-w-7xl px-4">
                <h1 class="text-xl md:text-2xl font-bold text-primary mb-8">Tài Khoản Của Tôi</h1>

                <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                    // Left Sidebar Navigation
                    <aside class="lg:col-span-1">
                        <div class="bg-base-100 p-4 rounded-[var(--radius-selector)] shadow-md">
                            <ul class="space-y-2">
                                <li>
                                    <button
                                        on:click=move |_| set_active_tab.set(ProfileTab::Information)
                                        class="w-full flex items-center gap-3 p-3 rounded-[var(--radius-box)] font-semibold transition"
                                        class:bg-primary=move || active_tab.read() == ProfileTab::Information
                                        class:text-primary-content=move || active_tab.read() == ProfileTab::Information
                                        class:hover:bg-base-200=move || active_tab.read() != ProfileTab::Information
                                    >
                                        <UserIcon /> "Thông Tin Cá Nhân"
                                    </button>
                                </li>
                                <li>
                                    <button
                                        on:click=move |_| set_active_tab.set(ProfileTab::Bookings)
                                        class="w-full flex items-center gap-3 p-3 rounded-[var(--radius-box)] font-semibold transition"
                                        class:bg-primary=move || active_tab.read() == ProfileTab::Bookings
                                        class:text-primary-content=move || active_tab.read() == ProfileTab::Bookings
                                        class:hover:bg-base-200=move || active_tab.read() != ProfileTab::Bookings
                                    >
                                        <ListIcon /> "Lịch Sử Đặt Phòng"
                                    </button>
                                </li>
                                 <li>
                                    <button
                                        on:click=move |_| set_active_tab.set(ProfileTab::Favorites)
                                        class="w-full flex items-center gap-3 p-3 rounded-[var(--radius-box)] font-semibold transition"
                                        class:bg-primary=move || active_tab.read() == ProfileTab::Favorites
                                        class:text-primary-content=move || active_tab.read() == ProfileTab::Favorites
                                        class:hover:bg-base-200=move || active_tab.read() != ProfileTab::Favorites
                                    >
                                        <HeartIcon /> "Phòng Yêu Thích"
                                    </button>
                                </li>
                                <li>
                                     <button
                                        on:click=move |_| set_active_tab.set(ProfileTab::Settings)
                                        class="w-full flex items-center gap-3 p-3 rounded-[var(--radius-box)] font-semibold transition"
                                        class:bg-primary=move || active_tab.read() == ProfileTab::Settings
                                        class:text-primary-content=move || active_tab.read() == ProfileTab::Settings
                                        class:hover:bg-base-200=move || active_tab.read() != ProfileTab::Settings
                                    >
                                        <SettingsIcon /> "Cài Đặt"
                                    </button>
                                </li>
                            </ul>
                        </div>
                    </aside>

                    // Right Content Area
                    <main class="lg:col-span-3">
                        <div class="bg-base-100 p-6 md:p-8 rounded-[var(--radius-selector)] shadow-md">
                            {move || match *active_tab.read() {
                                ProfileTab::Information => view! { <ProfileInformation
                                // id={state.read().user.as_ref().unwrap().id}
                                id={5}
                                /> }.into_any(),
                                ProfileTab::Bookings => view! { <BookingHistory /> }.into_any(),
                                ProfileTab::Favorites => view! { <Favorites /> }.into_any(),
                                ProfileTab::Settings => view! { <Settings /> }.into_any(),
                            }}
                        </div>
                    </main>
                </div>
            </div>
        </div>
      </Layout>
    }
}
