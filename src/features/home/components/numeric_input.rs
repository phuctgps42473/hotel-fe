use leptos::prelude::*;

#[component]
pub fn NumericInput() -> impl IntoView {
    let (number, set_number) = signal(Ok(0));

    view! {
      <h1>Input the number</h1>
      <input type="number" on:input:target=move |ev| {
        set_number.set(ev.target().value().parse());
      }
      />
      <ErrorBoundary
      fallback=|errors| view! {
        <p>Not a number!</p>
        <ul>
      {
        move || errors.get().into_iter().map(|(id, e)| view! {<li>{format!("{}:{}",id.to_string(), e.to_string())}</li>}).collect_view()
      }
        </ul>

      }
      >
      <p>You entered: <strong>{number}</strong></p>
      </ErrorBoundary>
    }
}
