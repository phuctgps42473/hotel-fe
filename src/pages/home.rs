use crate::features::shared::components::spinner::Spinner;
use crate::layouts::public::footer::Footer;
use crate::layouts::public::header::Header;
use crate::libs::fetcher::{fetch, fetch2};
use crate::libs::utils::currency_utils::format_currency;
use crate::pages::room_details::Room;
use crate::{PaginatedResponse, RoomType};
use leptos::prelude::*;
use leptos::reactive::spawn_local;

#[component]
fn RoomCard(room: Room) -> impl IntoView {
    view! {
        <div class="card bg-white shadow-lg rounded-xl overflow-hidden transform transition-all duration-300 hover:shadow-2xl hover:-translate-y-1">
            <figure class="h-56"> // Fixed height for image container
                <img src={room.image_url} alt={room.id.unwrap()} class="w-full h-full object-cover"/>
            </figure>
            <div class="card-body p-4">
                <h2 class="card-title text-xl font-bold text-gray-800">"Phòng số "{room.room_number}</h2>
                <div class="flex items-center text-sm text-gray-500 my-1">
                    <a href={format!("/rooms/{}", room.id.unwrap())} class="hover:text-teal-custom">"Xem chi tiết phòng"</a>
                    <span class="mx-1">"•"</span>
                    <div class="flex items-center">
                        <i class="fas fa-star text-yellow-400"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                    </div>
                </div>
                <p class="text-2xl font-bold text-teal-custom my-2">{format_currency(room.price_per_night.unwrap() as u64)} VNĐ</p>
                <div class="card-actions justify-end">
                    <a href={format!("/rooms/{}", room.id.unwrap())} class="btn btn-primary bg-teal-custom hover:bg-teal-light text-white border-none rounded-lg px-6">"Xem Ngay"</a>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (room_types, set_room_types) = signal(vec![]);
    let (rooms, set_rooms) = signal(vec![]);
    let (active_tab, set_active_tab) = signal(String::new());

    let find_rooms_by_room_id = move |type_id: u64| {
        spawn_local(async move {
            match fetch::<(), PaginatedResponse<Room>>(
                &format!("rooms?roomTypeId={}", type_id),
                "GET",
                None,
            )
            .await
            {
                Err(e) => leptos::logging::error!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        set_rooms.set(res.data.unwrap().content);
                    }
                }
            }
        });
    };

    Effect::new(move || {
        spawn_local(async move {
            match fetch2::<(), Vec<RoomType>>("admin/room-type", "GET", None).await {
                Err(e) => leptos::logging::error!("{:#?}", e),
                Ok(data) => {
                    if !data.is_empty() {
                        let first = data.get(0).unwrap();
                        set_active_tab.set(first.type_name.clone());
                        find_rooms_by_room_id(first.id);
                    }
                    set_room_types.set(data);
                }
            }
        });
    });

    view! {
      <Header />
            // Room Listing Section
            <section class="container mx-auto px-4 md:px-8 py-16">
                <div class="flex flex-col md:flex-row justify-between items-center mb-10">
                    <h2 class="text-3xl md:text-4xl font-bold text-gray-800 mb-4 md:mb-0">"CÁC LOẠI PHÒNG"</h2>
                    <div class="flex items-center space-x-2">
                        // Tabs
                        <div class="tabs tabs-boxed bg-transparent p-0">

                            <Show
                                when=move || !room_types.read().is_empty()
                                fallback=|| view!{ <Spinner /> }
                            >
                                {room_types.get().into_iter()
                                    .map(|room_type: RoomType| {
                                        let tab_name = room_type.type_name;
                                        let tn = tab_name.clone();
                                        let is_active = move || active_tab.get() == tn;
                                        let tab_class = move || if is_active() {
                                            "tab tab-lg text-xl font-semibold text-teal-custom border-b-2 border-teal-custom"
                                        } else {
                                            "tab tab-lg text-xl font-semibold text-gray-500 hover:text-teal-custom"
                                        };
                                        let tn_clone = tab_name.clone();
                                        view! {
                                            <a
                                                class={tab_class}
                                                on:click=move |_| {
                                                    find_rooms_by_room_id(room_type.id);
                                                    set_active_tab.set(tn_clone.clone())
                                                }
                                            >
                                                {tab_name}
                                            </a>
                                        }
                                    })
                                    .collect_view()
                                }
                            </Show>
                        </div>
                         // Navigation Arrows (Decorative)
                        <button class="btn btn-square btn-ghost text-gray-500 hover:text-teal-custom">
                            <i class="fas fa-chevron-left text-2xl"></i>
                        </button>
                        <button class="btn btn-square bg-teal-custom hover:bg-teal-light text-white">
                            <i class="fas fa-chevron-right text-2xl"></i>
                        </button>
                    </div>
                </div>

                // Room Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <Show
                        when=move || !rooms.read().is_empty()
                        fallback= || view!{ <Spinner /> }
                    >
                        <For
                            each=move || rooms.get()
                            key=|room| room.id
                            let(child)
                        >
                        <RoomCard room={child.clone()}/>
                        </For>
                    </Show>
                </div>
            </section>

      <Footer />
    }
}
