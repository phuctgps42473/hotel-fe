use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
      <header class="navbar bg-base-100 shadow-sm px-4 md:px-12 py-3 flex justify-between items-center">
        <div class="text-xl font-bold text-cyan-700">Bubu.vn</div>
        <nav class="hidden md:flex gap-6">
          <a href="#" class="hover:text-cyan-600">Home</a>
          <a href="#" class="hover:text-cyan-600">Du lịch</a>
          <a href="#" class="hover:text-cyan-600">Vận chuyển</a>
          <a href="#" class="hover:text-cyan-600">Tin tức</a>
          <a href="#" class="hover:text-cyan-600 text-blue-600">Liên hệ</a>
        </nav>
        <div class="flex gap-2">
          <button class="btn btn-ghost">Đăng nhập</button>
          <button class="btn btn-primary btn-sm">Đăng ký</button>
        </div>
      </header>

      <section class="max-w-6xl mx-auto mt-6 p-4">
        <div class="rounded-xl overflow-hidden">
          <img src="https://i.imgur.com/fVE3JJl.jpg" alt="Bubu Office" class="w-full object-cover h-60 md:h-[400px]" />
        </div>
        <div class="mt-4">
          <h1 class="text-xl font-bold">
            Công Ty Bubu.vn là công ty chuyên về du lịch và Tuor tạo ra những chuyến đi thật thú vị
          </h1>
          <div class="flex items-center gap-2 mt-1 text-yellow-500 text-sm">
            "⭐⭐⭐⭐⭐ (75)"
          </div>
          <div class="text-sm text-gray-500 mt-1">Tọa lạc tại đường 923, quận Thủ Đức thành phố HCM</div>
          <div class="flex gap-2 mt-2 flex-wrap">
            <button class="btn btn-outline btn-sm">Xem</button>
            <div class="badge badge-outline">"📍 1.6km"</div>
          </div>
          <div class="mt-4 flex gap-4 flex-wrap">
            <button class="btn btn-primary">"🎯 ĐI NGAY ĐẾN VỊ TRÍ"</button>
            <button class="btn btn-outline">"📌 Vị trí bản đồ"</button>
          </div>
        </div>
      </section>

      <section class="max-w-6xl mx-auto p-4 mt-4">
        <h2 class="font-bold text-lg mb-2">Chi tiết về công ty</h2>
        <p class="text-gray-700 text-sm leading-relaxed">
          Công ty "Bubu.vn" là một startup công nghệ Tour có trụ sở tại Hồ Chí Minh. Công ty chuyên phát triển các chuyến du lịch cho mọi người.
          <br /><br />
          Bubu.vn được thành lập bởi một nhóm các chuyên gia công nghệ, các nhà khoa học và các nhà phát triển phần mềm có kinh nghiệm. Các chuyến du lịch cho mọi người để giúp giải quyết các vấn đề khó khăn của thế giới hiện đại như bạn muốn tìm lại cảm hứng cho cá nhân.
        </p>
      </section>

      <section class="max-w-6xl mx-auto p-4 mt-6">
        <h2 class="text-xl font-bold mb-4">CÁC ĐIỂM DU LỊCH</h2>
        <div class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
          <div class="card bg-base-100 shadow">
            <figure><img src="https://i.imgur.com/6Hb6ZXG.jpg" alt="Đà Nẵng" /></figure>
            <div class="card-body">
              <h3 class="font-semibold">Đập Lâm AN</h3>
              <p class="text-gray-500 text-sm">6,190,000đ / Tour</p>
              <button class="btn btn-sm btn-primary">Đặt ngay</button>
            </div>
          </div>

          <div class="card bg-base-100 shadow">
            <figure><img src="https://i.imgur.com/nZR6YBZ.jpg" alt="Hội An" /></figure>
            <div class="card-body">
              <h3 class="font-semibold">Phố Cổ Hội An</h3>
              <p class="text-gray-500 text-sm">1,190,000đ / Tour</p>
              <button class="btn btn-sm btn-primary">Đặt ngay</button>
            </div>
          </div>

          <div class="card bg-base-100 shadow">
            <figure><img src="https://i.imgur.com/kj7BlJi.jpg" alt="Huế" /></figure>
            <div class="card-body">
              <h3 class="font-semibold">Cung Đình Huế</h3>
              <p class="text-gray-500 text-sm">2,290,000đ / Tour</p>
              <button class="btn btn-sm btn-primary">Đặt ngay</button>
            </div>
          </div>

        </div>
      </section>

      <footer class="bg-base-200 p-6 mt-10 text-sm">
        <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-6">
          <div>
            <h3 class="font-bold text-cyan-600 text-lg">Bubu.vn</h3>
            <p class="text-gray-600 mt-2">
              Chào mừng quý khách đến với chương trình tour du lịch tuyệt vời của chúng tôi!
            </p>
            <div class="flex gap-3 mt-3">
              <a href="#"><i class="text-xl text-blue-500">"🌐"</i></a>
              <a href="#"><i class="text-xl text-blue-400">"📘"</i></a>
              <a href="#"><i class="text-xl text-pink-500">"📷"</i></a>
            </div>
          </div>
          <div>
            <h4 class="font-semibold mb-2">Thông tin liên hệ</h4>
            <p>99, khu vực cân<br />Tp HỒ CHÍ MINH</p>
            <p>vanphuuuu@gmail.com</p>
            <p>+0983692067</p>
          </div>
          <div>
            <h4 class="font-semibold mb-2">Tài khoản</h4>
            <ul class="space-y-1">
              <li><a href="#">Tài khoản của tôi</a></li>
              <li><a href="#">Đăng nhập / Đăng ký</a></li>
              <li><a href="#">Xe đẩy</a></li>
              <li><a href="#">Cửa hàng</a></li>
            </ul>
          </div>
          <div>
            <h4 class="font-semibold mb-2">Liên kết nhanh</h4>
            <ul class="space-y-1">
              <li><a href="#">Bảo mật</a></li>
              <li><a href="#">Điều khoản</a></li>
              <li><a href="#">Về chúng tôi</a></li>
              <li><a href="#">Liên hệ</a></li>
            </ul>
          </div>
        </div>
      </footer>
    }
}
