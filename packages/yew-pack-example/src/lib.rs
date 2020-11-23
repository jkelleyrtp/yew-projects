use yew;

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}

struct MyApp {}
impl yew::Component for MyApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div align="center">
              <h1>{"📦✨  yew-pack"}</h1>
              <p>
                <strong>{"Tooling to supercharge Yew projects"}</strong>
              </p>
            </div>
        }
    }
}
