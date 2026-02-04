use leptos::*;
use wasm_bindgen::prelude::*;
use crate::core::JsonType;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

#[component]
pub fn SchemaDisplay(
    json_type: ReadSignal<Option<JsonType>>,
) -> impl IntoView {
    let code_ref = create_node_ref::<html::Code>();

    create_effect(move |_| {
        if json_type.get().is_some() {
            if let Some(element) = code_ref.get() {
                highlightElement(&element);
            }
        }
    });

    view! {
        <section class="schema-section">
            <div class="schema-box">
                <h2>"Inferred Type"</h2>
                {move || {
                    json_type.get().map(|jt| view! {
                        <pre class="type-display"><code node_ref=code_ref class="language-json">{jt.to_pretty()}</code></pre>
                    })
                }}
            </div>

        </section>
    }
}
