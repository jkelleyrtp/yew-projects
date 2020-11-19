use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct SideBar {
    link: ComponentLink<Self>,
}

pub enum SideBarMsg {
    Click,
}

impl Component for SideBar {
    type Message = SideBarMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SideBar { link }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SideBarMsg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| SideBarMsg::Click)>{ "Click ( wasm-bindgen )" }</button>
            </div>
        }
    }
}

// use super::event_bus::EventBus;
// use yew::agent::Bridged;
// use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};

// pub enum Msg {
//     NewMessage(String),
// }

// pub struct Subscriber {
//     message: String,
//     _producer: Box<dyn Bridge<EventBus>>,
// }

// impl Component for Subscriber {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let callback = link.callback(Msg::NewMessage);
//         let _producer = EventBus::bridge(callback);
//         Subscriber {
//             message: "No message yet.".to_string(),
//             _producer,
//         }
//     }

//     fn change(&mut self, _: Self::Properties) -> bool {
//         false
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::NewMessage(s) => self.message = s,
//         }
//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <h1>{ &self.message }</h1>
//         }
//     }
// }
