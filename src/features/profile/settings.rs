use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div>
            <h2 class="text-2xl font-bold text-base-content mb-6">Cài Đặt Tài Khoản</h2>

            <div class="space-y-6">
                // Notification Settings
                <div class="p-4 border border-base-200 rounded-[var(--radius-box)]">
                    <h3 class="font-semibold text-lg mb-3">Thông Báo</h3>
                    <div class="form-control">
                      <label class="label cursor-pointer">
                        <span class="label-text">Nhận email về các chương trình khuyến mãi</span>
                        <input type="checkbox" class="toggle toggle-primary" checked />
                      </label>
                    </div>
                     <div class="form-control">
                      <label class="label cursor-pointer">
                        <span class="label-text">Nhận thông báo cập nhật về đặt phòng</span>
                        <input type="checkbox" class="toggle toggle-primary" checked />
                      </label>
                    </div>
                </div>

                 // Language Settings
                <div class="p-4 border border-base-200 rounded-[var(--radius-box)]">
                    <h3 class="font-semibold text-lg mb-3">Ngôn Ngữ</h3>
                    <select class="select select-bordered w-full max-w-xs">
                      <option selected>Tiếng Việt</option>
                      <option>English</option>
                    </select>
                </div>

                // Account Actions
                <div class="p-4 border border-error/50 rounded-[var(--radius-box)]">
                    <h3 class="font-semibold text-lg mb-3 text-error">Hành Động</h3>
                    <div class="flex flex-col md:flex-row gap-4 items-center justify-between">
                         <p class="text-sm text-base-content/80">"Đăng xuất khỏi tài khoản hoặc xóa tài khoản vĩnh viễn."</p>
                         <div class="flex gap-3">
                             <button class="btn btn-outline">Đăng Xuất</button>
                             <button class="btn btn-error text-error-content">Xóa Tài Khoản</button>
                         </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
