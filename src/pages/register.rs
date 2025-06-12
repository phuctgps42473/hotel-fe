use std::time::Duration;

use js_sys::RegExp;
use leptoaster::*;
use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, SubmitEvent};

use crate::{
    config::reqwest::fetcher,
    layouts::public::{footer::Footer, header::Header},
    ApiResponse,
};

#[derive(Serialize)]
struct RegisterForm {
    #[serde(rename = "fullname")]
    fullname: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
    #[serde(rename = "password")]
    password: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    message: String,
}

#[component]
pub fn Register() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    let toaster = expect_toaster();
    let (disable_button, set_disable_button) = signal(true);
    let (fullname, set_fullname) = signal(String::new());
    let (fullname_error, set_fullname_error) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (email_error, set_email_error) = signal(String::new());
    let (phone_number, set_phone_number) = signal(String::new());
    let (phone_error, set_phone_error) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (password_error, set_password_error) = signal(String::new());

    let handle_fullname_change = move |e: Event| {
        set_fullname.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        )
    };
    let validate_fullname = move || {
        let fullname = fullname.get();
        if fullname.trim().is_empty() {
            set_fullname_error.set("Full name length required".to_string());
        } else if fullname.trim().len() < 2 {
            set_fullname_error.set("Full name length > 2".to_string());
        } else if fullname.len() > 100 {
            set_fullname_error.set("Full name length < 100".to_string());
        } else {
            set_fullname_error.set("".to_string());
            set_disable_button.set(
                email_error.read().is_empty()
                    && phone_error.read().is_empty()
                    && password_error.read().is_empty(),
            );
            return;
        }
        set_disable_button.set(true);
    };

    let handle_email_change = move |e: Event| {
        set_email.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        )
    };
    let validate_email = move || {
        let tester = RegExp::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", "");
        let email = email.read();
        if email.trim().is_empty() {
            set_email_error.set("Email is required".to_string());
        } else if !tester.test(email.as_borrowed()) {
            set_email_error.set("Invalid email account".to_string());
        } else {
            set_email_error.set("".to_string());
            set_disable_button.set(
                fullname_error.read().is_empty()
                    && phone_error.read().is_empty()
                    && password_error.read().is_empty(),
            );
            return;
        }
        set_disable_button.set(true);
    };

    let handle_phone_change = move |e: Event| {
        set_phone_number.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        )
    };
    let validate_phone = move || {
        let pattern = RegExp::new("^(0[3|5|7|8|9])+([0-9]{8})$", "");

        let phone = phone_number.read();
        if phone.trim().is_empty() {
            set_phone_error.set("Phone number is required".to_string());
        } else if !pattern.test(phone.as_borrowed()) {
            set_phone_error.set("Invalid phone number".to_string());
        } else {
            set_phone_error.set("".to_string());
            set_disable_button.set(
                fullname_error.read().is_empty()
                    && email_error.read().is_empty()
                    && password_error.read().is_empty(),
            );
            return;
        }
        set_disable_button.set(true);
    };

    let handle_password_change = move |e: Event| {
        set_password.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        )
    };
    let handle_confirm_password_change = move |e: Event| {
        set_confirm_password.set(
            e.target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        )
    };
    let validate_password = move || {
        let password = password.read();
        let confirm_password = confirm_password.read();
        if password.is_empty() {
            set_password_error.set("Password is required".to_string());
        }
        if password.len() < 6 {
            set_password_error.set("Password is too short".to_string());
        } else if password == password.to_lowercase() {
            set_password_error.set("Password should have uppcase letters and number".to_string());
        } else if !password.chars().any(|c| c.is_ascii_digit()) {
            set_password_error.set("Password should have uppcase letters and number".to_string());
        } else if !password.eq(&confirm_password) {
            set_password_error.set("Password does not match".to_string());
        } else {
            set_password_error.set("".to_string());
            set_disable_button.set(
                fullname.read().is_empty()
                    && phone_error.read().is_empty()
                    && email_error.read().is_empty(),
            );
            return;
        }
        set_disable_button.set(true);
    };

    let handle_submit = move |e: SubmitEvent| {
        let navigate = navigate.clone();
        e.prevent_default();

        let fullname = fullname.get();
        let email = email.get();
        let phone_number = phone_number.get();
        let password = password.get();

        let toaster = toaster.clone();
        spawn_local(async move {
            match fetcher()
                .post("http://localhost:8080/api/register")
                .json(&RegisterForm {
                    fullname,
                    email,
                    phone_number,
                    password,
                })
                .send()
                .await
            {
                Ok(res) => {
                    let body = res.json::<ApiResponse<RegisterResponse>>().await.unwrap();
                    match body.code {
                        201 => {
                            toaster.success("Successfully registered.Redirecting to login page");
                            set_timeout(
                                move || {
                                    navigate("/login", Default::default());
                                },
                                Duration::from_millis(3000),
                            );
                        }
                        400 => toaster.error(body.error.unwrap().message),
                        500 => toaster.error("Something wrong with the system"),
                        _ => leptos::logging::log!("Unexpected Response Status"),
                    }
                }
                Err(error) => {
                    leptos::logging::log!("{:?}", error)
                }
            }
        });

        // leptos::logging::log!("{}-{}-{}-{}", fullname, email, password, phone_number);
    };

    view! {
      <Header />
     <div class="relative font-sans min-h-screen flex flex-col bg-[url(/images/register-bg.jpg)] bg-cover bg-center bg-fixed">
     <div class="absolute inset-0 bg-black opacity-50 backdrop-blur-lg" />
            // Main Registration Content
            <main class="flex-grow flex items-center justify-center p-4 md:p-8">
                <div class="relative w-full max-w-4xl mx-auto bg-white rounded-3xl shadow-xl overflow-hidden
                            flex flex-col lg:flex-row min-h-[500px]">

                    // Left Image Section
                    <div class="relative w-full lg:w-1/2 bg-[url(/images/register-form.jpg)] bg-cover bg-center rounded-t-3xl lg:rounded-l-3xl lg:rounded-tr-none min-h-[250px] lg:min-h-full">
                        <div class="absolute inset-0 bg-black opacity-35 backdrop-blur-lg" />
                        <div class="absolute inset-0 bg-gradient-to-t from-teal-custom via-teal-custom/50 to-transparent opacity-70"></div> // Gradient overlay
                        <div class="absolute inset-0 flex items-end justify-start p-8 text-white z-10">
                            <h2 class="text-4xl font-bold">"ELARIS HOTEL"</h2>
                        </div>
                    </div>

                    // Right Form Section
                    <div class="w-full lg:w-1/2 p-8 md:p-12 flex flex-col justify-center">
                        <h1 class="text-4xl font-bold text-gray-800 text-center mb-8">"ĐĂNG KÝ"</h1>

                        <form on:submit={handle_submit} class="space-y-6">
                            <Input
                              id={"full_name".to_string()}
                              input_type={"text".to_string()}
                              label_name={"Họ và tên".to_string()}
                              min_len={Some(3)}
                              max_len={Some(100)}
                              value={fullname}
                              error={fullname_error}
                              handle_onchange={handle_fullname_change}
                              handle_onblur={validate_fullname}
                              pattern={None}
                            />
                            <Input
                              id={"email".to_string()}
                              input_type={"email".to_string()}
                              label_name={"Email".to_string()}
                              min_len={Some(6)}
                              max_len={Some(100)}
                              value={email}
                              error={email_error}
                              handle_onchange={handle_email_change}
                              handle_onblur={validate_email}
                              pattern={None}
                            />
                            <Input
                              id={"phone_number".to_string()}
                              input_type={"tel".to_string()}
                              label_name={"Phone Number".to_string()}
                              min_len={Some(10)}
                              max_len={Some(12)}
                              value={phone_number}
                              error={phone_error}
                              handle_onchange={handle_phone_change}
                              handle_onblur={validate_phone}
                              pattern={None}
                            />
                            <Input
                              id={"password".to_string()}
                              input_type={"password".to_string()}
                              label_name={"Password".to_string()}
                              min_len={Some(6)}
                              max_len={Some(50)}
                              value={password}
                              error={password_error}
                              handle_onchange={handle_password_change}
                              handle_onblur={validate_password}
                              pattern={None}
                            />
                            <Input
                              id={"comfirm-password".to_string()}
                              input_type={"password".to_string()}
                              label_name={"Confirm Password".to_string()}
                              min_len={Some(6)}
                              max_len={Some(50)}
                              value={confirm_password}
                              error={password_error}
                              handle_onchange={handle_confirm_password_change}
                              handle_onblur={validate_password}
                              pattern={None}
                            />

                            <button type="submit" disabled={disable_button} class="btn btn-info w-full bg-teal-custom hover:bg-teal-light text-white text-lg font-semibold py-3 rounded-md mt-6">
                                "Đăng ký"
                            </button>
                        </form>

                        <div class="divider text-gray-400 text-sm my-6">"Hoặc tiếp tục với"</div>

                        <button class="btn btn-outline w-full border-gray-300 hover:border-teal-custom hover:bg-teal-50 hover:text-gray-800 text-gray-600 font-semibold py-3 flex items-center justify-center rounded-md">
                            <img src="https://www.google.com/images/branding/googleg/1x/googleg_standard_color_18dp.png" alt="Google icon" class="w-5 h-5 mr-3"/>
                            "Đăng ký với Google"
                        </button>
                    </div>
                </div>
            </main>
        </div>
        <Footer />
    }
}

#[component]
pub fn Input(
    label_name: String,
    id: String,
    input_type: String,
    min_len: Option<u8>,
    max_len: Option<u16>,
    value: ReadSignal<String>,
    error: ReadSignal<String>,
    handle_onchange: impl Fn(Event) + 'static,
    handle_onblur: impl Fn() + 'static,
    pattern: Option<String>,
) -> impl IntoView {
    view! {
                              <div>
                                <label for={id.clone()} class="sr-only">{label_name.clone()}</label>
                                <input
                                type={input_type}
                                value={value}
                                on:change={move |e| handle_onchange(e)}
                                on:blur={move |_| handle_onblur()}
                                maxlength={max_len.unwrap_or(512)}
                                minlength={min_len.unwrap_or(0)}
                                id={id}
                                placeholder={label_name}
                                pattern={pattern.unwrap_or(".*".to_string())}
                                class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-lg placeholder-gray-500"/>
                                <Show
                                  when=move || { !error.get().is_empty() }
                                  fallback=|| {view! {} }
                                >
                                        <p class="text-red-500 text-xs mt-1">{move || error.get()}</p>
                                </Show>
                              </div>
    }
}
