use leptos::prelude::*;

use crate::layouts::public::Layout;

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <Layout>
        <div class="bg-base-100 text-base-content font-sans">
            // Hero Section with Background Image
            <section class="relative bg-primary h-[50vh] min-h-[300px] flex items-center justify-center text-center text-primary-content">
                <div class="absolute inset-0 bg-black/40 z-10"></div>
                <img
                    src="https://picsum.photos/id/1018/1600/900" // A scenic, grand landscape image
                    alt="Lobby of Elaris Hotel"
                    class="w-full h-full object-cover"
                />
                <div class="relative z-20 p-4">
                    <h1 class="text-4xl md:text-6xl font-bold tracking-tight">Câu Chuyện Về Elaris Hotel</h1>
                    <p class="mt-4 text-lg md:text-xl max-w-2xl mx-auto">"Nơi sự sang trọng và lòng hiếu khách tạo nên những trải nghiệm khó quên."</p>
                </div>
            </section>

            <main class="container mx-auto max-w-7xl px-4 py-12 md:py-20">
                // Introduction Section
                <section class="text-center max-w-4xl mx-auto">
                    <h2 class="text-3xl md:text-4xl font-bold text-primary mb-4">Chào Mừng Đến Elaris</h2>
                    <p class="text-base-content/80 leading-relaxed">
                        "Tại Elaris Hotel, chúng tôi tin rằng mỗi chuyến đi là một cơ hội để tạo ra những kỷ niệm đẹp. Được thành lập với niềm đam mê mang đến dịch vụ hoàn hảo và không gian nghỉ dưỡng đẳng cấp, Elaris là điểm đến lý tưởng cho cả du khách doanh nhân và khách du lịch. Chúng tôi tự hào kết hợp giữa kiến trúc hiện đại, tiện nghi sang trọng và sự phục vụ tận tâm từ trái tim để mang đến cho bạn một kỳ nghỉ vượt trên cả sự mong đợi."
                    </p>
                </section>

                // Core Values / Features Section
                <section class="py-16 md:py-24">
                    <div class="text-center mb-12">
                         <h2 class="text-3xl md:text-4xl font-bold text-primary">Giá Trị Cốt Lõi Của Chúng Tôi</h2>
                         <p class="text-base-content/70 mt-2">"Những cam kết làm nên thương hiệu Elaris."</p>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                        // Value Card 1: Prime Location
                        <div class="bg-base-200/50 p-6 text-center rounded-[var(--radius-selector)] shadow-sm">
                            <div class="flex justify-center mb-4">
                                <svg class="h-12 w-12 text-secondary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M15 10.5a3 3 0 11-6 0 3 3 0 016 0z" />
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1115 0z" />
                                </svg>
                            </div>
                            <h3 class="text-xl font-semibold text-base-content mb-2">Vị Trí Đắc Địa</h3>
                            <p class="text-sm text-base-content/70">Tọa lạc tại trung tâm thành phố, dễ dàng kết nối với các điểm tham quan và trung tâm thương mại hàng đầu.</p>
                        </div>

                        // Value Card 2: World-class Service
                        <div class="bg-base-200/50 p-6 text-center rounded-[var(--radius-selector)] shadow-sm">
                             <div class="flex justify-center mb-4">
                                <svg class="h-12 w-12 text-secondary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" />
                                </svg>
                             </div>
                            <h3 class="text-xl font-semibold text-base-content mb-2">Dịch Vụ Đẳng Cấp</h3>
                            <p class="text-sm text-base-content/70">Đội ngũ nhân viên chuyên nghiệp, thân thiện và luôn sẵn sàng phục vụ 24/7 để đáp ứng mọi yêu cầu của bạn.</p>
                        </div>

                        // Value Card 3: Exquisite Cuisine
                        <div class="bg-base-200/50 p-6 text-center rounded-[var(--radius-selector)] shadow-sm">
                            <div class="flex justify-center mb-4">
                               <svg class="h-12 w-12 text-secondary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                 <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 8.25a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h18a.75.75 0 01.75.75zM21 12a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h17.25a.75.75 0 01.75.75zM21 15.75a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h17.25a.75.75 0 01.75.75z" />
                                 <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 6.75h4.5a.75.75 0 01.75.75v9a.75.75 0 01-.75.75h-4.5a.75.75 0 01-.75-.75v-9a.75.75 0 01.75-.75z" />
                               </svg>
                            </div>
                            <h3 class="text-xl font-semibold text-base-content mb-2">Ẩm Thực Tinh Hoa</h3>
                            <p class="text-sm text-base-content/70">Nhà hàng và quầy bar của chúng tôi mang đến những hành trình ẩm thực đặc sắc từ địa phương đến quốc tế.</p>
                        </div>

                        // Value Card 4: Relaxing Spaces
                        <div class="bg-base-200/50 p-6 text-center rounded-[var(--radius-selector)] shadow-sm">
                             <div class="flex justify-center mb-4">
                                <svg class="h-12 w-12 text-secondary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c1.354 0 2.664-.336 3.86-1M12 21c-1.354 0-2.664-.336-3.86-1M12 3a9.004 9.004 0 00-8.716 6.747M12 3a9.004 9.004 0 018.716 6.747M12 3c-1.354 0-2.664.336-3.86 1M12 3c1.354 0 2.664.336 3.86 1" />
                                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 12.75a3 3 0 100-6 3 3 0 000 6z" />
                                </svg>
                             </div>
                            <h3 class="text-xl font-semibold text-base-content mb-2">Không Gian Thư Giãn</h3>
                            <p class="text-sm text-base-content/70">Từ hồ bơi vô cực, spa trị liệu đến phòng gym hiện đại, mọi tiện ích đều được thiết kế cho sự thư giãn của bạn.</p>
                        </div>
                    </div>
                </section>

                // Gallery Section
                <section class="pb-16 md:pb-24">
                    <div class="text-center mb-12">
                         <h2 class="text-3xl md:text-4xl font-bold text-primary">Không Gian Tại Elaris</h2>
                         <p class="text-base-content/70 mt-2">"Một vài hình ảnh về không gian của chúng tôi."</p>
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-2 md:gap-4">
                        <div class="col-span-2 row-span-2 rounded-[var(--radius-box)] overflow-hidden">
                            <img src="https://picsum.photos/id/20/800/800" class="w-full h-full object-cover hover:scale-105 transition-transform duration-300" alt="Hotel room"/>
                        </div>
                        <div class="rounded-[var(--radius-box)] overflow-hidden">
                            <img src="https://picsum.photos/id/27/400/400" class="w-full h-full object-cover hover:scale-105 transition-transform duration-300" alt="Hotel amenity"/>
                        </div>
                        <div class="rounded-[var(--radius-box)] overflow-hidden">
                            <img src="https://picsum.photos/id/22/400/400" class="w-full h-full object-cover hover:scale-105 transition-transform duration-300" alt="Hotel bathroom"/>
                        </div>
                        <div class="rounded-[var(--radius-box)] overflow-hidden">
                             <img src="https://picsum.photos/id/160/400/400" class="w-full h-full object-cover hover:scale-105 transition-transform duration-300" alt="Hotel restaurant"/>
                        </div>
                         <div class="rounded-[var(--radius-box)] overflow-hidden">
                            <img src="https://picsum.photos/id/30/400/400" class="w-full h-full object-cover hover:scale-105 transition-transform duration-300" alt="Hotel pool"/>
                        </div>
                    </div>
                </section>

                // Call to Action
                <section class="text-center bg-base-200 rounded-[var(--radius-selector)] p-10 md:p-16">
                     <h2 class="text-3xl font-bold text-primary mb-4">Sẵn Sàng Cho Kỳ Nghỉ Trong Mơ?</h2>
                     <p class="text-base-content/80 max-w-2xl mx-auto mb-8">
                        "Hãy để chúng tôi chăm sóc bạn. Khám phá các loại phòng và ưu đãi đặc biệt chỉ có tại Elaris Hotel."
                     </p>
                     <a href="/home" class="btn btn-primary text-primary-content rounded-[var(--radius-box)] btn-lg px-10">
                        Khám Phá Phòng & Suites
                     </a>
                </section>
            </main>
        </div>
        </Layout>
    }
}
