use std::cell::RefCell;

use yew::prelude::*;
use yew_functional::{function_component, use_context};
use yew_recoil::hooks::{use_recoil_state, use_recoil_value};

#[function_component(Canvas)]
pub fn canvas() -> Html {
    let rows = 3;
    let cols = 3;
    let elements = (0..(rows * cols))
        .map(|v| {
            let posx = (v % cols) * 225 + 200;
            let posy = ((v as f32 / rows as f32).floor() as i32) * 200 + 200;
            html! {<CanvasElement starting_pos=(posx,posy) elid=v key={v}/>}
        })
        .collect::<Vec<_>>();

    html! {
        <div class="text-center flex-1">
            {elements}
        </div>
    }
}

#[derive(yew::Properties, Clone, Debug, PartialEq)]
pub struct ElementProps {
    starting_pos: (i32, i32),
    elid: i32,
}

#[function_component(CanvasElement)]
pub fn canvas_element(props: &ElementProps) -> Html {
    let (dragging, set_dragging) = yew_functional::use_state(|| false);

    let x0: i32 = props.starting_pos.0;
    let y0: i32 = props.starting_pos.1;
    let (position, set_position) = yew_functional::use_state(move || (x0, y0));

    let node = yew_functional::use_ref(|| NodeRef::default());

    let mouse_down = {
        let set_dragging = set_dragging.clone();
        Callback::from(move |event: yew::MouseEvent| {
            event.prevent_default();
            set_dragging(true);
        })
    };

    let mouse_up = {
        let set_dragging = set_dragging.clone();
        Callback::from(move |event: yew::MouseEvent| {
            event.prevent_default();
            set_dragging(false);
        })
    };

    let mouse_drag = {
        let node = node.clone();
        Callback::from(move |event: yew::MouseEvent| {
            if *dragging {
                event.prevent_default();
                let x = event.client_x();
                let y = event.client_y();
                set_position((x - 40, y - 20));
            }
        })
    };

    let val = use_recoil_value(&crate::state::COUNTER);

    let (title, set_title) = use_recoil_state(&crate::state::TITLE);
    let oninput = Callback::from(move |e: InputData| set_title(e.value));

    html! {
        <div
            class="border-2 absolute cursor-move border-black-100"
            style=format!("left: {}px; top: {}px;", position.0, position.1)
            ref=node.borrow().clone()
        >
            <div
                class="bg-blue-200 p-2"
                onmousedown=mouse_down
                onmouseup=mouse_up
                onmousemove=mouse_drag
            >
            {"Click here to move"}</div>
            <p>{"Move"}</p>

            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                </div>
                <input
                    type="text"
                    id="price"
                    class="focus:ring-indigo-500 focus:border-indigo-500 block pl-1 pr-6 sm:text-sm border-gray-300 rounded-md"
                    placeholder="enter text"
                    value=&title
                    oninput=oninput
                />
                <div class="absolute inset-y-0 right-0 flex items-center">
                <label for="currency" class="sr-only">{"Currency"}</label>
            </div>
            <p class="m-1">{format!("Divid: {}", props.elid)}</p>
            <p class="m-1">{format!("Count: {}", val)}</p>
        </div>
    }
}
