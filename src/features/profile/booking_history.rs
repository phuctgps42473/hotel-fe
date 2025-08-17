use leptoaster::expect_toaster;
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};
use web_sys::SubmitEvent;

use crate::{
    features::shared::components::modal::Modal,
    libs::{
        fetcher::{fetch, fetch2},
        utils::{currency_utils, date_utils},
    },
};

#[derive(Deserialize, Debug, Clone)]
struct Booking {
    id: u64,
    #[serde(rename = "roomName")]
    room_name: String,
    #[serde(rename = "checkInDate")]
    checkin_date: String,
    #[serde(rename = "checkOutDate")]
    checkout_date: String,
    status: String,
}

#[component]
pub fn BookingHistory(user_id: u64) -> impl IntoView {
    let (bookings, set_bookings) = signal(vec![]);

    let (booking_id, set_booking_id) = signal(None);
    let (show_details_modal, set_show_details_modal) = signal(false);

    Effect::new(move || {
        spawn_local(async move {
            match fetch::<(), Vec<Booking>>(&format!("profile/{}/bookings", user_id), "GET", None)
                .await
            {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        set_bookings.set(res.data.unwrap());
                    }
                }
            }
        });
    });

    view! {
        <div>
            <h2 class="text-2xl font-bold text-base-content mb-6">Lịch Sử Đặt Phòng</h2>
            <div class="space-y-6">
                <For
                    each=move || bookings.get()
                    key=|booking| booking.id.to_string()
                    children=move |booking| {
                        view! {
                           <div class="flex flex-col md:flex-row items-center gap-6 p-4 border border-base-200 rounded-[var(--radius-box)] bg-base-100 shadow-sm">
                                <img src=format!("https://picsum.photos/seed/{}/200/150", booking.room_name.clone()) alt=booking.room_name.clone() class="w-full md:w-48 h-36 object-cover rounded-[var(--radius-box)]"/>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <h3 class="text-lg font-bold text-base-content">"Phòng "{booking.room_name.clone()}</h3>
                                        <div class=format!("badge badge-outline badge-{}", booking.status.clone())>{booking.status.clone()}</div>
                                    </div>
                                    <p class="text-sm text-base-content/70 mt-1">"Ngày nhận phòng: " {date_utils::timestamp_to_html_date(booking.checkin_date)}</p>
                                    <p class="text-sm text-base-content/70">"Ngày trả phòng: " {date_utils::timestamp_to_html_date(booking.checkout_date)}</p>
                                    <button
                                        on:click={move |_| {
                                            set_booking_id.set(Some(booking.id));
                                            set_show_details_modal.set(true);
                                        }}
                                        class="btn btn-sm btn-outline btn-primary self-center md:self-end mt-4 md:mt-0">Xem Chi Tiết
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
                <Show
                  when=move ||{ booking_id.read().is_some() }
                  fallback= {|| view! {}}
                >
                    <BookingDetailModal user_id={user_id} booking_id={booking_id.get().unwrap()} show_modal={show_details_modal} set_show_modal={set_show_details_modal} />
                </Show>
            </div>
        </div>
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookingDetail {
    pub id: u64,
    #[serde(rename = "checkInDate")]
    pub check_in_date: String,
    #[serde(rename = "checkOutDate")]
    pub check_out_date: String,
    #[serde(rename = "bookingDate")]
    pub booking_date: String,
    #[serde(rename = "numberOfGuests")]
    pub number_of_guests: i32,
    #[serde(rename = "totalPrice")]
    pub total_price: f64,
    pub status: String,
    #[serde(rename = "specialRequests")]
    pub special_requests: Option<String>,
    #[serde(rename = "bookingDetails")]
    pub service: Vec<BookingService>,
    pub payments: Vec<Payment>,
    pub reviews: Vec<Review>,
    #[serde(rename = "customerRequests")]
    pub customer_requests: Vec<CustomerRequest>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Review {
    pub id: u64,
    pub rating: u8,
    pub comment: String,
    #[serde(rename = "reviewDate")]
    pub review_date: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomerRequest {
    pub id: u64,
    #[serde(rename = "requestType")]
    pub request_type: String,
    pub description: String,
    #[serde(rename = "dateSubmitted")]
    pub date_submitted: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookingService {
    pub quantity: i32,
    #[serde(rename = "dateProvided")]
    pub date_provided: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Payment {
    pub id: i32,
    #[serde(rename = "bookingID")]
    pub booking_id: Option<i32>,
    #[serde(rename = "transactionNo")]
    pub transaction_no: Option<String>,
    pub amount: f64,
    #[serde(rename = "paymentDate")]
    pub payment_date: String,
    #[serde(rename = "paymentMethod")]
    pub payment_method: String,
    pub status: String,
    #[serde(rename = "processedByStaffID")]
    pub processed_by_staff_id: Option<i32>,
}

#[derive(Clone, PartialEq)]
enum DetailTab {
    InfoAndReview,
    UserRequests,
}

#[component]
pub fn BookingDetailModal(
    user_id: u64,
    booking_id: u64,
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
) -> impl IntoView {
    let (details, set_details) = signal(None::<BookingDetail>);
    let (active_tab, set_active_tab) = signal(DetailTab::InfoAndReview);

    Effect::new(move || {
        if show_modal.get() {
            set_details.set(None);
            set_active_tab.set(DetailTab::InfoAndReview);

            spawn_local(async move {
                match fetch::<(), BookingDetail>(
                    &format!("profile/{}/bookings/{}", user_id, booking_id),
                    "GET",
                    None,
                )
                .await
                {
                    Err(e) => leptos::logging::log!("{:#?}", e),
                    Ok(res) => {
                        if res.code == 200 {
                            set_details.set(res.data);
                        }
                    }
                };
            });
        }
    });

    view! {
        <Modal show_modal={show_modal} set_show_modal={set_show_modal}>
            <Show
                when=move || details.get().is_some()
                fallback=|| view! {
                    <div class="flex flex-col items-center justify-center min-h-[400px]">
                        <span class="loading loading-spinner loading-lg text-primary"></span>
                        <p class="mt-4 text-base-content/70">"Đang tải chi tiết..."</p>
                    </div>
                }
            >
                // Lấy dữ liệu an toàn
                { move || details.get().map(|booking| {
                    view! {
                        <div role="tablist" class="tabs tabs-lifted tabs-lg">
                            <button
                                role="tab"
                                class="tab"
                                class:tab-active=move || active_tab.get() == DetailTab::InfoAndReview
                                on:click=move |_| set_active_tab.set(DetailTab::InfoAndReview)
                            >"Chi tiết & Đánh giá"</button>
                            <button
                                role="tab"
                                class="tab"
                                class:tab-active=move || active_tab.get() == DetailTab::UserRequests
                                on:click=move |_| set_active_tab.set(DetailTab::UserRequests)
                            >"Yêu cầu hỗ trợ"</button>
                        </div>

                        <div class="bg-base-100 p-4 sm:p-6 rounded-b-box border-t-0 border-base-300 border -mt-px">
                            { move || match active_tab.get() {
                                DetailTab::InfoAndReview => view! { <TabInfoAndReview booking=booking.clone() /> }.into_any(),
                                DetailTab::UserRequests => view! { <TabUserRequests booking_id=booking.id customer_requests=booking.customer_requests.clone() booking_status={booking.status.clone()} /> }.into_any(),
                            }}
                        </div>
                    }
                })}
            </Show>
        </Modal>
    }
}

/// **Component Tab 1: Hiển thị thông tin và review**
#[component]
fn TabInfoAndReview(booking: BookingDetail) -> impl IntoView {
    view! {
        <h3 class="font-bold text-2xl text-primary">"Thông tin đặt phòng"</h3>
        <div class="py-4 space-y-2">
            <p><span class="font-semibold">"Trạng thái: "</span> <span class=format!("badge badge-{}", booking.status.clone())>{ booking.status.clone() }</span></p>
            <p><span class="font-semibold">"Nhận phòng: "</span> { date_utils::timestamp_to_html_date(booking.check_in_date.clone()) }</p>
            <p><span class="font-semibold">"Trả phòng: "</span> { date_utils::timestamp_to_html_date(booking.check_out_date.clone()) }</p>
            <p><span class="font-semibold">"Thanh toán: "</span> <span class="text-accent-content font-bold">{ currency_utils::format_currency(booking.total_price as u64) } " VNĐ"</span></p>
        </div>
        <Show
            when=move || booking.status == "CHECKED OUT"
            fallback=|| view! { <div class="divider">"Bạn sẽ có thể đánh giá sau khi check out"</div> }
        >
            <ReviewSection reviews=booking.reviews.clone() booking_id=booking.id />
        </Show>
    }
}

#[derive(Clone, Default, Serialize)]
struct ReviewRequest {
    #[serde(rename = "bookingId")]
    pub booking_id: u64,
    pub rating: u8,
    pub comment: String,
}

#[derive(Clone, Deserialize)]
struct ReviewResponse {
    pub id: Option<u64>,
    pub rating: Option<u8>,
    pub comment: Option<String>,
    #[serde(rename = "reviewDate")]
    pub review_date: Option<String>,
    pub message: Option<String>,
}

/// **Component con của Tab 1: Xử lý logic Review**
#[component]
fn ReviewSection(reviews: Vec<Review>, booking_id: u64) -> impl IntoView {
    let toaster = expect_toaster();
    let (review_list, set_review_list) = signal(reviews);

    let (new_review, set_new_review) = signal(ReviewRequest {
        booking_id,
        comment: String::new(),
        rating: 5,
    });

    let send_review = move |ev: SubmitEvent| {
        ev.prevent_default();
        let rv = new_review.get();

        let toaster = toaster.clone();
        spawn_local(async move {
            match fetch2::<ReviewRequest, ReviewResponse>("review", "POST", Some(rv)).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(rv) => {
                    if rv.id.is_some() {
                        toaster.success("Review thành công");
                        set_review_list.set(vec![Review {
                            id: rv.id.unwrap(),
                            comment: rv.comment.unwrap(),
                            rating: rv.rating.unwrap(),
                            review_date: rv.review_date.unwrap(),
                        }]);
                    } else {
                        toaster.error(rv.message.unwrap());
                    }
                }
            }
        });
    };

    view! {
        <Show
            when=move || review_list.get().is_empty()
            fallback=move || {
                let review = review_list.get().first().unwrap().clone();
                view! {
                    <div class="divider">"Đánh giá của bạn"</div>
                    <div class="space-y-4">
                         <div>
                            <label class="label"><span class="label-text font-semibold">"Chất lượng dịch vụ"</span></label>
                            <div class="rating rating-lg">
                                {(1..=5).map(|i| view!{
                                    <input type="radio" name="rating-view" class="mask mask-star-2 bg-accent" disabled checked={i <= review.rating} />
                                }).collect_view()}
                            </div>
                        </div>
                        <div class="form-control">
                            <label class="label"><span class="label-text font-semibold">"Bình luận"</span></label>
                            <p class="p-4 bg-base-200 rounded-box">{review.comment.clone()}</p>
                        </div>
                    </div>
                }
            }
        >
            <div class="divider">"Để lại đánh giá của bạn"</div>
            <form on:submit={send_review.clone()} class="space-y-4">
                <div>
                    <label class="label"><span class="label-text font-semibold">"Chất lượng dịch vụ"</span></label>
                    <div class="rating rating-lg">
                        <input on:click=move |_| set_new_review.update(|r| r.rating = 1) type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                        <input on:click=move |_| set_new_review.update(|r| r.rating = 2) type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                        <input on:click=move |_| set_new_review.update(|r| r.rating = 3) type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                        <input on:click=move |_| set_new_review.update(|r| r.rating = 4) type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                        <input on:click=move |_| set_new_review.update(|r| r.rating = 5) type="radio" name="rating-1" class="mask mask-star-2 bg-accent" checked />
                    </div>
                </div>
                <div class="form-control">
                    <label class="label"><span class="label-text font-semibold">"Bình luận"</span></label>
                    <textarea
                        class="textarea textarea-bordered h-24"
                        placeholder="Kỳ nghỉ của bạn tuyệt vời chứ?"
                        on:input=move |ev| set_new_review.update(|r| r.comment = event_target_value(&ev))
                    ></textarea>
                </div>
                <div class="flex justify-end">
                    <button type="submit" class="btn btn-primary">"Gửi Đánh Giá"</button>
                </div>
            </form>
        </Show>
    }
}

/// **Component Tab 2: Hiển thị và gửi Yêu cầu Hỗ trợ**
#[derive(Debug, Clone, Serialize)]
struct CustomerRequestReq {
    #[serde(rename = "bookingID")]
    id: u64,
    #[serde(rename = "requestType")]
    req_type: String,
    description: String,
    status: String,
}

#[component]
fn TabUserRequests(
    booking_id: u64,
    booking_status: String,
    customer_requests: Vec<CustomerRequest>,
) -> impl IntoView {
    let toaster = expect_toaster();
    let (requests, _) = signal(customer_requests);
    let (new_request, set_new_request) = signal(CustomerRequestReq {
        id: booking_id,
        req_type: "ROOM SERVICE".to_string(),
        description: String::new(),
        status: String::new(),
    });

    let send_request = move |ev: SubmitEvent| {
        ev.prevent_default();
        let rq = new_request.get();

        let toaster = toaster.clone();
        spawn_local(async move {
            match fetch2::<CustomerRequestReq, CustomerRequest>(
                "customer-requests",
                "POST",
                Some(rq),
            )
            .await
            {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(_) => {
                    toaster.success("Gửi yêu cầu thành công");
                    // set_requests.set(vec![review {
                    //     id: rv.id.unwrap(),
                    //     comment: rv.comment.unwrap(),
                    //     rating: rv.rating.unwrap(),
                    //     review_date: rv.review_date.unwrap(),
                    // }]);
                }
            }
        });
    };

    view! {
        <h3 class="font-bold text-2xl text-primary">"Yêu Cầu Hỗ Trợ"</h3>
        <div class="py-2 mt-4 mb-6 max-h-60 space-y-4 overflow-y-auto border rounded-box p-4 bg-base-200">
            <Show
                when=move ||{ requests.read().is_empty()}
                fallback= {|| view! {
                        <div>
                            <p class="text-sm text-base-content/80 mt-1">Không có yêu cầu nào.</p>
                        </div>
                }}
            >
                <For
                    each=move || {requests.get()}
                    key=|req| req.id
                    children=|req| view! {
                        <div>
                            <div class="flex justify-between items-center">
                                <p class="font-semibold text-base-content">{ req.request_type.clone() }</p>
                                <p class="text-xs text-base-content/60">{ date_utils::timestamp_to_html_date(req.date_submitted.clone()) }</p>
                            </div>
                            <p class="text-sm text-base-content/80 mt-1">{ req.description.clone() }</p>
                        </div>
                        <div class="divider my-2 last:hidden"></div>
                    }
                />
            </Show>
        </div>

        <Show
            when=move ||{ booking_status.eq("CHECKED_IN") }
            fallback= {|| view! {}}
        >
            <div class="divider">"Thêm Yêu Cầu Mới"</div>
            <form on:submit={send_request.clone()} class="space-y-4">
                <div class="form-control">
                    <label class="label"><span class="label-text font-semibold">"Nội dung yêu cầu"</span></label>
                    <textarea
                        class="textarea textarea-bordered h-24"
                        placeholder="Vui lòng mô tả chi tiết yêu cầu của bạn..."
                        prop:value=move || new_request.with(|r| r.description.clone())
                        on:input=move |ev| set_new_request.update(|req| req.description = event_target_value(&ev))
                    ></textarea>
                </div>
                <div class="flex justify-end">
                    <button type="submit" class="btn btn-primary">"Gửi Yêu Cầu"</button>
                </div>
            </form>
        </Show>
    }
}
