use leptos::prelude::*;

use crate::{
    features::shared::components::calendar::Calendar,
    layouts::public::{footer::Footer, header::Header},
};

#[component]
pub fn LandingPage() -> impl IntoView {
    let (is_drawer_open, set_is_drawer_open) = signal(false);
    let (checkin, set_checkin) = signal(String::new());
    let (checkout, set_checkout) = signal(String::new());
    let (room_type, set_room_type) = signal(String::new());
    let (guest_count, set_guest_count) = signal(0);

    view! {
    <div>
    <Header />

                <div class="drawer">
                    <input
                        id="my-drawer-3"
                        type="checkbox"
                        class="drawer-toggle"
                        checked={is_drawer_open}
                        on:change=move |_| set_is_drawer_open.update(|open| *open = !*open)
                    />
                    <div class="drawer-content flex flex-col">
                    </div>
                    <div class="drawer-side z-50">
                        <label
                            // for_="my-drawer-3"
                            aria-label="close sidebar"
                            class="drawer-overlay"
                            on:click=move |_| set_is_drawer_open.set(false)
                        ></label>
                        <ul class="menu p-4 w-80 bg-base-100 h-full">
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Trang chủ"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Loại phòng"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Khám phá"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Thông tin"</a></li>
                            <li><a class="font-semibold text-base-content hover:text-primary" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Liên hệ"</a></li>
                            <li class="mt-4"><a class="btn btn-ghost text-primary hover:text-primary-focus font-semibold" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng nhập"</a></li>
                            <li><a class="btn btn-primary text-primary-content font-semibold mt-2" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng ký"</a></li>
                        </ul>
                    </div>
                </div>

                <section class="relative h-[600px] bg-hero-pattern bg-cover bg-center bg-no-repeat flex items-center justify-center text-white p-4"> // text-white giữ nguyên cho văn bản trên nền ảnh
                    <img src="/images/hero.jpg" alt="hero" class="absolute inset-0 bg-opacity-40" />
                    <div class="relative z-10 text-center max-w-4xl mx-auto">
                        <h1 class="text-4xl md:text-6xl font-bold mb-4 leading-tight">"Du lịch và khám phá trải nghiệm trong cuộc sống"</h1>
                        <p class="text-lg md:text-xl mb-8">"khám phá văn hóa, lịch sử đến cảnh quan thiên nhiên đẹp,
                            đến tham quan các địa danh nổi tiếng."</p>
                        <button class="btn btn-primary text-primary-content text-lg px-8 py-3 rounded-full font-semibold mb-12"> // btn-primary tự động dùng bg-primary và hover
                            <i class="fas fa-map-marker-alt mr-2"></i>" ĐẶT PHÒNG"
                        </button>

                        // Booking Form
                        <div class="bg-base-100 bg-opacity-90 rounded-xl p-6 md:p-8 shadow-lg flex flex-col md:flex-row items-center justify-around gap-4 mx-auto max-w-5xl">
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-calendar-alt text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="checkin" class="block text-gray-600 text-sm">"Ngày nhận phòng"</label> // Giữ text-gray-600 nếu muốn màu xám cụ thể này
                                    <Calendar date_setter={set_checkin} id={String::from("checkin")} />
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-calendar-alt text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="checkout" class="block text-gray-600 text-sm">"Ngày trả phòng"</label> // Giữ text-gray-600
                                    <Calendar date_setter={set_checkout} id={String::from("checkout")}/>
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-bed text-primary text-xl"></i>
                                <div>
                                    <label for_="room-type" class="block text-gray-600 text-sm">"Loại phòng"</label> // Giữ text-gray-600
                                    <select id="room-type" class="select select-bordered w-full text-base-content bg-transparent border-none p-0 h-auto min-h-0">
                                        <option selected>"Standard"</option>
                                        <option>"Deluxe"</option>
                                        <option>"Suite"</option>
                                    </select>
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-user-friends text-primary text-xl"></i>
                                <div>
                                    <label for_="guests" class="block text-gray-600 text-sm">"Số người"</label>
                                    <input type="number" min=1 step=1 id="guests" class="w-full text-base-content bg-transparent border-none p-0 h-auto min-h-0" />
                                </div>
                            </div>
                            <button class="btn btn-primary text-primary-content w-full md:w-auto text-lg px-8 py-3 rounded-full font-semibold md:ml-4"> // btn-primary tự động dùng bg-primary và hover
                                "Tìm kiếm"
                            </button>
                        </div>
                    </div>
                </section>

                <section class="py-16 px-4 md:px-8 lg:px-16 text-center">
                    <h2 class="text-4xl font-bold text-base-content mb-6">"Tiện nghi của chúng tôi"</h2>
                    <p class="text-base-content max-w-2xl mx-auto mb-12">"Cung cấp cơ sở vật chất hiện đại đạt tiêu chuẩn 5 sao để mang đến sự thoải mái tối đa cho bạn."</p>

                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 max-w-6xl mx-auto">
                        <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-person-swimming text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Swimming Pool"</span>
                        </div>
                        <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-wifi text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Wifi"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-coffee text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Breakfast"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-dumbbell text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Gym"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-gamepad text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Game center"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-lightbulb text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"24/7 Light"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-shirt text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Laundry"</span>
                        </div>
                         <div class="bg-secondary text-secondary-content flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-parking text-3xl mb-3"></i>
                            <span class="text-lg font-semibold">"Parking space"</span>
                        </div>
                    </div>
                </section>

                // Room Types Section
                <section class="py-16 relative overflow-hidden">
                    <div class="absolute inset-0 bg-secondary opacity-20 z-0"></div>
                    <div class="relative z-10 px-4 md:px-8 lg:px-16 text-center">
                        <h2 class="text-4xl font-bold text-base-content mb-4">"Phòng ngủ cao cấp"</h2>
                        <p class="text-base-content max-w-2xl mx-auto mb-12">"Tận hưởng sự thoải mái tuyệt đối trong từng không gian phòng."</p>

                        <div class="carousel carousel-center w-full p-4 space-x-4 bg-transparent rounded-box max-w-7xl mx-auto">
                            <div class="carousel-item bg-base-100 rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://www.privateupgrades.com/blog/wp-content/uploads/2024/02/The-Art-of-Hotel-Room-Upgrades.jpg" alt="Single Room" class="w-full h-64 object-cover"/>
                                    <span class="absolute top-4 right-4 badge badge-primary text-primary-content p-3 font-semibold">"Phòng đơn"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-base-content mb-2">"Phòng Đơn Tiêu Chuẩn"</h3>
                                    <p class="text-base-content text-sm">"Television set, Extra sheets and Breakfast"</p>
                                    <div class="card-actions justify-end mt-4">
                                        <button class="btn btn-outline btn-primary">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>

                            <div class="carousel-item bg-base-100 rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://static.leonardo-hotels.com/image/leonardohotelbucharestcitycenter_room_comfortdouble2_2022_4000x2600_7e18f254bc75491965d36cc312e8111f_1200x780_mobile_3.jpeg" alt="Double Room" class="w-full h-64 object-cover"/>
                                     // Sử dụng badge badge-primary
                                    <span class="absolute top-4 right-4 badge badge-primary text-primary-content p-3 font-semibold">"Phòng đôi"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-base-content mb-2">"Phòng Đôi Sang Trọng"</h3>
                                    <p class="text-base-content text-sm">"Television set, Extra sheets, Breakfast, and fireplace"</p>
                                    <div class="card-actions justify-end mt-4">
                                         // Sử dụng btn btn-outline btn-primary
                                        <button class="btn btn-outline btn-primary">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>
                            // Carousel Item 3
                            <div class="carousel-item bg-base-100 rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://cf.bstatic.com/xdata/images/hotel/max1024x768/449960693.jpg?k=225914addc1d4cf76be18355b1e7f6c75b22a731914edeb67111b41a6cf56c4e&o=&hp=1" alt="Suite Room" class="w-full h-64 object-cover"/>
                                     // Sử dụng badge badge-primary
                                    <span class="absolute top-4 right-4 badge badge-primary text-primary-content p-3 font-semibold">"Phòng Suite"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-base-content mb-2">"Phòng Suite Hoàng Gia"</h3>
                                    <p class="text-base-content text-sm">"Television set, Extra sheets, Breakfast, and Fireplace, Console and bed rest"</p>
                                    <div class="card-actions justify-end mt-4">
                                         // Sử dụng btn btn-outline btn-primary
                                        <button class="btn btn-outline btn-primary">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="py-16 px-4 md:px-8 lg:px-16 bg-base-100">
                    <div class="max-w-6xl mx-auto flex flex-col lg:flex-row items-center gap-8">
                        <div class="lg:w-1/2 relative w-full h-64 md:h-96 lg:h-auto overflow-hidden rounded-xl shadow-lg">
                            <img src="https://media.cnn.com/api/v1/images/stellar/prod/140127103345-peninsula-shanghai-deluxe-mock-up.jpg?q=w_2226,h_1449,x_0,y_0,c_fill" alt="Hotel Interior" class="w-full h-full object-cover rounded-xl"/>
                        </div>
                        <div class="lg:w-1/2 text-left">
                            <h2 class="text-4xl font-bold text-base-content mb-6">"Nơi Dừng Chân Lý Tưởng "<br/>"Cho Mọi Hành Trình"</h2>
                            <div class="relative pl-12">
                                <span class="absolute top-0 left-0 text-primary text-8xl leading-none font-serif">"“"</span>
                                <p class="text-base-content text-lg md:text-xl leading-relaxed mb-8">
                                    "Chúng tôi tin rằng một chuyến đi hoàn hảo bắt đầu từ nơi lưu trú tuyệt vời. Mỗi căn phòng đều được chăm chút kỹ lưỡng với thiết kế hiện đại, tiện nghi đầy đủ và không gian ấm cúng. Dù bạn đến để thư giãn, khám phá thành phố hay làm việc, đội ngũ nhân viên thân thiện của chúng tôi luôn sẵn sàng phục vụ để mang lại cho bạn cảm giác thoải mái như ở nhà."
                                </p>
                                <span class="absolute bottom-0 right-0 text-primary text-8xl leading-none font-serif transform rotate-180">"”"</span>
                            </div>
                            <div class="flex justify-end space-x-4 mt-8">
                                // btn-circle btn-lg bg-gray-200 text-gray-600 -> btn btn-circle btn-lg bg-base-200 text-base-content
                                <button class="btn btn-circle btn-lg bg-base-200 text-base-content hover:bg-base-300">
                                    <i class="fas fa-chevron-left"></i>
                                </button>
                                // btn-circle btn-lg bg-teal-500 text-white -> btn btn-circle btn-lg btn-primary
                                <button class="btn btn-circle btn-lg btn-primary text-primary-content">
                                    <i class="fas fa-chevron-right"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                // Customer Feedback Section
                <section class="py-16 px-4 md:px-8 lg:px-16">
                    <h2 class="text-4xl font-bold text-base-content text-center mb-12">"CẢM NHẬN CỦA KHÁCH HÀNG"</h2>
                    <div class="max-w-6xl mx-auto flex flex-col md:flex-row items-start gap-12">
                        <div class="md:w-1/3 flex flex-col items-center justify-center space-y-4 md:space-y-0 md:space-x-4 md:flex-row md:flex-wrap md:justify-center">
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                // Sử dụng border-primary
                                <div class="w-24 rounded-full border-3 border-primary ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 1"/>
                                </div>
                            </div>
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                // Sử dụng border-primary
                                <div class="w-24 rounded-full border-3 border-primary ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1542596768-5d1d21f1cf98?q=80&w=1974&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 2"/>
                                </div>
                            </div>
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                // Sử dụng border-primary
                                <div class="w-24 rounded-full border-3 border-primary ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 3"/>
                                </div>
                            </div>
                            <div class="avatar online">
                                // Sử dụng border-primary
                                <div class="w-24 rounded-full border-3 border-primary ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1580489944761-15a19d654956?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 4"/>
                                </div>
                            </div>
                        </div>
                        <div class="md:w-2/3 text-left">
                            <div class="mb-6 relative pl-12">
                                // Sử dụng text-primary
                                <span class="absolute top-0 left-0 text-primary text-5xl leading-none font-serif">"“"</span>
                                // Sử dụng text-base-content cho văn bản
                                <p class="text-base-content text-lg leading-relaxed">
                                    // Sử dụng text-primary cho strong
                                    <strong class="text-primary">"1. Sự thư giãn và giảm stress:"</strong>" Khi đi du lịch, bạn được trải nghiệm một môi trường mới và khác biệt so với cuộc sống thường ngày. Điều này giúp bạn thư giãn, giảm stress và tăng cường sức khỏe tinh thần."
                                </p>
                            </div>
                            <div class="mb-6 relative pl-12">
                                // Sử dụng text-primary
                                <span class="absolute top-0 left-0 text-primary text-5xl leading-none font-serif">"“"</span>
                                // Sử dụng text-base-content cho văn bản
                                <p class="text-base-content text-lg leading-relaxed">
                                    // Sử dụng text-primary cho strong
                                    <strong class="text-primary">"2. Tận hưởng những trải nghiệm mới:"</strong>" Du lịch giúp bạn khám phá những điều mới mẻ và độc đáo, từ các món ăn địa phương, đến văn hóa và lịch sử của các địa điểm du lịch. Những trải nghiệm này giúp bạn mở rộng tầm nhìn và tăng cường kiến thức."
                                </p>
                                // Sử dụng text-primary
                                <span class="absolute bottom-0 right-0 text-primary text-5xl leading-none font-serif transform rotate-180">"”"</span>
                            </div>
                        </div>
                    </div>
                </section>

                // Footer
                <Footer />
            </div>
                 }
}
