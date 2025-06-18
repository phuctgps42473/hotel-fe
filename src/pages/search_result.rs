use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::layouts::public::{footer::Footer, header::Header};

#[component]
pub fn SearchResult() -> impl IntoView {
    let (from_date, set_from_date) = signal(String::new());
    let (to_date, set_to_date) = signal(String::new());

    let handle_from_date_change = move |e: Event| {
        set_from_date.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        );
    };

    let handle_to_date_change = move |e: Event| {
        set_to_date.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        );
    };

    view! {
      <Header />
        <div class="min-h-screen bg-base-100 p-4 lg:p-8">
            // Vùng nội dung chính, căn giữa và có đổ bóng
            <div class="container mx-auto bg-base-100 rounded-lg shadow-lg overflow-hidden">

                // Thanh tìm kiếm ở trên cùng của vùng nội dung
                <div class="px-6 py-4 flex flex-col sm:flex-row items-center justify-between gap-4 bg-base-100 border-b border-base-300">
                    <h2 class="text-2xl font-bold text-base-content">"Bạn Hãy Tìm Phòng"</h2>
                    <div class="flex flex-col sm:flex-row items-center gap-2 w-full sm:w-auto">
                        <label class="input input-bordered flex items-center gap-2 flex-grow sm:flex-grow-0">
                            <input type="text" class="grow" placeholder="Hãy tìm phòng"/>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 opacity-70"><path fill-rule="evenodd" d="M9.965 11.023A5.479 5.479 0 1 1 11.023 9.965l3.593 3.593a.75.75 0 1 1-1.06 1.06l-3.593-3.593ZM11 6.5A4.5 4.5 0 1 1 2 6.5a4.5 4.5 0 0 1 9 0Z" clip-rule="evenodd"></path></svg>
                        </label>
                        <select class="select select-bordered w-full sm:w-auto max-w-xs">
                            <option disabled selected>Các tỉnh</option>
                            <option>"Hồ Chí Minh"</option>
                            <option>"Hà Nội"</option>
                            <option>"Đà Nẵng"</option>
                            <option>"Cần Thơ"</option>
                            <option>"Huế"</option>
                            <option>"Nha Trang"</option>
                            <option>"Phú Quốc"</option>
                            <option>"Đà Lạt"</option>
                            <option>"Hội An"</option>
                            <option>"Vũng Tàu"</option>
                        </select>
                    </div>
                </div>

                // Khu vực chính: Sidebar lọc và danh sách kết quả
                <div class="grid grid-cols-1 lg:grid-cols-4 gap-6 p-6">
                    // Sidebar lọc (Cột bên trái)
                    <div class="lg:col-span-1 space-y-6">
                        // Thời Gian Nhận – Trả Phòng
                        <div class="bg-base-200 p-4 rounded-md shadow-sm">
                            <h3 class="text-lg font-semibold mb-3 text-base-content">"Thời Gian Nhận – Trả Phòng"</h3>
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium mb-1 text-base-content">"Từ Ngày"</label>
                                    <div class="relative">
                                        <DatePicker min={from_date} max={to_date} date_change_handler={handle_from_date_change}  />
                                        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content opacity-70 pointer-events-none">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                                        </span>
                                    </div>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium mb-1 text-base-content">"Đến Ngày"</label>
                                    <div class="relative">
                                        <input type="date" min={from_date.get()} value={to_date.get()} on:change={handle_to_date_change} class="input input-bordered w-full"/>
                                        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content opacity-70 pointer-events-none">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                                        </span>
                                    </div>
                                </div>
                            </div>
                            <button class="btn btn-primary w-full mt-6">"Kiểm Tra Phòng Trống"</button>
                        </div>

                        // Chủ Đề (Themes)
                        <div class="collapse collapse-arrow bg-base-200 rounded-md shadow-sm">
                            <input type="checkbox" checked/> // checked by default
                            <div class="collapse-title text-lg font-semibold text-base-content">"Chủ Đề"</div>
                            <div class="collapse-content space-y-2 text-base-content">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Hoạt động gần sông"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Gần trung tâm"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"View thành phố"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Nghỉ dưỡng"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Khu yên tĩnh"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Cảnh thiên nhiên"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Gần địa điểm du lịch"</span>
                                </label>
                                <a href="#" class="link link-primary text-sm mt-2 block">"Xem thêm địa điểm khác"</a>
                            </div>
                        </div>

                        // Thời Gian Lưu Trú (Stay Duration)
                        <div class="collapse collapse-arrow bg-base-200 rounded-md shadow-sm">
                            <input type="checkbox" checked/> // checked by default
                            <div class="collapse-title text-lg font-semibold text-base-content">"Thời Gian Lưu Trú"</div>
                            <div class="collapse-content space-y-2 text-base-content">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"0–3 giờ (nghỉ ngắn)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"3–5 giờ (theo giờ)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"5–7 giờ"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Cả ngày (trên 7 tiếng)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Qua đêm / Nhiều ngày"</span>
                                </label>
                            </div>
                        </div>

                        // Chi Nhánh Khách Sạn (Hotel Branches)
                        <div class="collapse collapse-arrow bg-base-200 rounded-md shadow-sm">
                            <input type="checkbox" checked/> // checked by default
                            <div class="collapse-title text-lg font-semibold text-base-content">"Chi Nhánh Khách Sạn"</div>
                            <div class="collapse-content space-y-2 text-base-content">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Quận 1 (TP.HCM)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Đà Nẵng (sông Hàn)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Hà Nội (hồ Tây)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Hội An (sông Hoài)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Cần Thơ (bến Ninh Kiều)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Đà Lạt (gần hồ Xuân Hương)"</span>
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="checkbox checkbox-primary"/>
                                    <span class="label-text">"Nha Trang (gần biển)"</span>
                                </label>
                                <a href="#" class="link link-primary text-sm mt-2 block">"Hiển thị thêm chi nhánh"</a>
                            </div>
                        </div>
                    </div>

                    // Danh sách kết quả khách sạn (Cột bên phải)
                    <div class="lg:col-span-3 space-y-6">
                        // Hotel Card 1
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+1" alt="Hotel 1" class="object-cover w-full h-full"/>
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
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"800.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Quận 1, TP.HCM"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần bến Bạch Đằng"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 2
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+2" alt="Hotel 2" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-2" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"950.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Đà Nẵng (view sông Hàn)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần cầu Rồng"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 3
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+3" alt="Hotel 3" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-3" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"1.000.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Hà Nội (view hồ Tây)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần Phố Cổ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 4
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+4" alt="Hotel 4" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-4" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"850.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Cần Thơ (view bến Ninh Kiều)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần chợ nổi Cái Răng"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 5
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+5" alt="Hotel 5" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-5" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"750.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Hội An (gần sông Hoài)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần phố cổ Hội An"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 6
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+6" alt="Hotel 6" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-6" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"900.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Đà Lạt (hồ Xuân Hương)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần chợ đêm Đà Lạt"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 7
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+7" alt="Hotel 7" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-7" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"950.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Nha Trang (gần biển Trần Phú)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần quảng trường 2 Tháng 4"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 8
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+8" alt="Hotel 8" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-8" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"1.100.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Huế (sông Hương)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần Đại Nội"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Hotel Card 9
                        <div class="card card-side bg-base-200 shadow-md p-4 flex flex-col sm:flex-row">
                            <figure class="w-full sm:w-1/3 h-48 sm:h-auto overflow-hidden rounded-md flex-shrink-0">
                                <img src="https://via.placeholder.com/300x200/cccccc/ffffff?text=Hotel+Image+9" alt="Hotel 9" class="object-cover w-full h-full"/>
                            </figure>
                            <div class="card-body p-4 sm:p-6 flex flex-col justify-between w-full sm:w-2/3">
                                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2">
                                    <div class="flex items-center gap-2">
                                        <div class="badge badge-secondary text-secondary-content">"WATER ACTIVITIES"</div>
                                        <div class="rating rating-sm">
                                            <input type="radio" name="rating-2-9" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-9" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-9" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-9" class="mask mask-star-2 bg-accent" checked disabled/>
                                            <input type="radio" name="rating-2-9" class="mask mask-star-2 bg-accent" disabled/>
                                        </div>
                                        <span class="text-sm opacity-70 text-base-content">(584 reviews)</span>
                                    </div>
                                    <p class="text-primary text-lg sm:text-xl font-bold mt-2 sm:mt-0 whitespace-nowrap">"1.200.000đ/đêm"</p>
                                </div>
                                <h2 class="card-title text-base-content text-xl font-bold">"Phú Quốc (Biển Dương Đông)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần chợ đêm Phú Quốc"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <button class="btn btn-outline btn-primary w-full mt-6">"Xem Thêm"</button>
                    </div>
                </div>

                // Phần "Ưu Đãi Đặc Biệt Ngoại Thành"
                <div class="p-6">
                    <h2 class="text-2xl font-bold text-base-content mb-6 mt-8">"Ưu Đãi Đặc Biệt Ngoại Thành"</h2>

                    // Tabs/Buttons cho các danh mục ưu đãi
                    <div role="tablist" class="tabs tabs-boxed mb-6 p-1 w-fit rounded-md shadow-sm">
                      <a role="tab" class="tab tab-active btn-category">"GẦN SÔNG – VIEW ĐẸP"</a>
                      <a role="tab" class="tab btn-category">"ẨM THỰC"</a>
                    </div>

                    // Grid hiển thị các ưu đãi đặc biệt
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        // Special Offer Card 1
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+10" alt="Hotel 10" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Đà Lạt (view hồ Xuân Hương)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần chợ đêm Đà Lạt"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
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
                                    <p class="text-primary text-xl font-bold">"900.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 2
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+11" alt="Hotel 11" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Nha Trang (biển Trần Phú)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần quảng trường 2/4"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-2" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-2" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"950.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 3
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+12" alt="Hotel 12" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Huế (sông Hương)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần Đại Nội"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-3" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-3" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"850.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 4
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+13" alt="Hotel 13" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Hà Nội (view hồ Tây)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần phố cổ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-4" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-4" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"950.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 5
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+14" alt="Hotel 14" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Quận 1, TP.HCM (phố đi bộ)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần phố đi bộ Nguyễn Huệ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-5" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-5" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"800.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 6
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+15" alt="Hotel 15" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Hội An (sông Hoài)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần phố cổ Hội An"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-6" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-6" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"850.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 7
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+16" alt="Hotel 16" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Vũng Tàu (Bãi Sau)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần ngọn hải đăng"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-7" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-7" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"850.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>

                        // Special Offer Card 8
                        <div class="card bg-base-200 shadow-md">
                            <figure><img src="https://via.placeholder.com/400x250/cccccc/ffffff?text=Hotel+Image+17" alt="Hotel 17" class="object-cover w-full h-full"/></figure>
                            <div class="card-body p-4">
                                <h2 class="card-title text-base-content text-lg">"ELARIS HOTEL – Cần Thơ (bến Ninh Kiều)"</h2>
                                <div class="text-sm opacity-70 text-base-content space-y-1">
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span>"Thời lượng 2 giờ"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17l-2 2m2-2l2 2m-2-2v-3m0-12V5a2 2 0 012-2h4a2 2 0 012 2v10M9 17a2 2 0 104 0m-4 0h4"></path></svg>
                                        <span>"Gần chợ nổi Cái Răng"</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20v-2a4 4 0 00-4-4H7a4 4 0 00-4 4v2m3-2h8a2 2 0 002-2v-4a2 2 0 00-2-2H8a2 2 0 00-2 2v4a2 2 0 002 2zM12 9a2 2 0 100-4 2 2 0 000 4z"></path></svg>
                                        <span>"Gói gia đình"</span>
                                    </div>
                                </div>
                                <div class="card-actions justify-between items-center mt-2">
                                    <div class="rating rating-sm">
                                        <input type="radio" name="rating-3-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-8" class="mask mask-star-2 bg-accent" checked disabled/>
                                        <input type="radio" name="rating-3-8" class="mask mask-star-2 bg-accent" disabled/>
                                    </div>
                                    <span class="text-sm opacity-70 text-base-content">"584 reviews"</span>
                                    <p class="text-primary text-xl font-bold">"750.000đ/đêm"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <Footer />
    }
}

#[component]
pub fn DatePicker(
    min: ReadSignal<String>,
    max: ReadSignal<String>,
    date_change_handler: impl Fn(Event) + 'static,
) -> impl IntoView {
    view! {
      <input type="date" min={min} max={max} on:change={date_change_handler} class="input input-bordered w-full"/>
    }
}

pub fn get_today() -> String {
  js_sys::Date::new_0().to_iso_string().split("T").get(0).as_string().unwrap()
}