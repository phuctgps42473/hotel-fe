use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
                <footer class="footer flex flex-col justify-center items-center md:flex-row md:justify-around  p-10 bg-base-100 text-base-content border-t border-gray-200">
                    <div class="w-full md:w-auto">
                        <h3 class="text-2xl font-bold text-base-content mb-4">"ELARIS HOTEL"</h3>
                        <p class="text-base-content mb-4">"Chào mừng quý khách đến với chương trình tour du lịch tuyệt vời của chúng tôi!"</p>
                        <div class="grid grid-flow-col gap-4 text-2xl">
                            <a href="#" class="text-base-content hover:text-primary"><i class="fab fa-youtube"></i></a>
                            <a href="#" class="text-base-content hover:text-primary"><i class="fab fa-facebook-f"></i></a>
                            <a href="#" class="text-base-content hover:text-primary"><i class="fab fa-telegram-plane"></i></a>
                            <a href="#" class="text-base-content hover:text-primary"><i class="fab fa-whatsapp"></i></a>
                        </div>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-base-content">"Thông tin liên hệ"</span>
                        <a class="link link-hover text-base-content hover:text-primary">127 Lê Văn Chí</a>
                        <a class="link link-hover text-base-content hover:text-primary">Tp HỒ CHÍ MINH</a>
                        <a class="link link-hover text-base-content hover:text-primary">bugsix.contact@gmail.com</a>
                        <a class="link link-hover text-base-content hover:text-primary">+84983692067</a>
                        <a class="link link-hover text-base-content hover:text-primary">Yêu thích</a>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-base-content">"Tài khoản"</span>
                        <a class="link link-hover text-base-content hover:text-primary">Tài khoản của tôi</a>
                        <a class="link link-hover text-base-content hover:text-primary">Đăng nhập/ Đăng ký</a>
                        <a class="link link-hover text-base-content hover:text-primary">Xe đẩy</a>
                        <a class="link link-hover text-base-content hover:text-primary">Cửa hàng</a>
                        <a class="link link-hover text-base-content hover:text-primary">Yêu thích</a>
                    </div>
                    <div class="w-full md:w-auto">
                        <span class="footer-title text-base-content">"Liên kết nhanh"</span>
                        <a class="link link-hover text-base-content hover:text-primary">Bảo mật</a>
                        <a class="link link-hover text-base-content hover:text-primary">Điều khoản</a>
                        <a class="link link-hover text-base-content hover:text-primary">FAQ</a>
                        <a class="link link-hover text-base-content hover:text-primary">Liên hệ</a>
                    </div>
                </footer>
    }
}
