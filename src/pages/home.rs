use crate::layouts::public::footer::Footer;
use crate::layouts::public::header::Header;
use leptos::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Room {
    id: u32,
    name: String,
    category: String,
    image_url: String,
    price_per_night: u64,
    rating: f32,
    reviews: u32,
}

// Helper to format currency
fn format_currency(amount: u64) -> String {
    format!("{}.000đ/đêm", amount / 1000)
}

#[component]
fn RoomCard(room: Room) -> impl IntoView {
    view! {
        <div class="card bg-white shadow-lg rounded-xl overflow-hidden transform transition-all duration-300 hover:shadow-2xl hover:-translate-y-1">
            <figure class="h-56"> // Fixed height for image container
                <img src={room.image_url} alt={room.name.clone()} class="w-full h-full object-cover"/>
            </figure>
            <div class="card-body p-4">
                <h2 class="card-title text-xl font-bold text-gray-800">{room.name}</h2>
                <div class="flex items-center text-sm text-gray-500 my-1">
                    <a href="/rooms/1" class="hover:text-teal-custom">"Xem chi tiết phòng"</a>
                    <span class="mx-1">"•"</span>
                    <div class="flex items-center">
                        <i class="fas fa-star text-yellow-400"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                        <i class="fas fa-star text-yellow-400 ml-0.5"></i>
                        <i class="fas fa-star text-gray-300 ml-0.5"></i>
                        <i class="fas fa-star text-gray-300 ml-0.5"></i>
                        <span class="ml-1">{format!("({} reviews)", room.reviews)}</span>
                    </div>
                </div>
                <p class="text-2xl font-bold text-teal-custom my-2">{format_currency(room.price_per_night)}</p>
                <div class="card-actions justify-end">
                    <button class="btn btn-primary bg-teal-custom hover:bg-teal-light text-white border-none rounded-lg px-6">"Xem Ngay"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let all_rooms_data = vec![
        Room { id: 1, name: "Phòng khách vua".to_string(), category: "Phòng khách".to_string(), image_url: "https://images.unsplash.com/photo-1560448204-e02f11c3d0e2?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 2, name: "Phòng khách đôi".to_string(), category: "Phòng khách".to_string(), image_url: "https://images.unsplash.com/photo-1598454238505-fab00095ab7c?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 3, name: "Phòng khách hướng thành phố".to_string(), category: "Phòng khách".to_string(), image_url: "https://images.unsplash.com/photo-1582719508461-905c673771fd?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 4, name: "Phòng đôi hướng thành phố".to_string(), category: "Phòng khách".to_string(), image_url: "https://images.unsplash.com/photo-1611892440504-42a792e24d32?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 5, name: "Phòng đôi hướng sông".to_string(), category: "Phòng khách".to_string(), image_url: "https://images.unsplash.com/photo-1522771739844-6a9f6d5f14af?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 6, name: "Phòng đôi Executive".to_string(), category: "Phòng Suite".to_string(), image_url: "https://images.unsplash.com/photo-1631049307264-da0ec9d70304?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 7, name: "Phòng vua Deluxe hướng sông".to_string(), category: "Phòng Suite".to_string(), image_url: "https://images.unsplash.com/photo-1596394516093-501ba68a0ba6?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 8, name: "Phòng vua Premium hướng sông".to_string(), category: "Phòng Suite".to_string(), image_url: "https://images.unsplash.com/photo-1618773928121-c32242e63f39?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
        Room { id: 9, name: "Phòng gia đình hướng sông".to_string(), category: "Phòng Suite".to_string(), image_url: "https://images.unsplash.com/photo-1578683010236-d716f9a3f461?q=80&w=2940&auto=format&fit=crop".to_string(), price_per_night: 4290000, rating: 3.0, reviews: 75 },
    ];

    let (active_tab, set_active_tab) = signal("Phòng khách".to_string());
    let tabs = vec!["Tất cả", "Phòng khách", "Phòng Suite"];

    let filtered_rooms = Memo::new(move |_| {
        let current_tab = active_tab.get();
        if current_tab == "Tất cả" {
            all_rooms_data.clone()
        } else {
            all_rooms_data
                .iter()
                .filter(|r| r.category == current_tab)
                .cloned()
                .collect::<Vec<_>>()
        }
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
                             {tabs.into_iter()
                                .map(|tab_name_str| {
                                    let tab_name = tab_name_str.to_string();
                                    let tn = tab_name.clone();
                                    let is_active = move || active_tab.get() == tn;
                                    let tab_class = move || if is_active() {
                                        "tab tab-lg text-xl font-semibold text-teal-custom border-b-2 border-teal-custom"
                                    } else {
                                        "tab tab-lg text-xl font-semibold text-gray-500 hover:text-teal-custom"
                                    };
                                    let tn_clone = tab_name.clone();
                                    view! {
                                        <a class={tab_class} on:click=move |_| set_active_tab.set(tn_clone.clone())>{tab_name}</a>
                                    }
                                })
                                .collect_view()
                             }
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
                    <For
                        each=move || filtered_rooms.get()
                        key=|room| room.id
                        let(child)
                    >
                      <RoomCard room={child.clone()}/>
                    </For>
                </div>
            </section>

      <Footer />
    }
}
