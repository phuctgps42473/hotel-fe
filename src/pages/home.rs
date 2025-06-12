use crate::features::home::components::category_bar::CategoryBar;
use crate::features::shared::layouts::user::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <Layout>
      <CategoryBar />
      <PlaceList />
      </Layout>
    }
}

fn get_sample_places() -> Vec<Place> {
    vec![
        Place {
            id: 1,
            image_url: "https://images.unsplash.com/photo-1580587771525-78b9dba3b914?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8dmlsbGF8ZW58MHx8MHx8fDA%3D&auto=format&fit=crop&w=500&q=60".to_string(),
            location_city: "Malibu".to_string(),
            location_country: "California".to_string(),
            title: "Oceanfront Villa with Private Pool".to_string(),
            distance_km: Some(25),
            availability: "Oct 10 - 17".to_string(),
            price_per_night: 750,
            rating: 4.92,
            is_superhost: true,
        },
        Place {
            id: 2,
            image_url: "https://images.unsplash.com/photo-1568605114967-8130f3a36994?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8aG91c2V8ZW58MHx8MHx8fDA%3D&auto=format&fit=crop&w=500&q=60".to_string(),
            location_city: "Aspen".to_string(),
            location_country: "Colorado".to_string(),
            title: "Cozy Mountain Chalet".to_string(),
            distance_km: None,
            availability: "Dec 5 - 12".to_string(),
            price_per_night: 450,
            rating: 4.85,
            is_superhost: false,
        },
        Place {
            id: 3,
            image_url: "https://images.unsplash.com/photo-1600596542815-ffad4c1539a9?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Nnx8aG91c2V8ZW58MHx8MHx8fDA%3D&auto=format&fit=crop&w=500&q=60".to_string(),
            location_city: "Kyoto".to_string(),
            location_country: "Japan".to_string(),
            title: "Traditional Machiya House".to_string(),
            distance_km: Some(5),
            availability: "Nov 1 - 7".to_string(),
            price_per_night: 300,
            rating: 4.78,
            is_superhost: true,
        },
        Place {
            id: 4,
            image_url: "https://images.unsplash.com/photo-1512917774080-9991f1c4c750?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTB8fGhvdXNlfGVufDB8fDB8fHww&auto=format&fit=crop&w=500&q=60".to_string(),
            location_city: "Santorini".to_string(),
            location_country: "Greece".to_string(),
            title: "Cave House with Caldera View".to_string(),
            distance_km: None,
            availability: "Sep 20 - 27".to_string(),
            price_per_night: 600,
            rating: 4.99,
            is_superhost: true,
        },
        // Add more places...
         Place {
            id: 5,
            image_url: "https://images.unsplash.com/photo-1600585154340-be6161a56a0c?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8OXx8aG91c2V8ZW58MHx8MHx8fDA%3D&auto=format&fit=crop&w=500&q=60".to_string(),
            location_city: "Tuscany".to_string(),
            location_country: "Italy".to_string(),
            title: "Rustic Farmhouse Retreat".to_string(),
            distance_km: Some(15),
            availability: "Aug 15 - 22".to_string(),
            price_per_night: 280,
            rating: 4.65,
            is_superhost: false,
        },
    ]
}

#[derive(Clone, Debug, PartialEq)]
pub struct Place {
    pub id: u32,
    pub image_url: String,
    pub location_city: String,
    pub location_country: String,
    pub title: String,
    pub distance_km: Option<u32>,
    pub availability: String,
    pub price_per_night: u32,
    pub rating: f32,
    pub is_superhost: bool,
}

#[component]
pub fn PlaceCard(place: Place) -> impl IntoView {
    // In a real app, this might be a signal to toggle favorite state
    let (is_favorite, _set_is_favorite) = signal(false);

    view! {
        <div class="group cursor-pointer">
            <div class="relative aspect-square w-full overflow-hidden rounded-xl bg-base-200">
                <img
                    src=place.image_url
                    alt=place.title.clone()
                    class="h-full w-full object-cover transition-transform duration-300 ease-in-out group-hover:scale-105"
                />
                <button
                    class="absolute top-3 right-3 z-10 p-1.5 rounded-full bg-black/30 hover:bg-black/50 text-white active:scale-90 transition-all"
                    on:click=move |_| {
                        leptos::logging::log!("Toggled favorite for: {}", place.id);
                    }
                    aria-label="Add to favorites"
                >
                    <Show
                        when=move || is_favorite.get()
                        fallback=|| view! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.8" stroke="currentColor" class="w-5 h-5">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
                            </svg>
                        }
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5 text-red-500">
                          <path d="M11.645 20.91l-.007-.003-.022-.012a15.247 15.247 0 01-.383-.218 25.18 25.18 0 01-4.244-3.17C4.688 15.36 2.25 12.174 2.25 8.25 2.25 5.322 4.714 3 7.688 3A5.5 5.5 0 0112 5.052 5.5 5.5 0 0116.313 3c2.973 0 5.437 2.322 5.437 5.25 0 3.925-2.438 7.111-4.739 9.256a25.175 25.175 0 01-4.244 3.17 15.247 15.247 0 01-.383.218l-.022.012-.007.004-.004.001a.752.752 0 01-.704 0l-.004-.001z" />
                        </svg>
                    </Show>
                </button>

                {if place.is_superhost {
                    view! {
                        <div class="absolute top-3 left-3 z-10 px-2 py-0.5 rounded-md bg-white text-xs font-semibold text-gray-800 shadow">
                            "SUPERHOST"
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </div>

            <div class="mt-2.5">
                <div class="flex justify-between items-start">
                    <h3 class="text-sm font-semibold text-base-content truncate pr-2">
                        {format!("{}, {}", place.location_city, place.location_country)}
                    </h3>
                    <div class="flex items-center space-x-1 text-sm text-base-content flex-shrink-0">
                        // Star Icon
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                          <path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.39-3.423 3.352c-.576.562-.26 1.559.451 1.657l4.86.446 1.944 4.595c.33.783 1.444.783 1.774 0l1.944-4.595 4.86-.446c.711-.098 1.027-1.095.45-1.657l-3.422-3.352-4.753-.39-1.831-4.401z" clip-rule="evenodd" />
                        </svg>
                        <span>{format!("{:.2}", place.rating)}</span>
                    </div>
                </div>
                <p class="text-sm text-base-content text-opacity-70 truncate">{place.title}</p>
                {if let Some(distance) = place.distance_km {
                    view! { <p class="text-sm text-base-content text-opacity-70">{format!("{} kilometers away", distance)}</p> }.into_any()
                } else {
                    ().into_any()
                }}
                <p class="text-sm text-base-content text-opacity-70">{place.availability}</p>
                <p class="mt-1">
                    <span class="font-semibold text-base-content">{format!("${}", place.price_per_night)}</span>
                    <span class="text-base-content text-opacity-90">" night"</span>
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn PlaceList() -> impl IntoView {
    let places = get_sample_places(); // Using our sample data function

    view! {
        <div class="
            container mx-auto px-4 sm:px-6 lg:px-8 py-8
            grid grid-cols-1 gap-x-6 gap-y-10
            sm:grid-cols-2
            lg:grid-cols-3
            xl:grid-cols-4
            2xl:grid-cols-5 // For very large screens
        ">
            // Using <For /> for efficient list rendering
            <For
                each=move || places.clone() // Or places() if it's a signal
                key=|place| place.id
                children=move |place| {
                    view! { <PlaceCard place=place /> }
                }
            />

            // --- Example of loading state if using create_resource ---
            // <Suspense fallback=move || view! { <p>"Loading places..."</p> }>
            //     {move || places_resource.get().map(|data| match data {
            //         Ok(loaded_places) => {
            //             if loaded_places.is_empty() {
            //                 view! { <p>"No places found."</p> }.into_view()
            //             } else {
            //                 view! {
            //                     <For
            //                         each=move || loaded_places.clone()
            //                         key=|place| place.id
            //                         view=move |place| view! { <PlaceCard place=place /> }
            //                     />
            //                 }.into_view()
            //             }
            //         }
            //         Err(e) => view! { <p class="text-error">{format!("Error loading places: {:?}", e)}</p> }.into_view(),
            //     })}
            // </Suspense>
        </div>
    }
}
