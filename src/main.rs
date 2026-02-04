use leptos::*;

mod components;
mod core;
mod utils;

use components::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
