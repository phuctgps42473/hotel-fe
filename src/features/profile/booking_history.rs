use leptos::prelude::*;

#[component]
pub fn BookingHistory() -> impl IntoView {
    // Dữ liệu mẫu
    let bookings = vec![
        (
            "Phòng Khách Vua",
            "20/12/2023",
            "22/12/2023",
            "8.480.000 VNĐ",
            "Sắp tới",
            "success",
        ),
        (
            "Suite Hướng Biển",
            "15/11/2023",
            "18/11/2023",
            "12.000.000 VNĐ",
            "Đã hoàn thành",
            "neutral",
        ),
        (
            "Phòng Deluxe",
            "01/10/2023",
            "02/10/2023",
            "3.500.000 VNĐ",
            "Đã hủy",
            "error",
        ),
    ];

    view! {
        <div>
            <h2 class="text-2xl font-bold text-base-content mb-6">Lịch Sử Đặt Phòng</h2>
            <div class="space-y-6">
                <For
                    each=move || bookings.clone()
                    key=|booking| booking.0.to_string()
                    children=|(name, check_in, check_out, price, status, badge_color)| {
                        view! {
                           <div class="flex flex-col md:flex-row items-center gap-6 p-4 border border-base-200 rounded-[var(--radius-box)] bg-base-100 shadow-sm">
                                <img src=format!("https://picsum.photos/seed/{}/200/150", name) alt=name class="w-full md:w-48 h-36 object-cover rounded-[var(--radius-box)]"/>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <h3 class="text-lg font-bold text-base-content">{name}</h3>
                                        <div class=format!("badge badge-outline badge-{}", badge_color)>{status}</div>
                                    </div>
                                    <p class="text-sm text-base-content/70 mt-1">"Ngày nhận phòng: " {check_in}</p>
                                    <p class="text-sm text-base-content/70">"Ngày trả phòng: " {check_out}</p>
                                    <p class="text-lg font-semibold text-primary mt-2">{price}</p>
                                </div>
                                <button class="btn btn-sm btn-outline btn-primary self-center md:self-end mt-4 md:mt-0">Xem Chi Tiết</button>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
