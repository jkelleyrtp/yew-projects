pub mod components;
pub mod state;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[wasm_bindgen(start)]
pub fn start_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::MyApp>();
}
