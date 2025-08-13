use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
    NavigateOptions,
};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    features::shared::components::date_range_picker::{DateRangePicker, ExcludeRange},
    layouts::public::{footer::Footer, header::Header},
    libs::{
        fetcher::fetch,
        utils::{
            currency_utils::format_currency,
            date_utils::{calculate_days_between_range, get_check_in_out},
        },
    },
};

#[derive(Clone, Debug, Deserialize)]
pub struct Service {
    #[serde(rename = "id")]
    id: Option<u64>,
    #[serde(rename = "serviceName")]
    name: Option<String>,
    #[serde(rename = "price")]
    price: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BookedDate {
    #[serde(rename = "checkInDate")]
    pub from: Option<String>,
    #[serde(rename = "checkOutDate")]
    pub to: Option<String>,
}

impl Into<ExcludeRange> for BookedDate {
    fn into(self) -> ExcludeRange {
        ExcludeRange {
            from: self.from.unwrap(),
            to: self.to.unwrap(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RoomMeta {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub price: Option<u64>,
    #[serde(rename = "bookedDates")]
    pub booked_dates: Vec<BookedDate>,
}

#[derive(Params, PartialEq)]
pub struct RoomParam {
    id: Option<u64>,
}

#[derive(Serialize, Clone)]
struct BookingRequest {
    #[serde(rename = "roomId")]
    room_id: u64,
    #[serde(rename = "customerEmail")]
    customer_email: String,
    #[serde(rename = "checkInDate")]
    checkin_date: String,
    #[serde(rename = "checkOutDate")]
    checkout_date: String,
    #[serde(rename = "numberOfGuests")]
    number_of_guest: u8,
    #[serde(rename = "servicesIds")]
    service_ids: Vec<u64>,
}

#[derive(Deserialize, Debug)]
struct BookingResponse {
    #[serde(rename = "bookingId")]
    booking_id: Option<String>,
}

#[derive(Serialize, Clone)]
struct PaymentRequest {
    #[serde(rename = "bookingId")]
    booking_id: u64,
    amount: u64,
}

#[derive(Deserialize, Debug)]
struct PaymentResponse {
    #[serde(rename = "url")]
    payment_url: Option<String>,
}

#[component]
pub fn Booking() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let user = state.user();

    Effect::new(move || {
        if user.read().is_none() {
            use_navigate()("/login", NavigateOptions::default());
        }
    });

    let params = use_params::<RoomParam>();
    let id = params.read().as_ref().unwrap().id.unwrap();
    let (room_meta, set_room_meta) = signal(RoomMeta {
        id: None,
        name: None,
        price: None,
        booked_dates: Vec::new(),
    });

    let (confirm, set_confirm) = signal(false);
    let (checked_services, set_checked_services) = signal::<Vec<u64>>(Vec::new());
    let (picked_date_range, set_picked_date_range) = signal(String::new());
    let (services, set_services) = signal(vec![]);

    Effect::new(move || {
        spawn_local(async move {
            let room_meta_response =
                fetch::<(), RoomMeta>(&format!("bookings/booked-dates/{}", id), "GET", None)
                    .await
                    .unwrap();
            if room_meta_response.code == 200 {
                set_room_meta.set(room_meta_response.data.unwrap());
            }
        });
    });

    Effect::new(move || {
        spawn_local(async move {
            let service_response = fetch::<(), Vec<Service>>("services", "GET", None)
                .await
                .unwrap();
            if service_response.code == 200 {
                set_services.set(service_response.data.unwrap());
            }
        });
    });

    let total_price = move || {
        let days = calculate_days_between_range(picked_date_range.get()) as u64;
        let mut current_total = room_meta.read().price.unwrap_or(0) * days;
        let checked = checked_services.get();
        for service in services.get().iter() {
            if checked.contains(service.id.as_ref().unwrap()) {
                current_total += service.price.unwrap();
            }
        }
        current_total
    };

    let handle_booking = move || {
        let (checkin_date, checkout_date) = get_check_in_out(picked_date_range.get()).unwrap();
        spawn_local(async move {
            match fetch::<BookingRequest, BookingResponse>(
                "bookings",
                "POST",
                Some(BookingRequest {
                    room_id: id,
                    customer_email: state.get().user.unwrap().email,
                    number_of_guest: 1,
                    checkin_date,
                    checkout_date,
                    service_ids: checked_services.get(),
                }),
            )
            .await
            {
                Err(e) => leptos::logging::log!("ERROR: {:?}", e),
                Ok(res) => {
                    if res.code == 201 {
                        let booking_id = res.data.unwrap().booking_id.unwrap();
                        match fetch::<PaymentRequest, PaymentResponse>(
                            "payment/url",
                            "POST",
                            Some(PaymentRequest {
                                booking_id: booking_id.parse::<u64>().unwrap(),
                                amount: total_price(),
                            }),
                        )
                        .await
                        {
                            Err(e) => leptos::logging::log!("ERROR: {:?}", e),
                            Ok(res) => {
                                let payment_url = res.data.unwrap().payment_url.unwrap();
                                window().location().set_href(&payment_url).unwrap();
                            }
                        }
                    } else {
                        leptos::logging::log!("ERROR: {:?}", res);
                    }
                }
            }
        });
    };

    view! {
      <Header />
        <Show
          when= move || { confirm.get()  }
          fallback= move || view! {
            <BookingCalculate
              room_meta={room_meta.read().clone()}
              set_confirm={set_confirm}
              set_checked_services = {set_checked_services}
              picked_date_range = {picked_date_range}
              set_picked_date_range = {set_picked_date_range}
              services={services}
              total_price={total_price}
            />
             }
        >
          <BookingConfirm
              room_meta={room_meta.read().clone()}
              day_range={picked_date_range.read().clone()}
              set_confirm={set_confirm}
              room_price={room_meta.get().price.unwrap()}
              checked_services={checked_services.get()}
              services={services.get()}
              handle_booking={handle_booking}
            />
        </Show>
      <Footer />
    }
}

#[component]
pub fn BookingCalculate(
    set_confirm: WriteSignal<bool>,
    set_checked_services: WriteSignal<Vec<u64>>,
    picked_date_range: ReadSignal<String>,
    set_picked_date_range: WriteSignal<String>,
    services: ReadSignal<Vec<Service>>,
    room_meta: RoomMeta,
    total_price: impl Fn() -> u64 + 'static + Send,
) -> impl IntoView {
    let exclude_date_range = room_meta
        .booked_dates
        .iter()
        .map(|d| d.clone().into())
        .collect();

    view! {
        <div class="font-sans min-h-screen flex flex-col items-center bg-gray-light text-gray-800">
            // Logo (Top center)
            <div class="w-full py-8 md:py-12 text-center">
                <a class="text-3xl md:text-4xl font-bold text-gray-800" href="#">"ELARIS HOTEL"</a>
            </div>

            // Stepper
            <div class="flex items-center justify-center space-x-4 mb-12">
                // Step 1: Completed
                <div class="relative flex items-center">
                    <div class="bg-teal-custom p-3 rounded-full text-white text-xl flex items-center justify-center aspect-square">
                        <i class="fas fa-check"></i>
                    </div>
                </div>
                // Step 2: Active
                <div class="relative flex items-center">
                    <div class="w-12 h-1 bg-gray-300"></div>
                    <div class="bg-teal-light p-3 rounded-full text-white text-xl font-semibold flex items-center justify-center aspect-square">
                        "2"
                    </div>
                </div>
                <div class="relative flex items-center">
                    <div class="w-12 h-1 bg-gray-300"></div>
                    <div class="bg-gray-300 p-3 rounded-full text-gray-600 text-xl font-semibold flex items-center justify-center aspect-square">
                        "3"
                    </div>
                </div>
            </div>

            // Main Content Area
            <main class="flex-grow flex flex-col items-center justify-center p-4 text-center max-w-5xl w-full mx-auto">
                <h1 class="text-3xl md:text-4xl font-bold text-teal-custom mb-2">"THÔNG TIN ĐẶT PHÒNG"</h1>
                <p class="text-gray-600 text-lg mb-10">"Vui lòng điền vào các ô trống bên dưới"</p>

                <div class="flex flex-col lg:flex-row bg-white rounded-xl shadow-lg overflow-hidden w-full max-w-4xl">
                    // Left Section: Room Image and Name
                    <div class="w-full lg:w-2/5 relative">
                        <img
                            src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.mos.cms.futurecdn.net%2FeSmxudoM7Gxc33JQdPhPs.jpg&f=1&nofb=1&ipt=955dcd364f49aefc7a7e9f9268d8ff65dedd8b0a944e0771b5a9762b39c03293"
                            alt="Phòng Khách Vua"
                            class="w-full h-64 lg:h-full object-cover"
                        />
                        <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/60 to-transparent p-4 text-white">
                            <h2 class="text-2xl font-bold">"PHÒNG KHÁCH VUA"</h2>
                        </div>
                    </div>

                    // Right Section: Booking Details Form
                    <div class="w-full lg:w-3/5 p-6 md:p-8 text-left">

                        <div class="mb-6">
                            <label class="block text-gray-700 text-lg font-semibold mb-2">"Chọn ngày"</label>
                            <div class="flex items-center space-x-3 bg-gray-100 p-4 rounded-lg cursor-pointer hover:bg-gray-200 transition-colors">
                                <i class="fas fa-calendar-alt text-teal-custom text-2xl"></i>
                                <DateRangePicker exclude_ranges={exclude_date_range} custom_style={String::from("w-full text-lg outline-0 bg-gray-100 border-none")} id={String::from("date-range")} date_range_setter={set_picked_date_range} />
                            </div>
                        </div>

                        // Included services checkboxes
                        <div class="mb-8">
                            <label class="block text-gray-700 text-lg font-semibold mb-3">"Dịch vụ đi kèm"</label>
                            <div class="space-y-3">
                              <For
                                each=move || services.get()
                                key=|state| state.id.as_ref().unwrap().clone()
                                let(child)
                              >
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-info [--chkbg:theme(colors.teal-custom)] [--chkfg:white]"
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            if !checked {
                                              set_checked_services.update(|svs| {
                                                svs.remove(svs.iter().position(|sv| sv.eq(child.id.as_ref().unwrap())).unwrap());
                                              });
                                            } else {
                                              set_checked_services.update(|svs| {
                                                svs.push(child.id.unwrap());
                                              });
                                            }
                                        }
                                    />
                                    <label class="ml-3 text-lg text-gray-700 select-none">
                                        {child.name.as_ref().unwrap().to_string()}" ("{format_currency(child.price.unwrap())}" VNĐ)"
                                    </label>
                                </div>
                              </For>
                            </div>
                        </div>

                        // Total Price
                        <div class="text-right mb-10">
                            <p class="text-gray-600 text-lg">"Bạn sẽ trả" <span class="text-teal-custom text-3xl font-bold">{move || format_currency(total_price())} VNĐ</span></p>
                            <p class="text-gray-600 text-xl font-semibold">"cho " <span class="text-teal-custom">{move || calculate_days_between_range(picked_date_range.get())}" ngày"</span></p>
                        </div>
                    </div>
                </div>

                // Action Buttons
                <div class="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4 mt-10 w-full max-w-sm">
                    <button on:click={move |_| set_confirm.set(true)} class="btn btn-info bg-teal-custom hover:bg-teal-light text-white font-semibold py-3 px-8 rounded-md w-full sm:w-1/2">
                        Xác Nhận
                    </button>
                    <button class="btn btn-ghost bg-gray-200 hover:bg-gray-300 text-gray-600 font-semibold py-3 px-8 rounded-md w-full sm:w-1/2">
                    Huỷ Bỏ
                    </button>
                </div>
            </main>
        </div>
    }
}

#[component]
pub fn BookingConfirm(
    day_range: String,
    set_confirm: WriteSignal<bool>,
    room_price: u64,
    room_meta: RoomMeta,
    checked_services: Vec<u64>,
    services: Vec<Service>,
    handle_booking: impl Fn() + 'static + Send,
) -> impl IntoView {
    let (services, _set_services) = signal(services);
    let (checkin, checkout) = get_check_in_out(day_range.clone()).unwrap();
    let total_days = calculate_days_between_range(day_range);

    // Derived signal for total services price
    let cs = checked_services.clone();
    let total_services_price = move || {
        services
            .get()
            .iter()
            .filter(|s| cs.contains(s.id.as_ref().unwrap())) // Filter for selected services
            .map(|s| *s.price.as_ref().unwrap()) // Get their price
            .sum::<u64>() // Sum them up
    };

    // Derived signal for total price
    let total_price = move || (room_price * total_days as u64) + total_services_price();

    view! {
        <div class="font-sans min-h-screen flex flex-col items-center bg-gray-light text-gray-800">
            <main class="flex-grow flex flex-col items-center justify-center p-4 text-center w-full">
                <div class="bg-gray-lighter rounded-xl shadow-lg p-6 md:p-8 w-full max-w-4xl">
                    <h1 class="text-3xl md:text-4xl font-bold text-teal-custom mb-8">"XÁC NHẬN ĐẶT PHÒNG"</h1>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 text-left">
                        <div>
                            <h2 class="text-2xl font-bold text-gray-800 mb-4">"Chi tiết phòng"</h2>
                            <div class="bg-white rounded-lg shadow-md overflow-hidden mb-6">
                                <img
                                    src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.mos.cms.futurecdn.net%2FeSmxudoM7Gxc33JQdPhPs.jpg&f=1&nofb=1&ipt=955dcd364f49aefc7a7e9f9268d8ff65dedd8b0a944e0771b5a9762b39c03293"
                                    alt="Phòng Khách Vua"
                                    class="w-full h-48 object-cover"
                                />
                                <div class="p-4">
                                    <h3 class="text-xl font-semibold text-gray-800">Phòng {room_meta.name}</h3>
                                    // <p class="text-gray-600 text-sm">{room_meta}</p>
                                </div>
                            </div>
                            <div class="space-y-3">
                                <p class="text-lg text-gray-700"><i class="fas fa-moon mr-2 text-teal-custom"></i>"Số đêm: " <span class="font-semibold">{total_days}</span></p>
                                <p class="text-lg text-gray-700"><i class="fas fa-calendar-alt mr-2 text-teal-custom"></i>"Ngày nhận phòng: " <span class="font-semibold">{checkin}</span></p>
                                <p class="text-lg text-gray-700"><i class="fas fa-calendar-check mr-2 text-teal-custom"></i>"Ngày trả phòng: " <span class="font-semibold">{checkout}</span></p>
                                // <p class="text-lg text-gray-700"><i class="fas fa-users mr-2 text-teal-custom"></i>"Số lượng khách: " <span class="font-semibold">"2"</span></p>
                            </div>
                        </div>

                        // Right Column: Services & Pricing Summary
                        <div>
                            <h2 class="text-2xl font-bold text-gray-800 mb-4">"Dịch vụ đi kèm"</h2>
                            <div class="space-y-3 mb-6">
                                <For
                                    each=move || services.get()
                                    key=|s| s.id.as_ref().unwrap().to_string()
                                    children=move |s| {
                                        let cs = checked_services.clone();
                                        view! {
                                            <Show when={move || cs.contains(s.id.as_ref().unwrap())}>
                                                <div class="flex justify-between items-center text-lg text-gray-700">
                                                    <span>{s.clone().name.unwrap()}</span>
                                                    <span class="font-semibold">{format_currency(s.price.unwrap())} VNĐ</span>
                                                </div>
                                            </Show>
                                        }
                                    }
                                />
                                <div class="flex justify-between items-center text-lg text-gray-700 font-semibold border-b border-gray-200 pb-3">
                                    <span>"Tổng tiền phòng"</span>
                                    <span>{format_currency(room_price * total_days as u64)} VNĐ</span>
                                </div>
                            </div>

                            <div class="flex justify-between items-center text-3xl font-bold text-teal-custom">
                                <span>"Tổng tiền phải trả"</span>
                                <span>{format_currency(total_price())} VNĐ</span>
                            </div>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4 mt-10 w-full max-w-md mx-auto">
                        <button on:click={move |_| handle_booking()} class="btn btn-info bg-teal-custom hover:bg-teal-light text-white font-semibold py-3 px-8 rounded-md w-full sm:w-1/2">
                            "Xác nhận và Thanh toán"
                        </button>
                        <button
                            on:click={move |_| set_confirm.set(false)}
                            class="btn btn-ghost bg-gray-200 hover:bg-gray-300 text-gray-600 font-semibold py-3 px-8 rounded-md w-full sm:w-1/2 flex items-center justify-center"
                        >
                            "Chỉnh sửa"
                        </button>
                    </div>
                </div>
            </main>
        </div>
    }
}
