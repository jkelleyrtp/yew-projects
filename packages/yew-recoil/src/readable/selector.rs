use std::{collections::HashMap, rc::Rc};

use uuid::Uuid;

use super::{atom::Atom, AtomValue};

pub struct SelectorApi {
    // graph: &'g mut HashMap<String, String>,
}

impl SelectorApi {
    pub fn get<T: AtomValue>(&mut self, _reader: &'static Atom<T>) -> &T {
        todo!()
    }
    pub fn set(&mut self) -> () {}
}

pub struct SelectorBuilder<'a, T: AtomValue + 'a> {
    key: String,
    getter_fn: Option<fn(&mut SelectorApi) -> T>,
    _never: std::marker::PhantomData<&'a T>,
}

impl<'a, T: AtomValue> SelectorBuilder<'a, T> {
    pub fn new() -> Self {
        Self {
            key: Uuid::new_v4().to_string(),
            getter_fn: None,
            _never: std::marker::PhantomData {},
        }
    }
    pub fn getter(&mut self, f: fn(&mut SelectorApi) -> T) -> &mut Self {
        self.getter_fn = Some(f);
        self
    }
}

pub struct selector<T: AtomValue>(pub fn(&mut SelectorBuilder<T>));
pub type Selector<T> = selector<T>;

// mod tests {
//     use super::*;
//     use crate::readable::atom::*;
//     static COUNT: Atom<u32> = atom(|_| {});

//     static SELECT_DOUBLE_COUNT: Selector<u32> = selector(|builder| {
//         builder.set_key("yomama");
//         builder.getter(|api| {
//             let c = api.get(&COUNT);
//             c * 2
//         });
//     });
// }
