use leptoaster::expect_toaster;
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlTextAreaElement, SubmitEvent};

use crate::libs::{
    fetcher::{fetch, fetch2},
    utils::date_utils,
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
pub fn BookingHistory(id: u64) -> impl IntoView {
    let (bookings, set_bookings) = signal(vec![]);

    Effect::new(move || {
        spawn_local(async move {
            match fetch::<(), Vec<Booking>>(&format!("profile/{}/bookings", id), "GET", None).await
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
                                    <BookingDetail id={id} booking_id={booking.id} />
                                </div>
                            </div>
                        }
                    }
                />
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
    pub transaction_no: String,
    pub amount: f64,
    #[serde(rename = "paymentDate")]
    pub payment_date: String,
    #[serde(rename = "paymentMethod")]
    pub payment_method: String,
    pub status: String,
    #[serde(rename = "processedByStaffID")]
    pub processed_by_staff_id: Option<i32>,
}

#[component]
fn BookingDetail(id: u64, booking_id: u64) -> impl IntoView {
    let (details, set_details) = signal(None);
    Effect::new(move || {
        spawn_local(async move {
            match fetch::<(), BookingDetail>(
                &format!("profile/{}/bookings/{}", id, booking_id),
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
            }
        });
    });

    view! {
      <dialog id="my_modal_2" class="modal">
        <div class="modal-box">
            <Show when=move || details.read().is_some() fallback=|| view!{}>
                {
                    // Unwrap an toàn
                    let booking = details.get().unwrap();
                    view! {
                        <h3 class="font-bold text-2xl text-primary">{ "Chi tiết" }</h3>
                        <div class="py-4 space-y-2">
                            <p><span class="font-semibold">Trạng thái:</span> <span class=format!("badge badge-{}", booking.status.clone())>{ booking.status.clone() }</span></p>
                            <p><span class="font-semibold">Nhận phòng:</span> { date_utils::timestamp_to_html_date(booking.check_in_date.clone()) }</p>
                            <p><span class="font-semibold">Trả phòng:</span> { date_utils::timestamp_to_html_date(booking.check_out_date.clone()) }</p>
                            <p><span class="font-semibold">Thanh toán:</span> <span class="text-accent-content font-bold">{ booking.total_price }" VNĐ"</span></p>
                        </div>

                        <Show
                          when=move || booking.status.eq("Checked Out")
                          fallback= {|| view! {
                            <div class="divider">"Bạn sẽ có thể đánh giá sau khi check out"</div>
                          }}
                        >
                          <Review review={booking.reviews.clone()} booking_id={booking.id} />
                        </Show>
                    }
                }
            </Show>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>
      <button onclick="my_modal_2.showModal()" class="btn btn-sm btn-outline btn-primary self-center md:self-end mt-4 md:mt-0">Xem Chi Tiết</button>
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

#[component]
fn Review(review: Vec<Review>, booking_id: u64) -> impl IntoView {
    let (review, set_review) = signal(review);
    let toaster = expect_toaster();
    let (rv, set_rv) = signal(ReviewRequest {
        booking_id,
        comment: String::new(),
        rating: 5,
    });
    let send_review = move |e: SubmitEvent| {
        e.prevent_default();
        let rv = rv.get();
        let toaster = toaster.clone();
        spawn_local(async move {
            match fetch2::<ReviewRequest, ReviewResponse>("review", "POST", Some(rv)).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(rv) => {
                    if rv.id.is_some() {
                        toaster.success("Review thành công");
                        set_review.set(vec![Review {
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
        when={move || !review.read().is_empty()}
        fallback={move || {
          let sr = send_review.clone();
         view! {
            <div class="divider">Để lại đánh giá của bạn</div>
            <form on:submit={sr} class="space-y-4">
                <div>
                    <label class="label"><span class="label-text font-semibold">Chất lượng dịch vụ</span></label>
                    <div class="rating rating-lg">
                      <input on:change={move |_| set_rv.write().rating = 1} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                      <input on:change={move |_| set_rv.write().rating = 2} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                      <input on:change={move |_| set_rv.write().rating = 3} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                      <input on:change={move |_| set_rv.write().rating = 4} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
                      <input on:change={move |_| set_rv.write().rating = 5} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" checked />
                    </div>
                </div>
                <div class="form-control">
                    <label class="label"><span class="label-text font-semibold">Bình luận</span></label>
                    <textarea on:change={move |e| set_rv.write().comment = e.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value()} class="textarea textarea-bordered h-24" placeholder="Kỳ nghỉ của bạn tuyệt vời chứ?"></textarea>
                </div>
                <div class="flex justify-end">
                    <button class="btn btn-primary">Gửi Đánh Giá</button>
                </div>
            </form>
        }}}
        >
        {
        let review = review.get().clone();
        let review = review.get(0).unwrap();
          view !{
            <div class="divider">Đánh giá của bạn</div>
            <form class="space-y-4">
                <div>
                    <label class="label"><span class="label-text font-semibold">Chất lượng dịch vụ</span></label>
                    <div class="rating rating-lg">
                    {(0.. review.rating).into_iter().map(|_| view!{<input type="radio" checked readonly name="rating-1" class="mask mask-star-2 bg-accent" />}).collect_view()}
                    </div>
                </div>
                <div class="form-control">
                    <label class="label"><span class="label-text font-semibold">Bình luận</span></label>
                    <input readonly class="input input-primary" value={review.comment.clone()} />
                </div>
            </form>
        }}
      </Show>
    }

    // if review.read().is_empty() {
    //     view! {
    //         <div class="divider">Để lại đánh giá của bạn</div>
    //         <form on:submit={send_review} class="space-y-4">
    //             <div>
    //                 <label class="label"><span class="label-text font-semibold">Chất lượng dịch vụ</span></label>
    //                 <div class="rating rating-lg">
    //                   <input on:change={move |_| set_rv.write().rating = 1} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
    //                   <input on:change={move |_| set_rv.write().rating = 2} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
    //                   <input on:change={move |_| set_rv.write().rating = 3} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
    //                   <input on:change={move |_| set_rv.write().rating = 4} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" />
    //                   <input on:change={move |_| set_rv.write().rating = 5} type="radio" name="rating-1" class="mask mask-star-2 bg-accent" checked />
    //                 </div>
    //             </div>
    //             <div class="form-control">
    //                 <label class="label"><span class="label-text font-semibold">Bình luận</span></label>
    //                 <textarea on:change={move |e| set_rv.write().comment = e.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value()} class="textarea textarea-bordered h-24" placeholder="Kỳ nghỉ của bạn tuyệt vời chứ?"></textarea>
    //             </div>
    //             <div class="flex justify-end">
    //                 <button class="btn btn-primary">Gửi Đánh Giá</button>
    //             </div>
    //         </form>
    //     }.into_any()
    // } else {
    //     let review = review.get().clone();
    //     let review = review.get(0).unwrap();
    //     view! {
    //         <div class="divider">Đánh giá của bạn</div>
    //         <form class="space-y-4">
    //             <div>
    //                 <label class="label"><span class="label-text font-semibold">Chất lượng dịch vụ</span></label>
    //                 <div class="rating rating-lg">
    //                 {(0.. review.rating).into_iter().map(|_| view!{<input type="radio" checked readonly name="rating-1" class="mask mask-star-2 bg-accent" />}).collect_view()}
    //                 </div>
    //             </div>
    //             <div class="form-control">
    //                 <label class="label"><span class="label-text font-semibold">Bình luận</span></label>
    //                 <input readonly class="input input-primary" value={review.comment.clone()} />
    //             </div>
    //         </form>
    //     }.into_any()
    // }
}
