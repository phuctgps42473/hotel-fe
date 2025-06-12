use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let (is_drawer_open, set_is_drawer_open) = signal(false);
    view! {
                <header class="navbar bg-base-100 shadow-sm px-4 md:px-8 lg:px-16 py-4">
                    <div class="flex-1">
                        <a class="text-2xl font-bold text-base-content" href="#">"ELARIS HOTEL"</a>
                    </div>
                    <div class="flex-none hidden lg:flex">
                        <ul class="menu menu-horizontal p-0">
                            <li><a class="font-semibold text-base-content hover:text-primary" href="/">"Trang chủ"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#">"Loại phòng"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#">"Khám phá"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#">"Thông tin"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#">"Liên hệ"</a></li>
                        </ul>
                    </div>
                    <div class="flex-none hidden lg:flex ml-4">
                        <a class="btn btn-ghost text-primary hover:text-primary-focus font-semibold" href="#">"Đăng nhập"</a>
                        <a class="btn btn-primary text-primary-content font-semibold" href="/register">"Đăng ký"</a>
                    </div>
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
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Trang chủ"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Loại phòng"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Khám phá"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Thông tin"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Liên hệ"</a></li>
                            <li class="mt-4"><a class="btn btn-ghost text-primary hover:text-primary-focus font-semibold" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng nhập"</a></li>
                            <li><a class="btn btn-primary text-primary-content font-semibold mt-2" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng ký"</a></li>
                        </ul>
                    </div>
                </div>
    }
}
