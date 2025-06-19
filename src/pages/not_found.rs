use leptos::prelude::*;

use crate::layouts::public::{footer::Footer, header::Header};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
      <Header />
            <div class="font-sans min-h-screen flex items-center justify-center bg-water-texture bg-cover bg-center bg-fixed p-4">
                <div class="bg-gray-lighter rounded-3xl shadow-xl p-8 md:p-12 text-center max-w-lg w-full mx-auto animate-fade-in">
                    // Optional image for visual appeal
                    <img
                        src="/images/not-found.png"
                        alt="Lost boat on water"
                        class="w-32 h-auto mx-auto mb-6 rounded-lg"
                    />

                    <h1 class="text-6xl md:text-8xl font-bold text-teal-custom mb-4">"404"</h1>
                    <h2 class="text-2xl md:text-3xl font-semibold text-gray-800 mb-4">"Trang không tìm thấy"</h2>
                    <p class="text-gray-600 text-lg mb-8 leading-relaxed">
                        "Rất tiếc, trang bạn đang tìm kiếm không tồn tại."<br/>
                        "Có thể địa chỉ đã bị sai, hoặc trang đã bị di chuyển."
                    </p>

                    <a
                        href="/"
                        class="btn btn-info bg-teal-custom hover:bg-teal-light text-white flex items-center justify-center
                               font-semibold py-3 px-8 rounded-md transition duration-300 ease-in-out transform hover:-translate-y-1"
                    >
                        <i class="fas fa-home mr-2"></i>" Quay về Trang chủ"
                    </a>
                </div>
            </div>
    <Footer />
    }
}
