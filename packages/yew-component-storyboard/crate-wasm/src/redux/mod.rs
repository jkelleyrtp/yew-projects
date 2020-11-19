mod component;
mod runnerslice;

use runnerslice::{RunnerAction, RunnerSlice};

pub struct Store {
    runner: RunnerSlice,
}

fn build_store() -> Store {
    Store {
        runner: RunnerSlice {
            running: false,
            steps_left: 0,
        },
    }
}
