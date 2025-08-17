use leptos::{portal::Portal, prelude::*};

#[component]
pub fn Modal<C>(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    C: IntoView + 'static,
{
    let children = children.into_inner();
    view! {
        <Portal>
            <dialog class="modal" class:modal-open={show_modal}>
                <div class="modal-box w-11/12 max-w-3xl">
                        {children()}
                    <div class="modal-action mt-6">
                        <button class="btn rounded-[var(--radius-box)]" on:click=move |_| set_show_modal.set(false)>Đóng</button>
                    </div>
                </div>

                <form method="dialog" class="modal-backdrop">
                    <button on:click=move |_| set_show_modal.set(false)>close</button>
                </form>
            </dialog>
        </Portal>
    }
}
