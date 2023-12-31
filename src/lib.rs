use cfg_if::cfg_if;
mod islands;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    extern crate console_error_panic_hook;
    use log::info;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        let _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        info!("hydrating app");

        leptos::leptos_dom::HydrationCtx::stop_hydrating();
    }
}}
