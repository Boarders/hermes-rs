use leptos::*;
use wasm_bindgen::prelude::*;
use serde_json::Value;
use crate::core::{Transform, TransformPipeline};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

#[component]
pub fn TransformPanel(
    pipeline: ReadSignal<TransformPipeline>,
    set_pipeline: WriteSignal<TransformPipeline>,
    transform_result: ReadSignal<Option<Value>>,
) -> impl IntoView {
    let (transform_input, set_transform_input) = create_signal(String::new());
    let result_ref = create_node_ref::<html::Code>();

    create_effect(move |_| {
        if transform_result.get().is_some()
	    && let Some(element) = result_ref.get() {
                highlightElement(&element);
        }
    });

    let add_transform_logic = move || {
        let input = transform_input.get();
        if let Ok(transform) = Transform::from_jq(&input) {
            let mut pipe = pipeline.get();
            pipe.add(transform);
            set_pipeline.set(pipe);
            set_transform_input.set(String::new());
        }
    };

    let add_transform_click = {
        move |_| add_transform_logic()
    };

    let add_transform_keyup = {
        move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Enter" {
                add_transform_logic();
            }
        }
    };

    let clear_pipeline = move |_| {
        set_pipeline.set(TransformPipeline::new());
    };

    view! {
        <section class="transform-section">
            <h2>"Transformations"</h2>

            <div class="transform-input-area">
                <input
                    type="text"
                    class="transform-input"
                    placeholder="Enter jq expression (e.g., .field)"
                    prop:value=move || transform_input.get()
                    on:input=move |ev| {
                        set_transform_input.set(event_target_value(&ev));
                    }
                    on:keyup=add_transform_keyup
                />
                <button on:click=add_transform_click>"Add Transform"</button>
                <button on:click=clear_pipeline>"Clear"</button>
            </div>

            <div class="pipeline-display">
                <h3>"Current Pipeline"</h3>
                <pre class="jq-syntax">{move || pipeline.get().to_jq()}</pre>
            </div>

            <div class="transform-result">
                <h3>"Result"</h3>
                {move || {
                    transform_result.get().map(|result| view! {
                        <pre class="result-json"><code node_ref=result_ref class="language-json">{
                            serde_json::to_string_pretty(&result).unwrap_or_default()
                        }</code></pre>
                    })
                }}
            </div>
        </section>
    }
}
