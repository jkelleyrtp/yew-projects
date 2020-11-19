use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::store;

pub struct TopBar {
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub a: String,
}

pub enum Msg {
    Click,
}

enum b {}

impl Component for TopBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TopBar { link }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( wasm-bindgen )" }</button>
            </div>
        }
    }
}
