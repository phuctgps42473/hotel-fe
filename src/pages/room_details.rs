use leptos::prelude::*;

use crate::layouts::public::{footer::Footer, header::Header};

#[component]
pub fn RoomDetails() -> impl IntoView {
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
                                Trải nghiệm một kỳ nghỉ thư giãn trong phòng Twin Guest Room sang trọng và tinh tế.
                                Phòng được trang bị hai giường đơn cùng menu gối tùy chọn để đảm bảo giấc ngủ êm ái.
                            </p>
                            <p class="text-base-content text-opacity-80 leading-relaxed">
                                Thư giãn khi xem các kênh phim cao cấp trên TV 55 inch hoặc nhâm nhi đồ uống mát lạnh từ minibar.
                                Phòng tắm riêng có bồn tắm tách biệt, sản phẩm chăm sóc cơ thể cao cấp và hai bồn rửa mặt tiện lợi.
                            </p>
                        </div>

                        // Amenities
                        <div class="mb-10">
                            <h2 class="text-2xl font-semibold text-base-content mb-4">TIỆN ÍCH ĐI KÈM</h2>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-3 gap-x-6 text-base-content text-opacity-80">
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" /></svg>
                                    <span>Phòng bếp</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h4c.057 0 .113.003.17.003h.363c.057 0 .113-.003.17-.003h4l-1-1-1-3L9.75 17z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                    <span>Tivi kèm Netflix</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14H5c-1.105 0-2-.895-2-2V9c0-1.105.895-2 2-2h5m-5 5h5m-5 0l-1-1" /></svg>
                                    <span>Máy lạnh</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M12 5l7 7-7 7" /></svg>
                                    <span>Wifi miễn phí</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5a2 2 0 00-2 2v6a2 2 0 002 2h14a2 2 0 002-2v-6a2 2 0 00-2-2z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-2a1 1 0 01-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 01-1 1z" /></svg>
                                    <span>Giặt ủi</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6L6 2l-4 4" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M22 10L18 6l-4 4" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18l4 4 4-4" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 14l-4 4-4-4" /></svg>
                                    <span>Ban công hoặc sân hiên</span>
                                </div>
                            </div>
                        </div>

                        // Reviews Section
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
                    </div>

                    // Right Column (Booking Panel)
                    <div class="lg:col-span-1 bg-base-100 p-6 rounded-[var(--radius-box)] shadow-md border border-base-200 sticky top-4">
                        <div class="text-3xl font-bold text-base-content mb-2">
                            "4,290,000₫/đêm"
                        </div>
                        <p class="text-sm text-base-content text-opacity-70 mb-6">(Đã bao gồm thuế & phí dịch vụ)</p>

                        <ul class="space-y-3 text-base-content mb-8">
                            <li>Phù hợp cho 2 người lớn</li>
                            <li>Bao gồm bữa sáng</li>
                            <li>Không hoàn hủy</li>
                            <li>Nhận phòng: từ 14:00 | Trả phòng: trước 12:00</li>
                        </ul>

                        <button class="btn btn-primary w-full text-primary-content rounded-[var(--radius-box)] text-lg py-3 mb-4">
                            ĐẶT NGAY
                        </button>
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
