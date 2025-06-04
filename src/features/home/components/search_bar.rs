use leptos::prelude::*;

#[derive(Default)]
struct SearchState {
    location: String,
    checkin_date: String,
    checkout_date: String,
    guest_count: u8,
}

#[component]
pub fn SearchBar() -> impl IntoView {
    let state = RwSignal::new(SearchState::default());
    let (location, set_location) = create_slice(
        state,
        |state| state.location.clone(),
        |state, loc| state.location = loc,
    );

    let (checkin_date, set_checkin_date) = create_slice(
        state,
        |state| state.checkin_date.clone(),
        |state, new_checkin_date| state.checkin_date = new_checkin_date,
    );

    let (checkout_date, set_checkout_date) = create_slice(
        state,
        |state| state.checkout_date.clone(),
        |state, new_checkout_date| state.checkout_date = new_checkout_date,
    );

    let (guest_count, set_guest_count) = create_slice(
        state,
        |state| state.guest_count,
        |state, count| state.location = count,
    );

    let handle_search = move |_| {
      leptos::logging::log!("{}", location.get());
      leptos::logging::log!("{}", checkin_date.get());
      leptos::logging::log!("{}", checkout_date.get());
      leptos::logging::log!("{}", guest_count.get());
    };

    view! {
        <div class="hidden md:flex items-center border border-base-300 rounded-full shadow-md hover:shadow-lg transition-shadow duration-200 mx-auto max-w-3xl bg-base-100 my-4">
            <div class="flex-1 px-6 py-3 cursor-pointer hover:bg-base-200 rounded-full">
                <p class="text-xs font-bold uppercase">"Where"</p>
                <div class="dropdown">
                  <input
                      tabindex="0"
                      type="text"
                      value={location}
                      placeholder="Search destinations"
                      class="text-sm w-full text-base-content text-opacity-70 bg-transparent outline-none border-none placeholder-base-content placeholder-opacity-50"
                      on:input:target={move |ev| set_location.set(ev.target().value())}
                  />
                  <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-1 w-full p-2 shadow-sm">
                    <li><a>Item 1</a></li>
                    <li><a>Item 2</a></li>
                  </ul>
                </div>
            </div>

            <div class="border-l border-base-300 h-8 self-center"></div>

            <div class="px-6 py-3 cursor-pointer hover:bg-base-200 rounded-full">
                <p class="text-xs font-bold uppercase">"Check in"</p>
                <input
                 on:change:target={move |ev| set_checkin_date.set(ev.target().value())}
                 type="date" class="input border-none shadow-none outline-none"/>
            </div>

            <div class="border-l border-base-300 h-8 self-center"></div>

            <div class="px-6 py-3 cursor-pointer hover:bg-base-200 rounded-full">
                <p class="text-xs font-bold uppercase">"Check out"</p>
                <input
                  on:change:target={move |ev| set_checkout_date.set(ev.target().value())}
                  type="date" class="input border-none shadow-none outline-none"/>
            </div>

            <div class="border-l border-base-300 h-8 self-center"></div>

            <div class="flex items-center pl-6 pr-2 py-2 cursor-pointer hover:bg-base-200 rounded-full">
                 <div class="mr-3">
                    <p class="text-xs font-bold uppercase">"Who"</p>
                    <div class="dropdown dropdown-center">
                    <span tabindex="0" role="button" class="text-sm text-base-content text-opacity-70">"Add guests"</span>
                    <div tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
                        <div class="flex flex-row items-center justify-between p-2">
                          <button class="btn">-</button>
                          <div class="flex flex-col items-center justify-center">
                            <p>Adult</p>
                            <p>{guest_count.get()}</p>
                          </div>
                          <button class="btn">+</button>
                        </div>
                        <div class="flex flex-row items-center justify-between p-2">
                          <button class="btn">-</button>
                          <div class="flex flex-col items-center justify-center">
                            <p>Children</p>
                            <p>{guest_count.get()}</p>
                          </div>
                          <button class="btn">+</button>
                        </div>
                        <div class="flex flex-row items-center justify-between p-2">
                          <button class="btn">-</button>
                          <div class="flex flex-col items-center justify-center">
                            <p>Infant</p>
                            <p>{guest_count.get()}</p>
                          </div>
                          <button class="btn">+</button>
                        </div>
                    </div>
                  </div>
                 </div>
                <button on:click={handle_search} class="btn btn-primary btn-circle btn-md">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                      <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
        </div>

        <div class="md:hidden p-4">
            <button class="btn btn-outline w-full justify-between border-base-300 rounded-full shadow hover:shadow-md transition-shadow duration-200 h-14"> // Làm nút cao hơn chút
                 <div class="flex items-center">
                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6 mr-3 text-primary"> // Icon to hơn và có màu
                       <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                     </svg>
                     <div class="text-left">
                         <p class="text-sm font-semibold">"Where to?"</p>
                         <p class="text-xs text-base-content text-opacity-60">"Anywhere • Any week • Add guests"</p>
                     </div>
                 </div>
                 <div class="border border-base-300 rounded-full p-1.5">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                       <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75" />
                    </svg>
                 </div>
            </button>
        </div>
    }
}
