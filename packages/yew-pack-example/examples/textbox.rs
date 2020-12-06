use yew::prelude::*;
use yew_functional::function_component;
// Called when the wasm module is instantiated
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}

#[function_component(MyApp)]
fn my_app() -> Html {
    html! {
        <Storyboard>
            <components::Textbox />
            <components::RadioInput />
        </Storyboard>
    }
}

#[derive(yew::Properties, PartialEq, Debug, Clone)]
struct StoryboardProps {
    children: yew::Children,
}

#[function_component(Storyboard)]
fn storyboard(props: &StoryboardProps) -> Html {
    html! {
        <div>
            <h1>{"Storyboard of Components"}</h1>

            {props.children.clone()}
        </div>
    }
}

mod components {
    use super::*;

    #[function_component(Textbox)]
    pub fn textbox() -> Html {
        html! {
            <div>
                <input />
            </div>
        }
    }

    #[function_component(RadioInput)]
    pub fn radio() -> Html {
        html! {
            <div>
                <input />
            </div>
        }
    }
}
