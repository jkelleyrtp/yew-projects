mod app;
mod state;
mod zew;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasm_logger;
use yew;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[wasm_bindgen(start)]
pub fn launch() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::MyApp>();
}
