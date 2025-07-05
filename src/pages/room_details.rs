use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::{hooks::use_navigate, params::Params, NavigateOptions};
use serde::Deserialize;

use crate::features::shared::components::date_range_picker::{DateRangePicker, ExcludeRange};
use crate::features::shared::components::spinner::Spinner;
use crate::layouts::public::{footer::Footer, header::Header};
use crate::libs::fetcher::{fetch, fetch2};
use crate::pages::booking::BookedDate;

#[derive(Params, PartialEq)]
pub struct RoomParam {
    id: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Room {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "roomNumber")]
    pub room_number: Option<String>,
    #[serde(rename = "roomTypeID")]
    pub room_type_id: Option<RoomType>,
    #[serde(rename = "pricePerNight")]
    pub price_per_night: Option<f64>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "capacity")]
    pub capacity: Option<i32>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "amenities")]
    pub amenities: Option<String>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct RoomType {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "typeName")]
    pub type_name: Option<String>,
    #[serde(rename = "defaultPrice")]
    pub default_price: Option<f64>,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

#[component]
pub fn RoomDetails() -> impl IntoView {
    let params = use_params::<RoomParam>();
    let id = params.read().as_ref().unwrap().id;
    let (room_details, set_room_details) = signal(Room::default());
    if id.is_none() {
        use_navigate()("/notfound", NavigateOptions::default());
    }

    let (_, set_pick_date_range) = signal(String::new());

    let (exclude_date_ranges, set_exclude_date_ranges) = signal(None);

    Effect::new(move || {
        let room_id = id.as_ref().unwrap().clone();
        spawn_local(async move {
            match fetch2::<(), Room>(&format!("rooms/{}", room_id), "GET", None).await {
                Err(e) => leptos::logging::log!("{:?}", e),
                Ok(rdetails) => {
                    set_room_details.set(rdetails);
                }
            }
        });
    });

    Effect::new(move || {
        let room_id = id.as_ref().unwrap().clone();
        spawn_local(async move {
            match fetch::<(), Vec<BookedDate>>(&format!("bookings/dates/{}", room_id), "GET", None)
                .await
            {
                Err(e) => leptos::logging::log!("{:?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        let exclude_ranges = res
                            .data
                            .unwrap()
                            .into_iter()
                            .map(
                                |booked_date| booked_date.into(), // ExcludeRange {
                                                                  //     from: date_utils::timestamp_to_html_date(booked_date.from.unwrap()),
                                                                  //     to: date_utils::timestamp_to_html_date(booked_date.to.unwrap()),
                                                                  // }
                            )
                            .collect::<Vec<ExcludeRange>>();
                        set_exclude_date_ranges.set(Some(exclude_ranges));
                    } else {
                        set_exclude_date_ranges.set(Some(vec![]));
                    }
                }
            }
        });
    });

    view! {
      <Header />
            <div class="container mx-auto bg-base-100 p-6 md:p-8 lg:p-10 rounded-[var(--radius-selector)] shadow-lg max-w-7xl">

                // Image Gallery Section
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-10">
                    // Main image
                    <div class="col-span-1 lg:col-span-1">
                        <img src="https://picsum.photos/id/20/800/600" alt="Room Main" class="w-full h-full object-cover rounded-[var(--radius-box)]"/>
                    </div>
                    // Smaller images
                    <div class="grid grid-cols-2 gap-4">
                        <img src="https://picsum.photos/id/21/400/300" alt="Room thumbnail 1" class="w-full h-full object-cover rounded-[var(--radius-box)]"/>
                        <img src="https://picsum.photos/id/22/400/300" alt="Room thumbnail 2" class="w-full h-full object-cover rounded-[var(--radius-box)]"/>
                        <img src="https://picsum.photos/id/23/400/300" alt="Room thumbnail 3" class="w-full h-full object-cover rounded-[var(--radius-box)]"/>
                        <img src="https://picsum.photos/id/24/400/300" alt="Room thumbnail 4" class="w-full h-full object-cover rounded-[var(--radius-box)]"/>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    // Left Column (Room Details)
                    <div class="lg:col-span-2">
                        // Room Title and Actions
                        <div class="flex flex-col sm:flex-row items-center justify-between mb-8">
                            <h1 class="text-3xl font-bold text-base-content mb-4 sm:mb-0">PHÒNG KHÁCH VUA</h1>
                            <div class="flex gap-4 text-base-content text-2xl">
                                <button class="btn btn-ghost btn-circle">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 22.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" /></svg>
                                </button>
                                <button class="btn btn-ghost btn-circle">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.882 12.682 9 12 9 12s12-7 12-7-3 14-3 14S8 16 8 16V9l6-3 4 5 4-4m-4 4L9 16l-4 5V14" /></svg>
                                </button>
                            </div>
                        </div>

                        // Room Facilities/Amenities Icons
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-10">
                            <div class="bg-primary text-primary-content p-4 rounded-[var(--radius-box)] flex flex-col items-center justify-center text-center shadow-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10v11l6-2 6 2 6-2V10a2 2 0 00-2-2H5a2 2 0 00-2 2zM12 2v6m0 0l-3 3m3-3l3 3" /></svg>
                                <span class="text-sm font-medium">Nhà hàng nội khu</span>
                            </div>
                            <div class="bg-primary text-primary-content p-4 rounded-[var(--radius-box)] flex flex-col items-center justify-center text-center shadow-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c1.657 0 3 1.343 3 3v4a3 3 0 01-3 3H9a3 3 0 01-3-3v-4c0-1.657 1.343-3 3-3h3z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 11V7a5 5 0 00-10 0v4m-3 0h16m-4 0v4a3 3 0 01-3 3h-2a3 3 0 01-3-3v-4" /></svg>
                                <span class="text-sm font-medium">Phòng Gym</span>
                            </div>
                            <div class="bg-primary text-primary-content p-4 rounded-[var(--radius-box)] flex flex-col items-center justify-center text-center shadow-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 13s2.34-4 5-4 5 4 5 4M10 20l2-2 2 2m-2-2V4m0 16a2 2 0 100-4 2 2 0 000 4z" /></svg>
                                <span class="text-sm font-medium">Bể bơi ngoài trời</span>
                            </div>
                            <div class="bg-primary text-primary-content p-4 rounded-[var(--radius-box)] flex flex-col items-center justify-center text-center shadow-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c1.657 0 3 1.343 3 3v4a3 3 0 01-3 3H9a3 3 0 01-3-3v-4c0-1.657 1.343-3 3-3h3z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 11V7a5 5 0 00-10 0v4m-3 0h16m-4 0v4a3 3 0 01-3 3h-2a3 3 0 01-3-3v-4" /></svg>
                                <span class="text-sm font-medium">Spa</span>
                            </div>
                        </div>

                        // Description
                        <div class="mb-10">
                            <h2 class="text-2xl font-semibold text-base-content mb-4">MÔ TẢ CHI TIẾT</h2>
                            <p class="text-base-content text-opacity-80 leading-relaxed mb-4">
                                {move || room_details.read().description.clone()}
                            </p>
                        </div>
                        <Show
                          fallback={move || view!{<Spinner />}}
                          when={move || room_details.read().amenities.is_some()}
                        >
                          <AmenityList amenities={room_details.get().amenities.unwrap().split(",").map(|a| a.to_string()).collect()} />
                        </Show>
                        <Show
                          fallback={move || view!{<Spinner />}}
                          when={move || room_details.read().amenities.is_some()}
                        >
                        <ReviewList />
                        </Show>
                    </div>

                    // Right Column (Booking Panel)
                    <div class="lg:col-span-1 bg-base-100 p-6 rounded-[var(--radius-box)] shadow-md border border-base-200 sticky top-4">
                        <div class="text-3xl font-bold text-base-content mb-2">
                            {move || room_details.read().price_per_night}"₫/đêm"
                        </div>
                        <p class="text-sm text-base-content text-opacity-70 mb-6">(Đã bao gồm thuế & phí dịch vụ)</p>

                        <ul class="space-y-3 text-base-content mb-4">
                            <li>Phù hợp cho 2 người lớn</li>
                            <li>Không hoàn hủy</li>
                            <li>Nhận phòng: từ 14:00</li>
                            <li>Trả phòng: trước 12:00</li>
                        </ul>

                        <Show
                          fallback={move || view!{}}
                          when={move || exclude_date_ranges.read().is_some()}
                        >
                            <label class="label">Xem ngày trống</label>
                            <DateRangePicker exclude_ranges={exclude_date_ranges.read().as_ref().unwrap().clone()} custom_style={String::from("w-full text-lg outline-0 py-1 rounded bg-gray-100 border-black mb-4")} id={String::from("date-range")} date_range_setter={set_pick_date_range} />
                        </Show>

                        <a href={format!("/booking/{}", id.unwrap())} class="btn btn-primary w-full text-primary-content rounded-[var(--radius-box)] text-lg py-3 mb-4">
                            ĐẶT NGAY
                        </a>
                        <a href="#" class="flex items-center justify-center gap-2 text-primary hover:underline">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" /></svg>
                            Liên hệ với khách sạn
                        </a>
                    </div>
                </div>

            </div>
            <Footer />
    }
}

#[component]
fn AmenityList(amenities: Vec<String>) -> impl IntoView {
    view! {
      <div class="mb-10">
          <h2 class="text-2xl font-semibold text-base-content mb-4">TIỆN ÍCH ĐI KÈM</h2>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-3 gap-x-6 text-base-content text-opacity-80">
            <For
              each={move || amenities.clone()}
              key={|c| c.clone()}
              children={|amenity| view! {
                <div class="flex items-center gap-3">
                    <i class="fa-solid fa-check-double"></i>
                    <span>{amenity}</span>
                </div>
              }}
            />
          </div>
      </div>
    }
}

#[component]
fn ReviewList() -> impl IntoView {
    view! {
      <div>
        <h2 class="text-2xl font-semibold text-base-content mb-4">ĐÁNH GIÁ</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            // Review Card 1
            <div class="flex gap-4">
                <div class="avatar">
                    <div class="w-12 h-12 rounded-full overflow-hidden">
                        <img src="https://i.pravatar.cc/150?img=1" alt="Avatar"/>
                    </div>
                </div>
                <div>
                    <p class="font-semibold text-base-content">Lê Lê</p>
                    <p class="text-sm text-base-content text-opacity-70 mb-2">4 - 12 - 2020</p>
                    <p class="text-base-content text-opacity-80 text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    </p>
                </div>
            </div>
            // Review Card 2
            <div class="flex gap-4">
                <div class="avatar">
                    <div class="w-12 h-12 rounded-full overflow-hidden">
                        <img src="https://i.pravatar.cc/150?img=2" alt="Avatar"/>
                    </div>
                </div>
                <div>
                    <p class="font-semibold text-base-content">Mèo Mèo</p>
                    <p class="text-sm text-base-content text-opacity-70 mb-2">6 - 12 - 2020</p>
                    <p class="text-base-content text-opacity-80 text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    </p>
                </div>
            </div>
            // Review Card 3
            <div class="flex gap-4">
                <div class="avatar">
                    <div class="w-12 h-12 rounded-full overflow-hidden">
                        <img src="https://i.pravatar.cc/150?img=3" alt="Avatar"/>
                    </div>
                </div>
                <div>
                    <p class="font-semibold text-base-content">Anh Thư</p>
                    <p class="text-sm text-base-content text-opacity-70 mb-2">4 - 12 - 2020</p>
                    <p class="text-base-content text-opacity-80 text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    </p>
                </div>
            </div>
            // Review Card 4
            <div class="flex gap-4">
                <div class="avatar">
                    <div class="w-12 h-12 rounded-full overflow-hidden">
                        <img src="https://i.pravatar.cc/150?img=4" alt="Avatar"/>
                    </div>
                </div>
                <div>
                    <p class="font-semibold text-base-content">Nhi Nhi</p>
                    <p class="text-sm text-base-content text-opacity-70 mb-2">4 - 12 - 2020</p>
                    <p class="text-base-content text-opacity-80 text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    </p>
                </div>
            </div>
        </div>
        <button class="btn btn-primary text-primary-content rounded-[var(--radius-box)] px-6 py-2">
            Xem thêm đánh giá
        </button>
    </div>
    }
}
