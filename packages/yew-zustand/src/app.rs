// use crate::zew::use_selector;
use anyhow::Result;
use closure::closure;
use fc_macros::{callback_once, functional_component};
use std::rc::Rc;
use yew::{html, Callback, Html, Properties};
use yew_functional::{
    use_context, use_state, ContextProvider, FunctionComponent, FunctionProvider,
};

#[derive(PartialEq, Debug, Clone)]
pub struct ZewStore {
    count: f32,
}

#[functional_component]
pub fn my_app() -> Html {
    let context = ZewStore { count: 32.0 };
    html! {
        <ContextProvider<ZewStore> context=context>
            <div>
                <Display />
                <Incr />
            </div>
        </ContextProvider<ZewStore>>
    }
}

fn select_count(z: &ZewStore) -> f32 {
    z.count
}

#[functional_component]
pub fn display() -> Html {
    // currently a fake selector
    // let count = use_selector(select_count);
    // <p>{"Value:"}{count}</p>

    html! {
        <p>{"Value:"}</p>
    }
}

fn dispatch(z: &mut ZewStore) {}

#[functional_component]
pub fn incr() -> Html {
    // let incr = use_dispatch(|z: &mut ZewStore| z.count += 1.0);

    html! {
        <button >{"Incr"}</button>
    }
}

fn ideas() {}
