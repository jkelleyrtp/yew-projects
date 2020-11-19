//! State of the entire application
// It's cool for us to store the entire application state here - we're an agent!
use super::objects::Task;
use crate::prelude::ApplicationState;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum AppUpdateAction {
    AddTodoItem(Task),
}

impl AppUpdateAction {
    pub fn apply(self, state: &mut ApplicationState) -> Result<()> {
        let res = match self {
            AppUpdateAction::AddTodoItem(t) => state.tasks.push(t),
        };
        Ok(())
    }
}

// Here are all the actions the application can take
mod addtask;
// pub use addtask::add_todo_item;
