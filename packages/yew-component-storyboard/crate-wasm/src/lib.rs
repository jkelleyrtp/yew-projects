#![recursion_limit = "256"]

pub mod app;
pub mod redux;
pub mod state;
pub mod storybook;
pub mod ui;
pub mod utils;
pub mod worker;

pub mod prelude {
    use super::*;
    pub use {app::App, state::actions::AppUpdateAction, state::appstate::ApplicationState};
}

use wasm_bindgen::prelude::*;

//====== Running the primary frontend ======//

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}

//====== Running the worker ======///

// We need to import the Threaded trait to register the worker
use yew::agent::Threaded;

/// This gets called by the worker.js entrypoint
/// We need to wrap it in wasm_bindgen so the worker knows the spin the the yew worker instance
#[wasm_bindgen]
pub fn init_worker() {
    // Set the panic flags
    utils::init();

    // Spawning a yew component without StartApp requires initializing
    yew::initialize();

    // ... registering the worker
    worker::Worker::register();

    // ... and then starting the run loop
    yew::run_loop();
}

// you would need a new launch function for each of the unique workers you want to register
