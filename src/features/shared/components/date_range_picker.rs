use js_sys::Function;
use leptos::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::HtmlInputElement;

#[component]
pub fn DateRangePicker(id: String, date_range_setter: WriteSignal<String>) -> impl IntoView {
    let id_cloned = id.clone();

    Effect::new(move |_| {
        let window = web_sys::window().expect("Không tìm thấy window");
        let document = window.document().expect("Không tìm thấy document");
        let input_el = document
            .get_element_by_id(&id)
            .expect("Không tìm thấy element #datePicker");

        let listener = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
            let date_str = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            date_range_setter.set(date_str);
        }) as Box<dyn FnMut(_)>);

        input_el
            .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
            .expect("Lỗi khi thêm event listener");
        listener.forget();

        let init_pikaday = Function::new_with_args(
            "elm",
            r#"
            setTimeout(() => {
                if (typeof flatpickr === 'undefined') {
                    console.error('flatpickr không được tải');
                    return;
                }
                flatpickr(elm, {
                  mode: "range",
                  minDate: "today",
                  dateFormat: "Y-m-d",
                  disable: [
                    {
                        from: "2025-06-11",
                        to: "2025-06-21"
                    },
                    {
                        from: "2025-07-01",
                        to: "2025-12-01"
                    }
                  ]
                });
            }, 500);
        "#,
        );

        if let Err(err) = init_pikaday.call1(&JsValue::NULL, &input_el.into()) {
            leptos::logging::error!("Lỗi khi khởi tạo Date Picker: {:?}", err);
        }
    });

    view! {
      <input readonly type="text" id={id_cloned} />
    }
}
