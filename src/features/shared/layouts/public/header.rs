use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
      <header class="px-6 py-4 flex justify-between items-center shadow-md sticky top-0 z-50 bg-white">
        <div class="text-2xl font-bold text-teal-600">Bug Six</div>
        <nav class="space-x-6 text-sm">
          <a href="#" class="text-teal-600 font-semibold">Trang chủ</a>
          <a href="#">Khám phá</a>
          <a href="#">Loại phòng</a>
          <a href="#">Thông tin</a>
          <a href="#">Liên hệ</a>
        </nav>
        <div class="space-x-2">
          <button class="btn btn-ghost btn-sm">Đăng nhập</button>
          <button class="btn btn-primary btn-sm">Đăng ký</button>
        </div>
      </header>
    }
}
