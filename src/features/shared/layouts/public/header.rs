use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, UserState},
    libs::{fetcher::fetch, utils::token::clear_access_token},
};

#[component]
pub fn Header() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let (is_drawer_open, set_is_drawer_open) = signal(false);

    let logout = move || {
        spawn_local(async move {
            match fetch::<(), ()>("logout", "GET", None).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        clear_access_token().unwrap();
                        state.write().user = None;
                        window().location().set_href("/login").unwrap();
                    }
                }
            }
        });
    };

    view! {
                <header class="navbar bg-base-100 shadow-sm px-4 md:px-8 lg:px-16 py-4">
                    <div class="flex-1">
                        <a class="text-2xl font-bold text-base-content" href="/">"ELARIS HOTEL"</a>
                    </div>
                    <div class="flex-none hidden lg:flex">
                        <ul class="menu menu-horizontal p-0">
                            // <li><a class="font-semibold text-base-content hover:text-primary" href="/">"Trang chủ"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/home">"Khám phá"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/about-us">"Thông tin"</a></li>
                            // <li><a class="font-semibold text-base-content hover:text-primary" href="/">"Liên hệ"</a></li>
                        </ul>
                    </div>

                    <Show
                      when={move || state.read().user.is_some()}
                      fallback={|| view! {
                        <div class="flex-none hidden lg:flex ml-4">
                            <a class="btn btn-ghost text-primary hover:text-primary-focus font-semibold" href="/login">"Đăng nhập"</a>
                            <a class="btn btn-primary text-primary-content font-semibold" href="/register">"Đăng ký"</a>
                        </div>
                      }}
                    >
                      <div class="flex-none hidden lg:flex ml-4">
                          <UserProfile user={state.get().user.unwrap()} logout={logout} />
                      </div>
                    </Show>

                    <div class="flex-none lg:hidden">
                        <label
                            // for_="my-drawer-3"
                            class="btn btn-square btn-ghost"
                            on:click=move |_| set_is_drawer_open.update(|open| *open = !*open)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                        </label>
                    </div>
                </header>

                <div class="drawer">
                    <input
                        id="my-drawer-3"
                        type="checkbox"
                        class="drawer-toggle"
                        checked={is_drawer_open}
                        on:change=move |_| set_is_drawer_open.update(|open| *open = !*open)
                    />
                    <div class="drawer-content flex flex-col">
                    </div>
                    <div class="drawer-side z-50">
                        <label
                            // for_="my-drawer-3"
                            aria-label="close sidebar"
                            class="drawer-overlay"
                            on:click=move |_| set_is_drawer_open.set(false)
                        ></label>
                        <ul class="menu p-4 w-80 bg-base-100 h-full">
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/" on:click=move |_| set_is_drawer_open.set(false)>"Trang chủ"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/home" on:click=move |_| set_is_drawer_open.set(false)>"Khám phá"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/about-us" on:click=move |_| set_is_drawer_open.set(false)>"Thông tin"</a></li>

                             <Show
                              when={move || state.read().user.is_some()}
                              fallback={move || view! {
                                <li class="mt-4"><a class="btn btn-ghost text-primary hover:text-primary-focus font-semibold" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng nhập"</a></li>
                                <li><a class="btn btn-primary text-primary-content font-semibold mt-2" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng ký"</a></li>
                              }}
                            >
                              <li><a class="font-semibold text-base-content hover:text-primary" href="/profile" on:click=move |_| set_is_drawer_open.set(false)>"Cá nhân"</a></li>
                              <li>
                                <button
                                  class="font-semibold text-base-content hover:text-primary"
                                  on:click=move |_| {
                                    set_is_drawer_open.set(false);
                                    logout();
                                  }
                                >"Đăng xuất"</button></li>
                            </Show>
                        </ul>
                    </div>
                </div>
    }
}

#[component]
fn UserProfile(user: UserState, logout: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="dropdown dropdown-end">
            // Avatar kích hoạt dropdown
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar transition-transform duration-300 hover:scale-110">
                <div class="w-10 rounded-full ring-2 ring-primary ring-offset-base-100 ring-offset-2">
                    <img alt="User Avatar" src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=2944&auto=format&fit=crop" />
                </div>
            </div>

            // Nội dung dropdown, sử dụng đúng màu từ palette
            <ul tabindex="0" class="menu dropdown-content mt-4 z-[1] p-2 shadow-xl bg-base-100 border border-base-300 rounded-box w-60">
                // Header của menu
                <div class="px-4 py-2">
                    <div class="font-bold text-base-content">{user.fullname}</div>
                    <div class="text-sm text-base-content/70 -mt-1">{user.email}</div>
                </div>
                <div class="divider my-1"></div>

                <li>
                    <a href="/profile" class="text-base-content hover:bg-primary/10 hover:text-primary rounded-lg">
                        <i class="fas fa-user w-4 mr-2"></i>"Tài khoản của tôi"
                    </a>
                </li>
                <li>
                    <a href="/bookings" class="text-base-content hover:bg-primary/10 hover:text-primary rounded-lg">
                        <i class="fas fa-history w-4 mr-2"></i>"Lịch sử đặt phòng"
                    </a>
                </li>
                <div class="divider my-1"></div>
                <li>
                    <button on:click={move |_| logout()} class="text-error hover:bg-error/10 hover:font-semibold rounded-lg">
                        <i class="fas fa-sign-out-alt w-4 mr-2"></i>"Đăng xuất"
                    </button>
                </li>
            </ul>
        </div>
    }
}
