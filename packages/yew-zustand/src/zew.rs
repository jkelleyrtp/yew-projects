use std::any::TypeId;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::{iter, mem};
use yew::html;
use yew::html::{AnyScope, Scope};
use yew::{Children, Component, ComponentLink, Html, Properties};
use yew_functional::*;

// pub fn use_selector<T: Clone + PartialEq + 'static, O>(narrow: impl Fn(&T) -> O) -> O {
//     let c = use_context::<T>().unwrap();
//     narrow(c.as_ref())
// }
// #[derive(Debug)]
// struct MyStruct {
//     value: f32,
// }

// fn modify_mut(value: &mut f32) {
//     *value += 1.0;
// }

// struct ZewStore {}
// enum ZewAction {
//     Incr,
//     Decr,
//     Set(f32),
// }

// fn dispatch(state: Selector<Rc<ZewStore>>, action: ZewAction) {
//     let a = |_: Selector<()>| {};
//     // Dispatching relies on tracing to
//     match action {
//         ZewAction::Set(val) => {
//             state
//                 .select(|_| {})
//                 .select(|_| {})
//                 .select(|_| {})
//                 .select(|_| {})
//                 .select(|_| {})
//                 .modify(|_| true);
//         }

//         ZewAction::Decr => {
//             state
//                 .select(|_| {})
//                 .select(a)
//                 .select(a)
//                 .select(a)
//                 .select(a)
//                 .modify(|_| false);
//         }

//         ZewAction::Incr => {
//             // state
//             //     .select(aabc123)
//             //     .select(aabc123)
//             //     .select(aabc123)
//             //     .modify(|_| false);
//         }
//     }
// }

// /// A composable tracing selector that updates a "higher power" when mutable/immutable references are taken
// struct Selector<O> {
//     out: O,
//     selectors: Vec<String>,
// }
// impl<O> Selector<O> {
//     fn select<J>(self, runner: impl Fn(Selector<O>) -> J) -> Selector<J> {
//         let out = runner(self.clo);
//         let mut selectors = self.selectors.clone();
//         selectors.push(format!("selector: {:#?}", selectors.len()));
//         Selector { out, selectors }
//     }

//     fn modify(self, runner: impl Fn(&mut O) -> bool) {}
// }

// // #[Selector]
// fn compound_selector(b: Selector<f32>) -> Selector<bool> {
//     b.select(|_| true)
// }

fn test() {
    // Redundant selectors
    // Selectors present a unqiue handle of a graph that can be both written to, and read from
    // If two selectors read the same value but only one is "traced," then there is no guarantee
    // components reading from the other selector will update.
    //
    // This shouldn't be an issue with proper composition
    // let b = Selector {
    //     out: 10_f32,
    //     selectors: vec![],
    // };

    // let a = b
    //     .select(|_| true)
    //     .select(|_| 10_f32)
    //     .select(compound_selector);

    // let c = use_selector(compound_selector);
}
