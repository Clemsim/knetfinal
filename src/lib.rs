pub mod app;
#[cfg(feature="ssr")]
pub mod connection;
#[cfg(feature="ssr")]
pub mod models;
#[cfg(feature="ssr")]
pub mod schema;
#[cfg(feature="ssr")]
pub mod balance;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
