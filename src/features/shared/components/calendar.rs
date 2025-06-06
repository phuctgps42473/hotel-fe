use js_sys::Function;
use leptos::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::HtmlInputElement;

#[component]
pub fn Calendar(id: String, date_setter: WriteSignal<String>) -> impl IntoView {
    let id_cloned = id.clone();

    Effect::new(move |_| {
        let window = web_sys::window().expect("Không tìm thấy window");
        let document = window.document().expect("Không tìm thấy document");
        let input_el = document
            .get_element_by_id(&id)
            .expect("Không tìm thấy element #datePicker");

        let init_pikaday = Function::new_with_args(
            "element",
            r#"
            setTimeout(() => {
                if (typeof Pikaday === 'undefined') {
                    console.error('Pikaday không được tải');
                    return;
                }
                const picker = new Pikaday({
                    field: element,
                    format: 'DD-MM-YYYY',
                });
            }, 500);
        "#,
        );

        if let Err(err) = init_pikaday.call1(&JsValue::NULL, &input_el.into()) {
            leptos::logging::error!("Lỗi khi khởi tạo Pikaday: {:?}", err);
        }

        let listener = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
            let date_str = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            date_setter.set(date_str);
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("dateSelected", listener.as_ref().unchecked_ref())
            .expect("Lỗi khi thêm event listener");

        listener.forget();
    });

    view! {
      <input readonly type="text" class="input pika-single" id={id_cloned} />
    }
}
