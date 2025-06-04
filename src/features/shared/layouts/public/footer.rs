use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer class="px-6 py-10 bg-white border-t">
        <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-6 text-sm">
          <div>
            <div class="font-bold text-teal-600 text-xl mb-2">Bubu.vn</div>
            <p>Chào mừng quý khách đến với chương trình tour du lịch tuyệt vời</p>
            <div class="mt-2 flex gap-2">
              <a href="#"><img src="/icons/facebook.svg" alt="fb" class="w-5" /></a>
              <a href="#"><img src="/icons/twitter.svg" alt="tw" class="w-5" /></a>
              <a href="#"><img src="/icons/instagram.svg" alt="ig" class="w-5" /></a>
            </div>
          </div>
          <div>
            <h4 class="font-bold mb-2">Thông tin liên hệ</h4>
            <p>90, K***, TP HCM</p>
            <p>vanphu*****@mail.com</p>
            <p>+0985020697</p>
          </div>
          <div>
            <h4 class="font-bold mb-2">Tài khoản</h4>
            <ul>
              <li>Đăng nhập / Đăng ký</li>
              <li>Lịch sử đặt</li>
              <li>Yêu thích</li>
            </ul>
          </div>
          <div>
            <h4 class="font-bold mb-2">Liên kết nhanh</h4>
            <ul>
              <li>Khám phá</li>
              <li>Loại phòng</li>
              <li>Liên hệ</li>
            </ul>
          </div>
        </div>
      </footer>
    }
}
