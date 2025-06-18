pub mod app;
pub mod oauth;
pub mod balance_component;
#[cfg(feature="ssr")]
pub mod connection;
pub mod models;
#[cfg(feature="ssr")]
pub mod schema;
pub mod balance;

#[cfg(feature="ssr")]
pub mod create_user;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
