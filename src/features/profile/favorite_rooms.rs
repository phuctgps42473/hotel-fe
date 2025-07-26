use leptos::prelude::*;

#[component]
pub fn Favorites() -> impl IntoView {
    let favorites = vec![
        ("Phòng Khách Vua", "Từ 4.290.000 VNĐ/đêm"),
        ("Suite Tổng Thống", "Từ 25.000.000 VNĐ/đêm"),
    ];

    view! {
        <div>
            <h2 class="text-2xl font-bold text-base-content mb-6">Danh Sách Phòng Yêu Thích</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                 <For
                    each=move || favorites.clone()
                    key=|fav| fav.0.to_string()
                    children=|(name, price)| {
                         view! {
                           <div class="bg-base-100 border border-base-200 rounded-[var(--radius-box)] shadow-sm overflow-hidden">
                               <img src=format!("https://picsum.photos/seed/{}/400/250", name) alt=name class="w-full h-48 object-cover"/>
                               <div class="p-4">
                                   <div class="flex justify-between items-start">
                                       <div>
                                           <h3 class="text-lg font-bold text-base-content">{name}</h3>
                                           <p class="text-primary font-semibold">{price}</p>
                                       </div>
                                       <button class="btn btn-ghost btn-circle text-error">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
                                       </button>
                                   </div>
                                    <button class="btn btn-primary w-full mt-4">Đặt Ngay</button>
                               </div>
                           </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

