use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};

use crate::features::shared::components::date_range_picker::DateRangePicker;

#[component]
pub fn SearchBar(
    set_date_range: WriteSignal<String>,
    set_room_type: WriteSignal<String>,
    set_guest_count: WriteSignal<u8>,
    handle_search_rooms: impl Fn() + 'static,
) -> impl IntoView {
    let handle_room_type_change = move |e: Event| {
        set_room_type.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap()
                .value(),
        );
    };
    let handle_guest_count = move |e: Event| {
        set_guest_count.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<u8>()
                .unwrap(),
        );
    };

    view! {
      <div class="bg-base-100 bg-opacity-90 rounded-xl p-6 md:p-8 shadow-lg flex flex-col md:flex-row items-center justify-around gap-4 mx-auto max-w-5xl">
          <div class="flex items-center space-x-2 w-full md:w-auto">
              <i class="fas fa-calendar-alt text-teal-500 text-xl"></i>
              <div>
                  <label for_="checkin" class="block text-gray-600 text-sm">"Ngày nhận phòng"</label> // Giữ text-gray-600 nếu muốn màu xám cụ thể này
                  <DateRangePicker date_range_setter={set_date_range} id={String::from("checkin")} />
              </div>
          </div>
          <div class="flex items-center space-x-2 w-full md:w-auto">
              <i class="fas fa-bed text-primary text-xl"></i>
              <div>
                  <label for_="room-type" class="block text-gray-600 text-sm">"Loại phòng"</label> // Giữ text-gray-600
                  <select on:change={handle_room_type_change} id="room-type" class="select select-bordered w-full text-base-content bg-transparent border-none p-0 h-auto min-h-0">
                      <option selected>"Standard"</option>
                      <option>"Deluxe"</option>
                      <option>"Suite"</option>
                  </select>
              </div>
          </div>
          <div class="flex items-center space-x-2 w-full md:w-auto">
              <i class="fas fa-user-friends text-primary text-xl"></i>
              <div>
                  <label for_="guests" class="block text-gray-600 text-sm">"Số người"</label>
                  <input on:change={handle_guest_count} type="number" min=1 step=1 id="guests" class="w-full text-base-content bg-transparent border-none p-0 h-auto min-h-0" />
              </div>
          </div>
          <button on:click={move |_| handle_search_rooms()} class="btn btn-primary text-primary-content w-full md:w-auto text-lg px-8 py-3 rounded-full font-semibold md:ml-4"> // btn-primary tự động dùng bg-primary và hover
              "Tìm kiếm"
          </button>
      </div>
    }
}
