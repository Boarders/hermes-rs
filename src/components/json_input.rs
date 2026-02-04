use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

#[component]
pub fn JsonInput(
    value: ReadSignal<String>,
    on_change: WriteSignal<String>,
) -> impl IntoView {
    let code_ref = create_node_ref::<html::Code>();

    // Re-highlight when value changes
    create_effect(move |_| {
        let _ = value.get();
        if let Some(element) = code_ref.get() {
            highlightElement(&element);
        }
    });

    view! {
        <section class="json-input-section">
            <h2>"Input"</h2>
            <pre class="json-input">
                <code
                    node_ref=code_ref
                    class="language-json"
                    contenteditable="true"
                    placeholder="<enter json>"
                    on:input=move |ev| {
                        let target = ev.target().unwrap();
                        let element = target.dyn_into::<web_sys::HtmlElement>().unwrap();
                        let text = element.inner_text();
                        on_change.set(text);
                    }
                >{move || value.get()}</code>
            </pre>
        </section>
    }
}
