use crate::components::{JsonInput, SchemaDisplay, TransformPanel};
use crate::core::{JsonType, Transform, TransformPipeline};
use leptos::*;
use serde_json::Value;

#[component]
pub fn App() -> impl IntoView {
    // Application state
    let (json_input, set_json_input) = create_signal(String::new());
    let (parsed_json, set_parsed_json) = create_signal::<Option<Value>>(None);
    let (json_type, set_json_type) = create_signal::<Option<JsonType>>(None);
    let (pipeline, set_pipeline) = create_signal(TransformPipeline::new());
    let (transform_result, set_transform_result) = create_signal::<Option<Value>>(None);
    let (error_message, set_error_message) = create_signal::<Option<String>>(None);

    // Parse JSON whenever input changes
    create_effect(move |_| {
        let input = json_input.get();
        if input.trim().is_empty() {
            set_parsed_json.set(None);
            set_json_type.set(None);
            set_error_message.set(None);
            return;
        }

        match serde_json::from_str::<Value>(&input) {
            Ok(value) => {
                let inferred_type = JsonType::infer(&value);
                set_parsed_json.set(Some(value));
                set_json_type.set(Some(inferred_type));
                set_error_message.set(None);
            }
            Err(e) => {
                set_error_message.set(Some(format!("JSON Parse Error: {}", e)));
                set_parsed_json.set(None);
                set_json_type.set(None);
            }
        }
    });

    // Apply transformations whenever pipeline or parsed JSON changes
    create_effect(move |_| {
        if let Some(json) = parsed_json.get() {
            let pipe = pipeline.get();
            match pipe.apply(json) {
                Ok(result) => {
                    set_transform_result.set(Some(result));
                    set_error_message.set(None);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Transform Error: {}", e)));
                    set_transform_result.set(None);
                }
            }
        } else {
            set_transform_result.set(None);
        }
    });

    view! {
        <div class="app-container">
            <header>
                <h1>"Hermes"</h1>
            </header>

            <main class="main-content">
                <JsonInput
                    value=json_input
                    on_change=set_json_input
                />

                <div class="right-panel">
                    {move || error_message.get().map(|err| view! {
                        <div class="error-message">{err}</div>
                    })}

                    <SchemaDisplay
                        json_type=json_type
                    />

                    <TransformPanel
                        pipeline=pipeline
                        set_pipeline=set_pipeline
                        transform_result=transform_result
                    />
                </div>
            </main>
        </div>
    }
}
