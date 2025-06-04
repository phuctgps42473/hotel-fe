// use crate::layouts::public::Layout;
use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    let (is_drawer_open, set_is_drawer_open) = signal(false);

    view! {
    <div class="font-sans bg-gray-light">
                <header class="navbar bg-white shadow-sm px-4 md:px-8 lg:px-16 py-4">
                    <div class="flex-1">
                        <a class="text-2xl font-bold text-gray-800" href="#">"ELARIS HOTEL"</a>
                    </div>
                    <div class="flex-none hidden lg:flex">
                        <ul class="menu menu-horizontal p-0">
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#">"Trang chủ"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#">"Loại phòng"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#">"Khám phá"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#">"Thông tin"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#">"Liên hệ"</a></li>
                        </ul>
                    </div>
                    <div class="flex-none hidden lg:flex ml-4">
                        <a class="btn btn-ghost text-teal-500 hover:text-teal-700 font-semibold" href="#">"Đăng nhập"</a>
                        <a class="btn btn-info text-white bg-teal-500 hover:bg-teal-600 font-semibold" href="#">"Đăng ký"</a>
                    </div>
                    <div class="flex-none lg:hidden">
                        <label
                            // for_="my-drawer-3"
                            class="btn btn-square btn-ghost"
                            on:click=move |_| set_is_drawer_open.update(|open| *open = !*open)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                        </label>
                    </div>
                </header>

                // Drawer for mobile navigation
                <div class="drawer">
                    <input
                        id="my-drawer-3"
                        type="checkbox"
                        class="drawer-toggle"
                        checked={is_drawer_open} // Bind checked state to signal
                        on:change=move |_| set_is_drawer_open.update(|open| *open = !*open)
                    />
                    <div class="drawer-content flex flex-col">
                        // Page content (rendered by `App` directly, so nothing here for the drawer content)
                    </div>
                    <div class="drawer-side z-50"> // Added z-index to ensure drawer is on top
                        <label
                            // for_="my-drawer-3"
                            aria-label="close sidebar"
                            class="drawer-overlay"
                            on:click=move |_| set_is_drawer_open.set(false)
                        ></label>
                        <ul class="menu p-4 w-80 bg-white h-full">
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Trang chủ"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Loại phòng"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Khám phá"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Thông tin"</a></li>
                            <li><a class="font-semibold text-gray-700 hover:text-teal-500" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Liên hệ"</a></li>
                            <li class="mt-4"><a class="btn btn-ghost text-teal-500 hover:text-teal-700 font-semibold" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng nhập"</a></li>
                            <li><a class="btn btn-info text-white bg-teal-500 hover:bg-teal-600 font-semibold mt-2" href="#" on:click=move |_| set_is_drawer_open.set(false)>"Đăng ký"</a></li>
                        </ul>
                    </div>
                </div>

                // Hero Section
                <section class="relative h-[600px] bg-hero-pattern bg-cover bg-center bg-no-repeat flex items-center justify-center text-white p-4">
                    <div class="absolute inset-0 bg-black bg-opacity-40"></div>
                    <div class="relative z-10 text-center max-w-4xl mx-auto">
                        <h1 class="text-4xl md:text-6xl font-bold mb-4 leading-tight">"Du lịch và khám phá trải nghiệm trong cuộc sống"</h1>
                        <p class="text-lg md:text-xl mb-8">"khám phá văn hóa, lịch sử đến cảnh quan thiên nhiên đẹp,
                            đến tham quan các địa danh nổi tiếng."</p>
                        <button class="btn btn-info bg-teal-500 hover:bg-teal-600 text-white text-lg px-8 py-3 rounded-full font-semibold mb-12">
                            <i class="fas fa-map-marker-alt mr-2"></i>" ĐẶT PHÒNG"
                        </button>

                        // Booking Form
                        <div class="bg-white bg-opacity-90 rounded-xl p-6 md:p-8 shadow-lg flex flex-col md:flex-row items-center justify-around gap-4 mx-auto max-w-5xl">
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-calendar-alt text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="checkin" class="block text-gray-600 text-sm">"Ngày nhận phòng"</label>
                                    <select id="checkin" class="select select-bordered w-full text-gray-800 bg-transparent border-none p-0 h-auto min-h-0">
                                        <option selected>"09 mar 2023"</option>
                                        <option>"10 mar 2023"</option>
                                        <option>"11 mar 2023"</option>
                                    </select>
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-calendar-alt text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="checkout" class="block text-gray-600 text-sm">"Ngày trả phòng"</label>
                                    <select id="checkout" class="select select-bordered w-full text-gray-800 bg-transparent border-none p-0 h-auto min-h-0">
                                        <option selected>"09 mar 2023"</option>
                                        <option>"10 mar 2023"</option>
                                        <option>"11 mar 2023"</option>
                                    </select>
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-bed text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="room-type" class="block text-gray-600 text-sm">"Loại phòng"</label>
                                    <select id="room-type" class="select select-bordered w-full text-gray-800 bg-transparent border-none p-0 h-auto min-h-0">
                                        <option selected>"Standard"</option>
                                        <option>"Deluxe"</option>
                                        <option>"Suite"</option>
                                    </select>
                                </div>
                            </div>
                            <div class="flex items-center space-x-2 w-full md:w-auto">
                                <i class="fas fa-user-friends text-teal-500 text-xl"></i>
                                <div>
                                    <label for_="guests" class="block text-gray-600 text-sm">"Số người"</label>
                                    <select id="guests" class="select select-bordered w-full text-gray-800 bg-transparent border-none p-0 h-auto min-h-0">
                                        <option selected>"01"</option>
                                        <option>"02"</option>
                                        <option>"03"</option>
                                        <option>"04"</option>
                                    </select>
                                </div>
                            </div>
                            <button class="btn btn-info bg-teal-500 hover:bg-teal-600 text-white w-full md:w-auto text-lg px-8 py-3 rounded-full font-semibold md:ml-4">
                                "Tìm kiếm"
                            </button>
                        </div>
                    </div>
                </section>

                // Amenities Section
                <section class="py-16 px-4 md:px-8 lg:px-16 text-center">
                    <h2 class="text-4xl font-bold text-gray-800 mb-6">"Tiện nghi của chúng tôi"</h2>
                    <p class="text-gray-600 max-w-2xl mx-auto mb-12">"Cung cấp cơ sở vật chất hiện đại đạt tiêu chuẩn 5 sao để mang đến sự thoải mái tối đa cho bạn."</p>

                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 max-w-6xl mx-auto">
                        <div class="bg-teal-custom text-black flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-ripple"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 7c3 -2 6 -2 9 0s6 2 9 0" /><path d="M3 17c3 -2 6 -2 9 0s6 2 9 0" /><path d="M3 12c3 -2 6 -2 9 0s6 2 9 0" /></svg>
                            <span class="text-lg font-semibold">"Swimming Pool"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-wifi text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Wifi"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-coffee text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Breakfast"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-dumbbell text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Gym"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-gamepad text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Game center"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-lightbulb text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"24/7 Light"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-washer text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Laundry"</span>
                        </div>
                        <div class="bg-teal-custom text-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md aspect-square transition-transform duration-300 hover:-translate-y-1">
                            <i class="fas fa-parking text-4xl mb-3"></i>
                            <span class="text-lg font-semibold">"Parking space"</span>
                        </div>
                    </div>
                </section>

                // Room Types Section
                <section class="py-16 relative overflow-hidden">
                    <div class="absolute inset-0 bg-teal-100 opacity-20 z-0"></div>
                    <div class="relative z-10 px-4 md:px-8 lg:px-16 text-center">
                        <h2 class="text-4xl font-bold text-gray-800 mb-4">"Phòng ngủ cao cấp"</h2>
                        <p class="text-gray-600 max-w-2xl mx-auto mb-12">"Tận hưởng sự thoải mái tuyệt đối trong từng không gian phòng."</p>

                        <div class="carousel carousel-center w-full p-4 space-x-4 bg-transparent rounded-box max-w-7xl mx-auto">
                            <div class="carousel-item bg-white rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://www.privateupgrades.com/blog/wp-content/uploads/2024/02/The-Art-of-Hotel-Room-Upgrades.jpg" alt="Single Room" class="w-full h-64 object-cover"/>
                                    <span class="absolute top-4 right-4 badge badge-info bg-teal-500 text-white p-3 font-semibold">"Phòng đơn"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-gray-800 mb-2">"Phòng Đơn Tiêu Chuẩn"</h3>
                                    <p class="text-gray-600 text-sm">"Television set, Extra sheets and Breakfast"</p>
                                    <div class="card-actions justify-end mt-4">
                                        <button class="btn btn-outline btn-info border-teal-500 text-teal-500 hover:bg-teal-500 hover:text-white">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>
                            <div class="carousel-item bg-white rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://static.leonardo-hotels.com/image/leonardohotelbucharestcitycenter_room_comfortdouble2_2022_4000x2600_7e18f254bc75491965d36cc312e8111f_1200x780_mobile_3.jpeg" alt="Double Room" class="w-full h-64 object-cover"/>
                                    <span class="absolute top-4 right-4 badge badge-info bg-teal-500 text-white p-3 font-semibold">"Phòng đôi"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-gray-800 mb-2">"Phòng Đôi Sang Trọng"</h3>
                                    <p class="text-gray-600 text-sm">"Television set, Extra sheets, Breakfast, and fireplace"</p>
                                    <div class="card-actions justify-end mt-4">
                                        <button class="btn btn-outline btn-info border-teal-500 text-teal-500 hover:bg-teal-500 hover:text-white">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>
                            <div class="carousel-item bg-white rounded-lg shadow-xl overflow-hidden max-w-sm">
                                <figure class="relative">
                                    <img src="https://cf.bstatic.com/xdata/images/hotel/max1024x768/449960693.jpg?k=225914addc1d4cf76be18355b1e7f6c75b22a731914edeb67111b41a6cf56c4e&o=&hp=1" alt="Suite Room" class="w-full h-64 object-cover"/>
                                    <span class="absolute top-4 right-4 badge badge-info bg-teal-500 text-white p-3 font-semibold">"Phòng Suite"</span>
                                </figure>
                                <div class="card-body p-6 text-left">
                                    <h3 class="card-title text-xl font-semibold text-gray-800 mb-2">"Phòng Suite Hoàng Gia"</h3>
                                    <p class="text-gray-600 text-sm">"Television set, Extra sheets, Breakfast, and Fireplace, Console and bed rest"</p>
                                    <div class="card-actions justify-end mt-4">
                                        <button class="btn btn-outline btn-info border-teal-500 text-teal-500 hover:bg-teal-500 hover:text-white">"Xem chi tiết"</button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Testimonials/Philosophy Section
                <section class="py-16 px-4 md:px-8 lg:px-16 bg-white">
                    <div class="max-w-6xl mx-auto flex flex-col lg:flex-row items-center gap-8">
                        <div class="lg:w-1/2 relative w-full h-64 md:h-96 lg:h-auto overflow-hidden rounded-xl shadow-lg">
                            <img src="https://media.cnn.com/api/v1/images/stellar/prod/140127103345-peninsula-shanghai-deluxe-mock-up.jpg?q=w_2226,h_1449,x_0,y_0,c_fill" alt="Hotel Interior" class="w-full h-full object-cover rounded-xl"/>
                        </div>
                        <div class="lg:w-1/2 text-left">
                            <h2 class="text-4xl font-bold text-gray-800 mb-6">"Nơi Dừng Chân Lý Tưởng "<br/>"Cho Mọi Hành Trình"</h2>
                            <div class="relative pl-12">
                                <span class="absolute top-0 left-0 text-teal-custom text-8xl leading-none font-serif">"“"</span>
                                <p class="text-gray-700 text-lg md:text-xl leading-relaxed mb-8">
                                    "Chúng tôi tin rằng một chuyến đi hoàn hảo bắt đầu từ nơi lưu trú tuyệt vời. Mỗi căn phòng đều được chăm chút kỹ lưỡng với thiết kế hiện đại, tiện nghi đầy đủ và không gian ấm cúng. Dù bạn đến để thư giãn, khám phá thành phố hay làm việc, đội ngũ nhân viên thân thiện của chúng tôi luôn sẵn sàng phục vụ để mang lại cho bạn cảm giác thoải mái như ở nhà."
                                </p>
                                <span class="absolute bottom-0 right-0 text-teal-custom text-8xl leading-none font-serif transform rotate-180">"”"</span>
                            </div>
                            <div class="flex justify-end space-x-4 mt-8">
                                <button class="btn btn-circle btn-lg bg-gray-200 text-gray-600 hover:bg-gray-300">
                                    <i class="fas fa-chevron-left"></i>
                                </button>
                                <button class="btn btn-circle btn-lg bg-teal-500 text-white hover:bg-teal-600">
                                    <i class="fas fa-chevron-right"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                // Customer Feedback Section
                <section class="py-16 px-4 md:px-8 lg:px-16">
                    <h2 class="text-4xl font-bold text-gray-800 text-center mb-12">"CẢM NHẬN CỦA KHÁCH HÀNG"</h2>
                    <div class="max-w-6xl mx-auto flex flex-col md:flex-row items-start gap-12">
                        <div class="md:w-1/3 flex flex-col items-center justify-center space-y-4 md:space-y-0 md:space-x-4 md:flex-row md:flex-wrap md:justify-center">
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                <div class="w-24 rounded-full border-3 border-teal-custom ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 1"/>
                                </div>
                            </div>
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                <div class="w-24 rounded-full border-3 border-teal-custom ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1542596768-5d1d21f1cf98?q=80&w=1974&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 2"/>
                                </div>
                            </div>
                            <div class="avatar online mb-4 md:mb-0 md:mr-4">
                                <div class="w-24 rounded-full border-3 border-teal-custom ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?q=80&w=2944&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 3"/>
                                </div>
                            </div>
                            <div class="avatar online">
                                <div class="w-24 rounded-full border-3 border-teal-custom ring ring-offset-base-100 ring-offset-2">
                                    <img src="https://images.unsplash.com/photo-1580489944761-15a19d654956?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Customer 4"/>
                                </div>
                            </div>
                        </div>
                        <div class="md:w-2/3 text-left">
                            <div class="mb-6 relative pl-12">
                                <span class="absolute top-0 left-0 text-teal-custom text-5xl leading-none font-serif">"“"</span>
                                <p class="text-gray-700 text-lg leading-relaxed">
                                    <strong class="text-teal-500">"1. Sự thư giãn và giảm stress:"</strong>" Khi đi du lịch, bạn được trải nghiệm một môi trường mới và khác biệt so với cuộc sống thường ngày. Điều này giúp bạn thư giãn, giảm stress và tăng cường sức khỏe tinh thần."
                                </p>
                            </div>
                            <div class="mb-6 relative pl-12">
                                <span class="absolute top-0 left-0 text-teal-custom text-5xl leading-none font-serif">"“"</span>
                                <p class="text-gray-700 text-lg leading-relaxed">
                                    <strong class="text-teal-500">"2. Tận hưởng những trải nghiệm mới:"</strong>" Du lịch giúp bạn khám phá những điều mới mẻ và độc đáo, từ các món ăn địa phương, đến văn hóa và lịch sử của các địa điểm du lịch. Những trải nghiệm này giúp bạn mở rộng tầm nhìn và tăng cường kiến thức."
                                </p>
                                <span class="absolute bottom-0 right-0 text-teal-custom text-5xl leading-none font-serif transform rotate-180">"”"</span>
                            </div>
                        </div>
                    </div>
                </section>

                // Footer
                <footer class="footer p-10 bg-white text-base-content border-t border-gray-200">
                    <div class="w-full md:w-auto">
                        <h3 class="text-2xl font-bold text-gray-800 mb-4">"ELARIS HOTEL"</h3>
                        <p class="text-gray-600 mb-4">"Chào mừng quý khách đến với chương trình tour du lịch tuyệt vời của chúng tôi!"</p>
                        <div class="grid grid-flow-col gap-4 text-2xl">
                            <a href="#" class="text-gray-500 hover:text-teal-500"><i class="fab fa-youtube"></i></a>
                            <a href="#" class="text-gray-500 hover:text-teal-500"><i class="fab fa-facebook-f"></i></a>
                            <a href="#" class="text-gray-500 hover:text-teal-500"><i class="fab fa-telegram-plane"></i></a>
                            <a href="#" class="text-gray-500 hover:text-teal-500"><i class="fab fa-whatsapp"></i></a>
                        </div>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-gray-800">"Thông tin liên hệ"</span>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"99, kha vạn cân"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Tp HỒ CHÍ MINH"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"vanphuuuuuu@gmail.com"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"+0983692067"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Yêu thích"</a>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-gray-800">"Tài khoản"</span>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Tài khoản của tôi"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Đăng nhập / Đăng ký"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Xe đẩy"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Cửa hàng"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Yêu thích"</a>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-gray-800">"Liên kết nhanh"</span>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Bảo mật"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Điều khoản"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"FAQ"</a>
                        <a class="link link-hover text-gray-600 hover:text-teal-500">"Liên hệ"</a>
                    </div>
                </footer>
            </div>
                 }
}
