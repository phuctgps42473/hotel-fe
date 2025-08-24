use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    hooks::{use_navigate, use_query},
    params::Params,
    NavigateOptions,
};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{
    features::shared::components::{
        date_range_picker::DateRangePicker, input::Input, spinner::Spinner,
    },
    layouts::public::{footer::Footer, header::Header},
    libs::{
        fetcher::{fetch, fetch2},
        utils::{currency_utils, date_utils},
    },
    pages::room_details::Room,
    Pageable, PaginatedResponse,
};

#[derive(Deserialize, Debug, Clone)]
struct RoomType {
    id: Option<u64>,
    #[serde(rename = "typeName")]
    name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Params, PartialEq, Clone)]
struct RoomSearch {
    fromDate: Option<String>,
    toDate: Option<String>,
    minPrice: Option<u64>,
    maxPrice: Option<u64>,
    roomTypeId: Option<u64>,
    guest: Option<u8>,
    page: Option<u32>,
    size: Option<u16>,
    sort: Option<String>,
}

impl ToString for RoomSearch {
    fn to_string(&self) -> String {
        let from_date = self.fromDate.as_ref().unwrap();
        let to_date = self.toDate.as_ref().unwrap();
        let mut endpoint = format!("/search?fromDate={}&toDate={}", from_date, to_date);
        match (self.minPrice, self.maxPrice) {
            (Some(min), Some(max)) => {
                endpoint.push_str(&format!("&minPrice={}&maxPrice={}", min, max));
            }
            _ => {}
        }
        if let Some(room_type_id) = self.roomTypeId {
            endpoint.push_str(&format!("&roomTypeId={}", room_type_id));
        }
        if let Some(guest_count) = self.guest {
            endpoint.push_str(&format!("&guest={}", guest_count));
        }
        if let Some(page) = self.page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(size) = self.size {
            endpoint.push_str(&format!("&size={}", size));
        }
        if let Some(sort) = self.sort.as_ref() {
            endpoint.push_str(&format!("&sort={}", sort));
        }

        endpoint
    }
}

#[derive(Default)]
pub struct Pagination {
    pub page: Option<u32>,
    pub size: Option<u16>,
    pub sort: Option<(String, String)>,
}

impl From<Pageable> for Pagination {
    fn from(value: Pageable) -> Self {
        Self {
            page: Some(value.page_number),
            size: Some(value.page_size),
            sort: None,
        }
    }
}

#[component]
pub fn SearchResult() -> impl IntoView {
    console_error_panic_hook::set_once();
    let (date_range, set_date_range) = signal(String::new());
    let navigate = use_navigate();
    let (rooms, set_rooms) = signal(None);
    let (pagination, set_pagination) = signal(Pagination::default());
    let (price_error, set_price_error) = signal(None);
    let (room_types, set_room_types) = signal(vec![]);

    let query = use_query::<RoomSearch>();
    let (wery, set_wery) = signal(query.get().unwrap());

    Effect::new(move || {
        spawn_local(async move {
            match fetch2::<(), Vec<RoomType>>("admin/room-type", "GET", None).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => set_room_types.set(res),
            }
        });
    });

    let (last_page, set_last_page) = signal(0);
    let is_first_page = move || query.read().as_ref().unwrap().page.unwrap_or(0) == 0;
    let is_last_page = move || {
        pagination
            .read()
            .page
            .iter()
            .any(|p| *p as i32 == *last_page.read())
    };
    let nav = navigate.clone();
    Effect::new(move || {
        let query = query.read().as_ref().unwrap().clone();
        if query.fromDate.is_none() || query.toDate.is_none() {
            nav("/not-found", NavigateOptions::default());
            return;
        }
        set_date_range.set(format!(
            "{} to {}",
            query.fromDate.as_ref().unwrap(),
            query.toDate.as_ref().unwrap()
        ));

        spawn_local(async move {
            let from_date = query.fromDate.as_ref().unwrap();
            let to_date = query.toDate.as_ref().unwrap();
            let mut endpoint = format!("search?fromDate={}&toDate={}", from_date, to_date);
            match (query.minPrice, query.maxPrice) {
                (Some(min), Some(max)) => {
                    endpoint.push_str(&format!("&minPrice={}&maxPrice={}", min, max));
                }
                _ => {}
            }
            if let Some(room_type_id) = query.roomTypeId {
                endpoint.push_str(&format!("&roomTypeId={}", room_type_id));
            }
            if let Some(guest_count) = query.guest {
                endpoint.push_str(&format!("&guest={}", guest_count));
            }
            if let Some(page) = query.page {
                endpoint.push_str(&format!("&page={}", page));
            }
            if let Some(size) = query.size {
                endpoint.push_str(&format!("&size={}", size));
            }
            if let Some(sort) = &query.sort {
                endpoint.push_str(&format!("&sort={}", sort));
            }

            match fetch::<(), PaginatedResponse<Room>>(&endpoint, "GET", None).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        let data = res.data.unwrap();
                        set_rooms.set(Some(data.content));
                        set_pagination.set(Pagination::from(data.pageable));
                        if data.total_pages == 0 {
                            set_last_page.set(0);
                        } else {
                            set_last_page.set(data.total_pages - 1);
                        }
                    }
                }
            }
        });
    });

    let nav = navigate.clone();
    let handle_search = move || {
        let mut search = wery.get();
        let (from_date, to_date) = date_utils::get_check_in_out(date_range.get()).unwrap();
        search.fromDate = Some(from_date);
        search.toDate = Some(to_date);
        nav(&search.to_string(), NavigateOptions::default())
    };

    let nav = navigate.clone();
    let go_first_page = move |_| {
        let mut query = query.read().clone().unwrap();
        if let Some(current_page) = query.page.as_mut() {
            if *current_page == 0 {
                return;
            }
            *current_page = 0;
        }
        let endpoint = query.to_string();
        nav(&endpoint, NavigateOptions::default());
    };
    let nav = navigate.clone();
    let go_previous_page = move |_| {
        let mut query = query.read().clone().unwrap();
        if let Some(current_page) = query.page.as_mut() {
            if *current_page == 0 {
                return;
            }
            *current_page -= 1;
        }
        let endpoint = query.to_string();
        nav(&endpoint, NavigateOptions::default());
    };
    let nav = navigate.clone();
    let go_next_page = move |_| {
        let mut query = query.read().clone().unwrap();
        if let Some(current_page) = query.page.as_mut() {
            if *current_page == *last_page.read() as u32 {
                return;
            }
            *current_page += 1;
        }
        let endpoint = query.to_string();
        nav(&endpoint, NavigateOptions::default());
    };
    let nav = navigate.clone();
    let go_last_page = move |_| {
        let mut query = query.read().clone().unwrap();
        if let Some(current_page) = query.page.as_mut() {
            if *current_page == last_page.get() as u32 {
                return;
            }
            *current_page = last_page.get() as u32;
        }
        let endpoint = query.to_string();
        nav(&endpoint, NavigateOptions::default());
    };

    view! {
      <Header />
        <div class="min-h-screen bg-base-100 p-4 lg:p-8">
            // Vùng nội dung chính, căn giữa và có đổ bóng
            <div class="container mx-auto bg-base-100 rounded-lg shadow-lg overflow-hidden">
                // Khu vực chính: Sidebar lọc và danh sách kết quả
                <div class="grid grid-cols-1 lg:grid-cols-4 gap-6 p-6">
                    // Sidebar lọc (Cột bên trái)
                    <div class="lg:col-span-1 space-y-6">
                        // Thời Gian Nhận – Trả Phòng
                        <div class="bg-base-200 p-4 rounded-md shadow-sm">
                            <h3 class="text-lg font-semibold mb-3 text-base-content">"Hạng mục tìm kiếm"</h3>
                            <div class="space-y-2">
                                <div>
                                    <label class="block text-sm font-medium mb-1 text-base-content">"Thời gian"</label>
                                    <div class="relative">
                                        <DateRangePicker
                                          id={String::from("date-picker-search")}
                                          date_range_setter={set_date_range}
                                          custom_style={String::from("input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-sm placeholder-gray-500 bg-white")}
                                          exclude_ranges={vec![]}
                                        />
                                        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content opacity-70 pointer-events-none">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                                        </span>
                                    </div>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium mb-1 text-base-content">"Từ khoảng giá"</label>
                                      <Input
                                        id={"password".to_string()}
                                        input_type={"number".to_string()}
                                        label_name={"Từ khoảng giá".to_string()}
                                        min_len={Some(6)}
                                        max_len={Some(8)}
                                        value={wery.read().minPrice}
                                        error={price_error}
                                        handle_onchange={move |e| set_wery.write().minPrice = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<u64>().ok()}
                                        handle_onblur={move || {}}
                                        pattern={None}
                                        custom_style={Some("bg-white")}
                                      />
                                </div>
                                <div>
                                  <label class="block text-sm font-medium mb-1 text-base-content">"Đến khoảng giá"</label>
                                  <Input
                                    id={"max-price".to_string()}
                                    input_type={"number".to_string()}
                                    label_name={"Đến khoảng giá".to_string()}
                                    min_len={Some(6)}
                                    max_len={Some(8)}
                                    value={wery.read().maxPrice}
                                    error={price_error}
                                    handle_onchange={move |e| set_wery.write().maxPrice = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<u64>().ok()}
                                    handle_onblur={move || {}}
                                    pattern={None}
                                    custom_style={Some("bg-white")}
                                  />
                                </div>
                                <div>
                                  <label class="block text-sm font-medium mb-1 text-base-content">"Số lượng khách"</label>
                                  <input
                                  type="number"
                                  on:change={move |e| {
                                        set_wery.write().guest = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<u8>().ok()
                                  }}
                                  value={wery.read().guest}
                                  placeholder="Số lượng khách"
                                  min="1"
                                  max="2"
                                  pattern="^[12]$"
                                  class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-md placeholder-gray-500 bg-white"
                                  />
                                </div>
                                <div>
                                  <label class="block text-sm font-medium mb-1 text-base-content">"Loại phòng"</label>
                                  <div class="mb-2">
                                    <label class="flex items-center gap-2 cursor-pointer">
                                      <input
                                        on:change={move |_| set_wery.write().roomTypeId = None}
                                        checked={wery.read().roomTypeId.is_none()}
                                        type="radio" name="room-type" class="radio" />
                                      <span class="label-text">"Tất cả"</span>
                                    </label>
                                  </div>
                                  <For
                                    each={move || room_types.get()}
                                    key={|room| room.id.unwrap()}
                                    let(room)
                                  >
                                    <div class="mb-2">
                                      <label class="flex items-center gap-2 cursor-pointer">
                                        <input
                                          checked={wery.read().roomTypeId.is_some() && wery.read().roomTypeId.eq(&room.id)}
                                          on:change={move |_| set_wery.write().roomTypeId = room.id}
                                          type="radio" name="room-type" class="radio"
                                        />
                                        <span class="label-text">{room.name}</span>
                                      </label>
                                    </div>
                                  </For>
                                </div>
                              </div>
                            <button on:click={move |_| handle_search()} class="btn btn-primary w-full mt-6">"Kiểm Tra Phòng Trống"</button>
                        </div>

                        // Chủ Đề (Themes)
                        // <div class="collapse collapse-arrow bg-base-200 rounded-md shadow-sm">
                        //     <input type="checkbox" checked/> // checked by default
                        //     <div class="collapse-title text-lg font-semibold text-base-content">"Chủ Đề"</div>
                        //     <div class="collapse-content space-y-2 text-base-content">
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Hoạt động gần sông"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Gần trung tâm"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"View thành phố"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Nghỉ dưỡng"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Khu yên tĩnh"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Cảnh thiên nhiên"</span>
                        //         </label>
                        //         <label class="flex items-center gap-2 cursor-pointer">
                        //             <input type="checkbox" class="checkbox checkbox-primary"/>
                        //             <span class="label-text">"Gần địa điểm du lịch"</span>
                        //         </label>
                        //         <a href="#" class="link link-primary text-sm mt-2 block">"Xem thêm địa điểm khác"</a>
                        //     </div>
                        // </div>

                        // Chi Nhánh Khách Sạn (Hotel Branches)
                    //     <div class="collapse collapse-arrow bg-base-200 rounded-md shadow-sm">
                    //         <input type="checkbox" checked/> // checked by default
                    //         <div class="collapse-title text-lg font-semibold text-base-content">"Chi Nhánh Khách Sạn"</div>
                    //         <div class="collapse-content space-y-2 text-base-content">
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Quận 1 (TP.HCM)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Đà Nẵng (sông Hàn)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Hà Nội (hồ Tây)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Hội An (sông Hoài)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Cần Thơ (bến Ninh Kiều)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Đà Lạt (gần hồ Xuân Hương)"</span>
                    //             </label>
                    //             <label class="flex items-center gap-2 cursor-pointer">
                    //                 <input type="checkbox" class="checkbox checkbox-primary"/>
                    //                 <span class="label-text">"Nha Trang (gần biển)"</span>
                    //             </label>
                    //             <a href="#" class="link link-primary text-sm mt-2 block">"Hiển thị thêm chi nhánh"</a>
                    //         </div>
                    //     </div>
                    </div>

                    // Danh sách kết quả khách sạn (Cột bên phải)
                    <div class="lg:col-span-3 space-y-6">
                        <Show
                          when={move || {
                            let rooms = rooms.read();
                            rooms.is_some() && !rooms.as_ref().unwrap().is_empty()
                          }}
                          fallback={move ||
                            if rooms.read().is_none() {
                              view!{ <div class="p-2 flex items-center justify-center scale-[300%]"><Spinner /></div>}.into_any()
                            } else {
                              view!{ <div class="p-2 flex items-center justify-center scale-[300%] text-md">No Rooms Found.</div>}.into_any()
                            }
                          }
                        >
                          <For
                            each={move || rooms.get().unwrap()}
                            key={|room| room.id.as_ref().unwrap().clone()}
                            let(room)
                          >
                            <RoomCard room={room} />
                          </For>
                        </Show>

                        <div class="flex item-center justify-center">
                          <div class="join">
                            <button on:click={go_first_page} disabled={move || is_first_page()} class="join-item btn">"««"</button>
                            <button on:click={go_previous_page} disabled={move || is_first_page()} class="join-item btn">"«"</button>
                            <button class="join-item btn">"Trang" {move || pagination.read().page.unwrap_or(0) + 1}</button>
                            <button on:click={go_next_page} disabled={move || is_last_page()} class="join-item btn">"»"</button>
                            <button on:click={go_last_page} disabled={move || is_last_page()} class="join-item btn">"»»"</button>
                          </div>
                        </div>
                    </div>
                </div>
                <SuggestedRooms />
            </div>
        </div>
        <Footer />
    }
}

#[component]
fn RoomCard(room: Room) -> impl IntoView {
    let room_floor = String::from(&room.room_number.as_ref().unwrap()[0..1]);
    let href = format!("/rooms/{}", room.id.unwrap());
    view! {
        <a href={href} class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row cursor-pointer">
            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                <img src={room.image_url.unwrap_or("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fw0.peakpx.com%2Fwallpaper%2F247%2F443%2FHD-wallpaper-taj-hotel-mumbai.jpg&f=1&nofb=1&ipt=bd6d720da8e8c5bde64165dc3e8e3e4d6858e564d8367720676438bb486ae478".to_string())} alt="Hotel 1" class="object-cover w-full h-full"/>
            </figure>
            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                    <div class="flex items-center gap-2">
                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                        <div class="rating rating-sm">
                            <input type="radio" name="rating-2-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-2-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-2-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-2-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-2-1" class="mask mask-star-2 bg-accent" disabled/>
                        </div>
                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                    </div>
                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">{move || room.price_per_night.map(|p| currency_utils::format_currency(p as u64))}"/đêm"</p>
                </div>
                <h2 class="card-title text-base-content text-xl font-bold">"Phòng "{room.room_number}</h2>
                <div class="text-sm opacity-70 text-base-content space-y-1">
                    <div class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                        <span>{room.amenities}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                        <span>"Tầng " {room_floor}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                        <span>"Tối đa "{move || room.capacity}" khách"</span>
                    </div>
                </div>
            </div>
        </a>
    }
}

#[component]
fn SuggestedRooms() -> impl IntoView {
    let (suggested_rooms, set_suggested_rooms) = signal(None);
    Effect::new(move || {
        spawn_local(async move {
            match fetch2::<(), PaginatedResponse<Room>>("rooms?size=6", "GET", None).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(data) => {
                    set_suggested_rooms.set(Some(data.content));
                }
            }
        });
    });
    view! {
    <div class="p-6">
        <h2 class="text-2xl font-bold text-base-content mb-6 mt-8">"Phòng Nổi Bật"</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <Show
          when={move || {
            let suggested_rooms = suggested_rooms.read();
            suggested_rooms.is_some() && !suggested_rooms.as_ref().unwrap().is_empty()
          }}
          fallback={move ||
            if suggested_rooms.read().is_none() {
              view!{ <div class="p-2 flex items-center justify-center scale-[300%]"><Spinner /></div>}.into_any()
            } else {
              view!{ <div class="p-2 flex items-center justify-center scale-[300%] text-md">No Rooms Found.</div>}.into_any()
            }
          }
        >
          <For
            each={move || suggested_rooms.get().unwrap()}
            key={|room| room.id.as_ref().unwrap().clone()}
            let(room)
          >
            <div class="card bg-base-200 shadow-md">
                <figure><img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fw0.peakpx.com%2Fwallpaper%2F247%2F443%2FHD-wallpaper-taj-hotel-mumbai.jpg&f=1&nofb=1&ipt=bd6d720da8e8c5bde64165dc3e8e3e4d6858e564d8367720676438bb486ae478" alt="Hotel 10" class="object-cover w-full h-full"/></figure>
                <div class="card-body p-4">
                    <a href={format!("rooms/{}", room.id.as_ref().unwrap())} class="card-title text-base-content text-lg">"Phòng "{room.room_number}</a>
                    <div class="text-sm opacity-70 text-base-content space-y-1">
                        <div class="flex items-center gap-1">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            <span>{room.amenities}</span>
                        </div>
                        <div class="flex items-center gap-1">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                            <span>{room.capacity} " người"</span>
                        </div>
                    </div>
                    <div class="card-actions justify-between items-center mt-2">
                        <div class="rating rating-sm">
                            <input type="radio" name="rating-3-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-3-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-3-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-3-1" class="mask mask-star-2 bg-accent" checked disabled/>
                            <input type="radio" name="rating-3-1" class="mask mask-star-2 bg-accent" disabled/>
                        </div>
                        <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                        <p class="text-primary text-xl font-bold">{room.price_per_night}"/đêm"</p>
                    </div>
                </div>
            </div>
          </For>
        </Show>
        </div>
    </div>
    }
}
