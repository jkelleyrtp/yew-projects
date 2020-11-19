pub mod effect;
pub mod listitem;
use listitem::FizzBuzz;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

const APPSTYLE: &'static str = include_str!("../static/styles/main.css");

struct MyApp {
    fizz_count: u32,
    link: ComponentLink<Self>,
}

enum Action {
    NewFizzBuzz,
}

impl Component for MyApp {
    type Properties = ();
    type Message = Action;

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        MyApp {
            fizz_count: 0,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Action::NewFizzBuzz => {
                self.fizz_count += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!(
            <html>
                <head>
                    <style>
                     {APPSTYLE}
                    </style>
                </head>
                <div>
                    <h1>{"Hello Yew! 🎉"}</h1>
                    <p>{"This is a paragraph"}</p>
                    <button onclick=self.link.callback(|_| Action::NewFizzBuzz)>{"New fizz"}</button>
                    <ul>
                        {(0..self.fizz_count).map(|idx| html!(<FizzBuzz num=idx/>)).collect::<Vec<Html>>()}
                    </ul>
                </div>
            </html>
        )
    }
}

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[wasm_bindgen(start)]
pub fn start_app() {
    yew::start_app::<MyApp>();
}
