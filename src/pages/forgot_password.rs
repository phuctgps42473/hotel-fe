use leptoaster::expect_toaster;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_navigate;
use serde::Serialize;

use crate::libs::fetcher::fetch;

// Component Icon để trang trí
#[component]
fn LockIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
        </svg>
    }
}

#[component]
fn MailSentIcon() -> impl IntoView {
    view! {
         <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
    }
}

#[derive(Debug, Serialize, Clone)]
struct ForgotPasswordRequest {
    email: String,
}

#[component]
pub fn ForgotPassword() -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (email_submitted, set_email_submitted) = signal(false);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let email = email.get();
        if !email.is_empty() {
            spawn_local(async move {
                match fetch::<ForgotPasswordRequest, String>(
                    "forgot-password",
                    "POST",
                    Some(ForgotPasswordRequest { email }),
                )
                .await
                {
                    Err(e) => leptos::logging::error!("{:#?}", e),
                    Ok(res) => {
                        if res.code == 200 {
                            leptos::logging::log!("THANH CONG");
                            set_email_submitted.set(true);
                        }
                    }
                }
            });
        }
    };

    view! {
        <main class="min-h-screen bg-base-200 flex flex-col items-center justify-center p-4 font-sans">
            <h1 class="text-3xl font-bold text-primary tracking-wider mb-8">
                <a href="/">ELARIS HOTEL</a>
            </h1>

            <div class="card w-full max-w-md bg-base-100 shadow-xl rounded-[var(--radius-selector)]">
                <div class="card-body p-8 md:p-10">

                    <Show
                        when=move || !email_submitted.get()
                        fallback=move ||  ResetPasword(ResetPaswordProps { email })                   >
                        // --- TRẠNG THÁI BAN ĐẦU ---
                        <div class="flex flex-col items-center text-center space-y-4">
                            <div class="text-primary">
                                <LockIcon />
                            </div>
                            <h2 class="card-title text-2xl text-base-content">Quên Mật Khẩu?</h2>
                            <p class="text-base-content/70">
                                "Đừng lo lắng! Chuyện này vẫn thường xảy ra. Hãy nhập địa chỉ email của bạn và chúng tôi sẽ gửi hướng dẫn để đặt lại mật khẩu."
                            </p>
                        </div>

                        <form class="space-y-6 pt-4" on:submit=handle_submit>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">Địa chỉ email</span>
                                </label>
                                <input
                                    type="email"
                                    placeholder="your.email@example.com"
                                    class="input input-bordered w-full rounded-[var(--radius-field)]"
                                    required=true
                                    // Cập nhật signal khi người dùng nhập
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    prop:value=email
                                />
                            </div>
                            <div class="card-actions justify-end w-full">
                                <button type="submit" class="btn btn-primary w-full rounded-[var(--radius-box)] text-lg">
                                    Nhận mã OTP
                                </button>
                            </div>
                        </form>
                    </Show>
                </div>
            </div>
             <div class="mt-6 text-center">
                <a href="/login" class="link link-hover text-base-content/80">"Nhớ ra rồi? Quay lại Đăng nhập"</a>
            </div>
        </main>
    }
}

#[derive(Debug, Serialize, Clone)]
struct ResetPasswordRequest {
    email: String,
    otp: String,
    #[serde(rename = "newPassword")]
    password: String,
}
#[component]
fn ResetPasword(email: ReadSignal<String>) -> impl IntoView {
    let (otp, set_otp) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_pass, set_confirm_pass) = signal(String::new());

    let toaster = expect_toaster();
    let navigate = use_navigate();

    let toast_cl = toaster.clone();
    let nav_cl = navigate.clone();
    let handle_submit = move || {
        let email = email.get();
        let otp = otp.get();
        let password = password.get();
        let confirm_pass = confirm_pass.get();
        if password != confirm_pass {
            toast_cl.error("Password does not match");
            return;
        }

        let toast_cl2 = toast_cl.clone();
        let nav_cl2 = nav_cl.clone();
        spawn_local(async move {
            match fetch::<ResetPasswordRequest, String>(
                "reset-password",
                "POST",
                Some(ResetPasswordRequest {
                    email,
                    otp,
                    password,
                }),
            )
            .await
            {
                Err(e) => leptos::logging::error!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        toast_cl2.success("Reset password successfully!");
                        nav_cl2("/login", Default::default());
                    }
                }
            }
        });
    };

    view! {
        <div class="flex flex-col items-center text-center space-y-4">
            <div class="text-success">
                <MailSentIcon />
            </div>
            <h2 class="card-title text-2xl text-base-content">Vui Lòng Kiểm Tra Email</h2>
            <p class="text-base-content/70">
                "Nếu có một tài khoản được liên kết với "
                <span class="font-bold text-base-content">{ email }</span>
                ", chúng tôi đã gửi một email chứa liên kết để bạn đặt lại mật khẩu."
            </p>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Mã OTP</span>
                </label>
                <input
                    type="number"
                    placeholder="XXXXXX"
                    on:input=move |ev| set_otp.set(event_target_value(&ev))
                    prop:value=otp
                    class="input input-bordered w-full rounded"
                    required=true
                />
                <label class="label">
                    <span class="label-text">Mật khẩu mới</span>
                </label>
                <input
                    type="password"
                    placeholder="********"
                    class="input input-bordered w-full rounded"
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                    prop:value=password
                    required=true
                />
                <label class="label">
                    <span class="label-text">Nhập lại mật khẩu mới</span>
                </label>
                <input
                    type="password"
                    placeholder="********"
                    class="input input-bordered w-full rounded"
                    on:input=move |ev| set_confirm_pass.set(event_target_value(&ev))
                    prop:value=confirm_pass
                    required=true
                />
            </div>
            <div class="card-actions justify-end w-full">
                <button on:click=move |_| {handle_submit()} type="submit" class="btn btn-primary w-full rounded-[var(--radius-box)] text-lg">
                    Đặt lại mật khẩu
                </button>
            </div>
            <div class="card-actions justify-center w-full pt-4">
                <a href="/login" class="btn btn-primary btn-outline w-full rounded-[var(--radius-box)]">Quay Lại Trang Đăng Nhập</a>
            </div>
        </div>
    }
}
