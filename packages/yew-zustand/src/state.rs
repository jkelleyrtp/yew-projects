use im_rc::hashmap::HashMap;
use serde::{Deserialize, Serialize};

struct AppState {}

// ====================
// Thunks
// ====================
#[derive(Serialize, Deserialize)]
struct ExampleThunk {}

impl Thunk<AppState> for ExampleThunk {
    fn reduce(self, state: &AppState) {}
}

trait Thunk<RootState> {
    fn reduce(self, state: &RootState);
}

pub fn dispatch_thunk<T>(thunk: T) {}

// ====================
// Synchronous Reducers
// ====================
#[derive(Serialize, Deserialize)]
struct ExampleReducer {}
