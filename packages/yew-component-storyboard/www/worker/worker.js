import("../../crate-wasm/pkg").then(wasm => {
  // Call the library funciton we exported with wasm-bindgen in lib.rs
  wasm.init_worker();
});
