use leptos::*;

mod core;
mod components;
mod utils;

use components::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
