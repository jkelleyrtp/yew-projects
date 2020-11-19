use super::objects::Task;
use indexmap::IndexMap;

/// The entire storage of the application state
pub struct ApplicationState {
    // pub tasks: IndexMap<String, Task>,
    pub tasks: Vec<Task>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            tasks: vec![], // tasks: IndexMap::new(),
        }
    }
}
