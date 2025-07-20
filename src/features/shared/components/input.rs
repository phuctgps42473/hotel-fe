use js_sys::RegExp;
use leptos::{attr::AttributeValue, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn Input(
    label_name: String,
    id: String,
    input_type: String,
    min_len: Option<u8>,
    max_len: Option<u16>,
    value: impl AttributeValue,
    error: ReadSignal<Option<String>>,
    handle_onchange: impl Fn(Event) + 'static,
    handle_onblur: impl Fn() + 'static,
    pattern: Option<String>,
    custom_style: Option<&'static str>
) -> impl IntoView {
    // TODO: The idea is to create a pattern here
    let pattern = RegExp::new(&pattern.unwrap_or(String::new()), "");
    view! {
      <div>
        <label for={id.clone()} class="sr-only">{label_name.clone()}</label>
        <input
        type={input_type}
        value={value}
        on:change={move |e| {
          // And test it in here
          if pattern.test(&e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()) {
            handle_onchange(e);
          }
        }}
        on:blur={move |_| handle_onblur()}
        maxlength={max_len.unwrap_or(512)}
        minlength={min_len.unwrap_or(0)}
        id={id}
        placeholder={label_name}
        class={format!("input input-ghost w-full border-b border-gray-300 focus:outline-none focus:border-teal-custom pb-2 text-md placeholder-gray-500 {}", custom_style.unwrap_or(""))}
        />
        <Show
          when=move || { error.get().is_some() && !error.get().as_ref().unwrap().is_empty() }
          fallback=|| {view! {} }
        >
                <p class="text-red-500 text-xs mt-1">{move || error.get().unwrap()}</p>
        </Show>
      </div>
    }
}
