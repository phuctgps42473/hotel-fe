use leptos::prelude::*;

#[component]
pub fn CategoryBar() -> impl IntoView {
    let categories = vec![
        "Beachfront",
        "Cabins",
        "OMG!",
        "National Parks",
        "Islands",
        "Tiny Homes",
        "Design",
        "Arctic",
        "Amazing Pools",
        "Lakefront",
        "Surfing",
        "A-frames",
        "Castles",
        "Campers",
        "New",
        "Treehouses",
        "Tropical",
        "Shared Homes",
        "Luxe",
        "Skiing",
    ];

    let (selected_category, set_selected_category) = signal(categories[0].to_string());

    view! {
        <div class="sticky top-14 md:top-20 z-10 bg-base-100 border-b border-base-200 shadow-sm">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8 pt-3 pb-6 flex items-center space-x-5 sm:space-x-6 overflow-x-auto scrollbar-hide">
                <For
                    each=move || categories.clone()
                    key=|category| category.to_string()
                    children=move |category: &'static str| {
                        let is_selected = Memo::new(move |_| selected_category.get() == category);

                        view! {
                            <button
                                class="btn flex flex-col items-center justify-start flex-shrink-0 pt-2 pb-1 cursor-pointer group transition-all duration-200 ease-in-out border-b-2"
                                class:border-current = move || is_selected.get()
                                class:border-transparent = move || !is_selected.get()
                                class:hover:border-base-300 = move || !is_selected.get()

                                class:text-base-content = move || is_selected.get()
                                // class:text-base-content = move || !is_selected()

                                class:opacity-100 = move || is_selected.get()
                                class:opacity-70 = move || !is_selected.get()
                                class:hover:opacity-100 = true

                                // class:font-semibold = move || is_selected()

                                on:click=move |_| {
                                    set_selected_category.set(category.to_string());
                                    leptos::logging::log!("Selected: {}", category);
                                }
                            >
                                <div class="w-6 h-6 mb-1 flex items-center justify-center text-xl text-neutral-400">
                                     "★"
                                </div>

                                <span class="text-xs font-medium whitespace-nowrap">
                                    {category}
                                </span>
                            </button>
                        }
                    }
                />

                <button class="btn btn-outline btn-sm ml-auto flex-shrink-0">
                   <span>Filters</span>
                </button>
            </div>
        </div>
    }
}
