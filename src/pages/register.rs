use leptos::prelude::*;

#[component]
pub fn Register() -> impl IntoView {
    view! {
     <div class="font-sans min-h-screen flex flex-col bg-water-texture bg-cover bg-center bg-fixed">
            // Main Registration Content
            <main class="flex-grow flex items-center justify-center p-4 md:p-8">
                <div class="relative w-full max-w-4xl mx-auto bg-gray-lighter rounded-3xl shadow-xl overflow-hidden
                            flex flex-col lg:flex-row min-h-[500px]"> // Responsive layout, rounded corners

                    // Left Image Section
                    <div class="relative w-full lg:w-1/2 bg-login-image bg-cover bg-center rounded-t-3xl lg:rounded-l-3xl lg:rounded-tr-none min-h-[250px] lg:min-h-full">
                        <div class="absolute inset-0 bg-gradient-to-t from-teal-custom via-teal-custom/50 to-transparent opacity-70"></div> // Gradient overlay
                        <div class="absolute inset-0 flex items-end justify-start p-8 text-white z-10">
                            <h2 class="text-4xl font-bold">"ELARIS HOTEL"</h2>
                        </div>
                    </div>

                    // Right Form Section
                    <div class="w-full lg:w-1/2 p-8 md:p-12 flex flex-col justify-center">
                        <h1 class="text-4xl font-bold text-gray-800 text-center mb-8">"ĐĂNG KÝ"</h1>

                        <form class="space-y-6">
                            <div>
                                <label for_="full-name" class="sr-only">"Họ và tên"</label>
                                <input type="text" id="full-name" placeholder="Họ và tên"
                                       class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-xl placeholder-gray-500"/>
                            </div>
                            <div>
                                <label for_="email" class="sr-only">"Email"</label>
                                <input type="email" id="email" placeholder="Email"
                                       class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-xl placeholder-gray-500"/>
                            </div>
                            <div>
                                <label for_="phone" class="sr-only">"Số điện thoại"</label>
                                <input type="tel" id="phone" placeholder="Số điện thoại"
                                       class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-xl placeholder-gray-500"/>
                            </div>
                            <div>
                                <label for_="password" class="sr-only">"Mật khẩu"</label>
                                <input type="password" id="password" placeholder="Mật khẩu"
                                       class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-xl placeholder-gray-500"/>
                            </div>
                            <div>
                                <label for_="confirm-password" class="sr-only">"Xác nhận mật khẩu"</label>
                                <input type="password" id="confirm-password" placeholder="Xác nhận mật khẩu"
                                       class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-xl placeholder-gray-500"/>
                            </div>

                            <button type="submit" class="btn btn-info w-full bg-teal-custom hover:bg-teal-light text-white text-lg font-semibold py-3 rounded-md mt-6">
                                "Đăng ký"
                            </button>
                        </form>

                        <div class="divider text-gray-400 text-sm my-6">"Hoặc tiếp tục với"</div>

                        <button class="btn btn-outline w-full border-gray-300 hover:border-teal-custom hover:bg-teal-50 hover:text-gray-800 text-gray-600 font-semibold py-3 flex items-center justify-center rounded-md">
                            <img src="https://www.google.com/images/branding/googleg/1x/googleg_standard_color_18dp.png" alt="Google icon" class="w-5 h-5 mr-3"/>
                            "Đăng ký với Google"
                        </button>
                    </div>
                </div>
            </main>
        </div>
    }
}
