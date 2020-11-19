// use super::actions::add_todo_item;
use crate::prelude::{AppUpdateAction, ApplicationState};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use yew::worker::*;
pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
    appstate: ApplicationState,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = AppUpdateAction;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        EventBus {
            link,
            subscribers: HashSet::new(),
            appstate: ApplicationState::default(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        let res = msg.apply(&mut self.appstate);
    }
}
