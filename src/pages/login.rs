use std::time::Duration;

use js_sys::{RegExp, JSON};
use leptoaster::expect_toaster;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, HtmlInputElement, MessageEvent, SubmitEvent};

use crate::{
    app::{GlobalState, UserState}, layouts::public::{footer::Footer, header::Header}, libs::fetcher::fetch
};

#[derive(Serialize, Clone)]
struct LoginForm {
    email: String,
    password: String,
}

#[derive(Clone, Deserialize, Debug)]
struct LoginResponse {
    #[serde(rename = "accessToken")]
    access_token: Option<String>,
    #[serde(rename = "userInfo")]
    user_info: Option<UserState>,
}

#[component]
pub fn Login() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    Effect::new(move || {
      if state.read().user.is_some() {
        use_navigate()("/", Default::default());
      }
    });

    let navigate = leptos_router::hooks::use_navigate();
    let toaster = expect_toaster();
    let (disable_button, set_disable_button) = signal(true);
    let (email, set_email) = signal(String::new());
    let (email_error, set_email_error) = signal(None);
    let (password, set_password) = signal(String::new());
    let (password_error, set_password_error) = signal(None);

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
            set_email_error.set(Some("Email is required".to_string()));
        } else if !tester.test(email.as_borrowed()) {
            set_email_error.set(Some("Invalid email account".to_string()));
        } else {
            set_email_error.set(Some("".to_string()));
            set_disable_button.set(
                password_error.read().is_none()
                    || !String::is_empty(password_error.read().as_ref().unwrap()),
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
    let validate_password = move || {
        let password = password.read();
        if password.is_empty() {
            set_password_error.set(Some("Password is required".to_string()));
        } else if password.len() < 6 {
            set_password_error.set(Some("Password is too short".to_string()));
        } else if password == password.to_lowercase() {
            set_password_error.set(Some(
                "Password should have uppcase letters and number".to_string(),
            ));
        } else if !password.chars().any(|c| c.is_ascii_digit()) {
            set_password_error.set(Some(
                "Password should have uppcase letters and number".to_string(),
            ));
        } else {
            set_password_error.set(Some("".to_string()));
            set_disable_button.set(
                email_error.read().is_none()
                    || !String::is_empty(email_error.read().as_ref().unwrap()),
            );
            return;
        }
        set_disable_button.set(true);
    };

    let navigate_clone = navigate.clone();
    let toaster_clone = toaster.clone();
    let handle_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let email = email.get();
        let password = password.get();

        let navigate_clone = navigate_clone.clone();
        let toaster_clone = toaster_clone.clone();
        spawn_local(async move {
            match fetch::<LoginForm, LoginResponse>(
                "login",
                "POST",
                Some(LoginForm { email, password }),
            )
            .await
            {
                Ok(body) => match body.code {
                    200 => {
                        toaster_clone.success("Successfully Logged In.Redirecting to HomePage");
                        let data = body.data.unwrap();
                        set_local_storage("accessToken", data.access_token.as_ref().unwrap());
                        state.write().user = data.user_info;
                        set_timeout(
                            move || {
                                navigate_clone("/home", Default::default());
                            },
                            Duration::from_millis(1000),
                        );
                    }
                    401 => toaster_clone.error(body.error.unwrap().message),
                    500 => toaster_clone.error("Something wrong with the system"),
                    _ => leptos::logging::log!("Unexpected Response Status"),
                },
                Err(error) => {
                    leptos::logging::log!("{:?}", error)
                }
            }
        });
    };

    let toaster_clone1 = toaster.clone();
    let navigate_clone1 = navigate.clone();
    Effect::new(move || {
        let toaster_clone1 = toaster_clone1.clone();
        let navigate_clone1 = navigate_clone1.clone();
        let handler = Closure::wrap(Box::new(move |e: MessageEvent| {
            if e.origin().eq("http://localhost:8080") {
                let json = JSON::stringify(&e.data()).unwrap().as_string().unwrap();
                let data: LoginResponse = serde_json::from_str(&json).unwrap();

                toaster_clone1.success("Successfully Logged In.Redirecting to HomePage");
                set_local_storage("accessToken", &data.access_token.unwrap());
                navigate_clone1("/home", Default::default());
            }
        }) as Box<dyn FnMut(_)>);

        window()
            .add_event_listener_with_callback("message", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    });

    let oauth_login = |_| {
        window()
            .open_with_url_and_target_and_features(
                &generate_google_login_url("/"),
                "Google Oauth Window",
                "popup,left=400,width=500,height=800",
            )
            .unwrap()
            .unwrap();
    };

    view! {
      <Header />
      <div class="min-h-screen flex flex-col bg-[url(/images/register-bg.jpg)] bg-cover bg-center bg-fixed">
              // Main Login Content
              <main class="flex-grow flex items-center justify-center p-4 md:p-8">
                  <div class="relative w-full max-w-4xl mx-auto bg-base-100 rounded-3xl shadow-xl overflow-hidden
                              flex flex-col lg:flex-row min-h-[500px]">

                      // Left Image Section
                      <div class="relative w-full lg:w-1/2 bg-[url(/images/register-form.jpg)] bg-cover bg-center rounded-t-3xl lg:rounded-l-3xl lg:rounded-tr-none min-h-[250px] lg:min-h-full">
                          <div class="absolute inset-0 bg-gradient-to-t from-teal-custom via-teal-custom/50 to-transparent opacity-70"></div>
                          <div class="absolute inset-0 flex items-end justify-start p-8 text-white z-10">
                              <h2 class="text-4xl font-bold">"ELARIS HOTEL"</h2>
                          </div>
                      </div>

                      // Right Form Section
                      <div class="w-full lg:w-1/2 p-8 md:p-12 flex flex-col justify-center">
                          <h1 class="text-4xl font-bold text-gray-800 text-center mb-8">"ĐĂNG NHẬP"</h1>

                          <form
                          on:submit={handle_submit}
                          class="space-y-6">
                              <Input
                                id={"email".to_string()}
                                input_type={"email".to_string()}
                                label_name={"Email".to_string()}
                                min_len={Some(6)}
                                max_len={Some(50)}
                                value={email}
                                error={email_error}
                                handle_onchange={handle_email_change}
                                handle_onblur={validate_email}
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

                              <div class="flex justify-between items-center text-lg">
                                  <a href="/register" class="text-gray-600 hover:text-teal-custom font-semibold">"Đăng ký"</a>
                                  <a href="/forgot-password" class="text-teal-custom hover:text-teal-light font-semibold">"Quên mật khẩu?"</a>
                              </div>

                              <button disabled={disable_button} type="submit" class="btn btn-info w-full bg-teal-custom hover:bg-teal-light text-white text-lg font-semibold py-3 rounded-md mt-6">
                                  "Đăng nhập"
                              </button>
                          </form>

                          <div class="divider text-gray-400 text-sm my-6">"Hoặc tiếp tục với"</div>

                          <button on:click={oauth_login} class="btn btn-outline w-full border-gray-300 hover:border-teal-custom hover:bg-teal-50 hover:text-gray-800 text-gray-600 font-semibold py-3 flex items-center justify-center rounded-md">
                              <img src="https://www.google.com/images/branding/googleg/1x/googleg_standard_color_18dp.png" alt="Google icon" class="w-5 h-5 mr-3"/>
                              "Đăng nhập với Google"
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
    error: ReadSignal<Option<String>>,
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
                                class="input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-md placeholder-gray-500"/>
                                <Show
                                  when=move || { error.get().is_some() && !error.get().as_ref().unwrap().is_empty() }
                                  fallback=|| {view! {} }
                                >
                                        <p class="text-red-500 text-xs mt-1">{move || error.get().unwrap()}</p>
                                </Show>
                              </div>
    }
}

fn generate_google_login_url(redirect_path: &str) -> String {
    let client_id = "778948573201-rlq52k0i3cqc12oeponb08qgq6s7h0pt.apps.googleusercontent.com";
    let redirect_uri = "http://localhost:8080/api/oauth/code_grant/google";
    let response_type = "code";
    let scope = "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile";
    let include_granted_scopes = "true";
    let prompt = "consent";

    format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={client_id}&redirect_uri={redirect_uri}&response_type={response_type}&scope={scope}&state={redirect_path}&include_granted_scopes={include_granted_scopes}&prompt={prompt}")
}

fn set_local_storage(key: &str, value: &str) {
    window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set(key, value)
        .unwrap();
}
