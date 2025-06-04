use std::sync::Arc;

use leptos::{portal::Portal, prelude::*};

#[component]
pub fn Modal(
    hide_modal_fn: impl Fn() -> () + Send + Sync + Clone + 'static,
    children: ChildrenFn,
) -> impl IntoView {
    let hide_modal_fn = Arc::new(hide_modal_fn);

    view! {
        <Portal>
          <div
            on:click={
                let hide_modal_fn = hide_modal_fn.clone();
                move |_| {
                    leptos::logging::log!("Clicked overlay");
                    (hide_modal_fn)();
                }
            }
            class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
          >
            <div
              on:click=move |ev|  ev.stop_propagation()
              class="relative p-4 w-full max-w-md mx-auto max-h-full"
            >
              {children()}
            </div>
          </div>
        </Portal>
    }
}
