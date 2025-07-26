use leptoaster::expect_toaster;
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{features::shared::components::spinner::Spinner, libs::fetcher::fetch};

#[derive(Debug, Deserialize, Clone)]
struct UserInfo {
    id: u64,
    #[serde(rename = "fullName")]
    fullname: String,
    email: String,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
}

#[derive(Serialize, Default, Clone)]
struct UpdateProfile {
    #[serde(rename = "fullName")]
    fullname: String,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
}

#[derive(Serialize, Default, Clone)]
struct ChangePassword {
    #[serde(rename = "oldPassword")]
    old_password: String,
    #[serde(rename = "newPassword")]
    new_password: String,
    #[serde(skip)]
    confirm_password: String,
}

#[component]
pub fn ProfileInformation(id: u32) -> impl IntoView {
    let (info, set_info) = signal(None);
    let (change_pass, set_change_pass) = signal(ChangePassword::default());

    // let (form_error, set_form_error) = signal(None);

    Effect::new(move || {
        spawn_local(async move {
            match fetch::<(), UserInfo>(&format!("profile/{}", id), "GET", None).await {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        set_info.set(res.data);
                    }
                }
            }
        });
    });

    let (is_editing, set_is_editing) = signal(false);

    let toaster = expect_toaster();
    let handle_change_info = move |_| {
        let user_info = info.read().clone().unwrap();
        if user_info.fullname.is_empty() || user_info.phone_number.is_empty() {
            toaster.error("Cannot leave required field empty or blank");
            return;
        }

        let toaster = toaster.clone();
        spawn_local(async move {
            match fetch::<UpdateProfile, UserInfo>(
                &format!("profile/{}", id),
                "PUT",
                Some(UpdateProfile {
                    fullname: user_info.fullname,
                    phone_number: user_info.phone_number,
                }),
            )
            .await
            {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    leptos::logging::log!("{:#?}", res);
                    if res.code == 200 {
                        toaster.success("Profile update successfully");
                    }
                }
            }
        });
    };

    let toaster = expect_toaster();
    let handle_change_password = move |_| {
        let ch_pass = change_pass.get();
        if !ch_pass.new_password.eq(&ch_pass.confirm_password) {
            toaster.error("New password does not match");
            return;
        }

        let toaster = toaster.clone();
        spawn_local(async move {
            match fetch::<ChangePassword, UserInfo>(
                &format!("profile/{}/change-password", id),
                "PUT",
                Some(ch_pass),
            )
            .await
            {
                Err(e) => leptos::logging::log!("{:#?}", e),
                Ok(res) => {
                    if res.code == 200 {
                        toaster.success("Change password successfully");
                    }
                }
            }
        });
    };

    view! {
            <h2 class="text-2xl font-bold text-base-content mb-6">Thông Tin Cá Nhân</h2>

            // Avatar Section
            <div class="flex items-center gap-6 mb-8">
                <div class="avatar relative">
                    <div class="w-24 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2">
                        <img src="https://i.pravatar.cc/150?img=5" alt="User Avatar" />
                    </div>
                     <button class="btn btn-xs btn-circle btn-neutral absolute bottom-0 right-0">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4"><path d="M10 3.75a2 2 0 100 4 2 2 0 000-4zM16.667 15.417a1.667 1.667 0 00-1.667-1.667H5a1.667 1.667 0 00-1.667 1.667c0 1.523 2.11 2.75 4.889 2.75h4.556c2.779 0 4.889-1.227 4.889-2.75z" /></svg>
                    </button>
                </div>
                <div>
                  <Show
                    when={move || info.read().is_some()}
                    fallback={|| view!{ <Spinner /> }}
                  >
                      <p class="text-xl font-bold text-base-content">{info.read().as_ref().unwrap().fullname.to_string()}</p>
                      <p class="text-base-content/70">{info.read().as_ref().unwrap().email.to_string()}</p>
                  </Show>
                </div>
            </div>

            // Form Section
            <div class="space-y-4">
                  <Show
                    when={move || info.read().is_some()}
                    fallback={|| view!{ <Spinner /> }}
                  >
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="form-control">
                            <label class="label"><span class="label-text">Họ và tên</span></label>
                            <input
                              on:change={move |e| set_info.write().as_mut().unwrap().fullname = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()}
                             type="text" value={info.read().as_ref().unwrap().fullname.to_string()} class="input input-bordered bg-base-200" readonly=move || !*is_editing.read() />
                        </div>
                        <div class="form-control">
                            <label class="label"><span class="label-text">Email</span></label>
                            <input type="email" value={info.read().as_ref().unwrap().email.to_string()} class="input input-bordered bg-base-200" readonly=true />
                        </div>
                        <div class="form-control">
                            <label class="label"><span class="label-text">Số điện thoại</span></label>
                            <input
                              on:change={move |e| set_info.write().as_mut().unwrap().phone_number = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()}
                             type="tel" value={info.read().as_ref().unwrap().phone_number.to_string()} class="input input-bordered bg-base-200" readonly=move || !*is_editing.read() />
                        </div>
                    </div>
                  </Show>

                <div class="pt-4 flex justify-end gap-3">
                      {move || {
                        let handle_clone = handle_change_info.clone();
                        match is_editing.read().clone() {
                          // view! TAKES OWNERSHIP
                          true => view! {
                            <button type="button" class="btn btn-ghost" on:click=move |_| set_is_editing.set(false)>Hủy</button>
                            <button type="submit" class="btn btn-success text-success-content" on:click={handle_clone}>Lưu Thay Đổi</button>
                          }.into_any(),
                          false => view! { <button type="button" class="btn btn-primary" on:click=move |_| set_is_editing.set(true)>Chỉnh Sửa</button> }.into_any()
                        }
                      }}
                </div>
            </div>

            <div class="divider my-8">Đổi Mật Khẩu</div>

            <div class="space-y-4 max-w-md">
                <div class="form-control">
                    <label class="label"><span class="label-text">Mật khẩu hiện tại</span></label>
                    <input type="password" on:change={move |e| set_change_pass.write().old_password = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()} placeholder="••••••••" class="input input-bordered" />
                </div>
                 <div class="form-control">
                    <label class="label"><span class="label-text">Mật khẩu mới</span></label>
                    <input type="password" on:change={move |e| set_change_pass.write().new_password = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()} placeholder="••••••••" class="input input-bordered" />
                </div>
                 <div class="form-control">
                    <label class="label"><span class="label-text">Xác nhận mật khẩu mới</span></label>
                    <input type="password" on:change={move |e| set_change_pass.write().confirm_password = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()} placeholder="••••••••" class="input input-bordered" />
                </div>
                <div class="pt-4 flex justify-end">
                    <button on:click={handle_change_password} class="btn btn-primary">Cập Nhật Mật Khẩu</button>
                </div>
            </div>
    }
}
