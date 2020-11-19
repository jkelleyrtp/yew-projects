use crate::state::eventbus::EventBus;
use crate::ui;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

use ui::{sidebar::SideBar, topbar::TopBar};
pub struct App {
    link: ComponentLink<Self>,
}

// No props, no messages, no updates, just a container for the children
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let bus = EventBus;

        App { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="button_container">
                <TopBar a="Hello!"/>
            </div>
        }
    }
}
