use std::rc::Rc;
use uuid;

/// We use this set of constraints *everywhere* in the codebase, which gets very noisy
/// To make things simpler, we can just use a trait alias
pub trait AtomValue: Default + PartialEq + std::fmt::Debug {}
impl<T> AtomValue for T where T: Default + PartialEq + std::fmt::Debug {}

pub trait Readable {
    type Output: AtomValue;
    fn use_recoil_state(reader: &'static Self) -> Rc<Self::Output>;
}

pub use atoms::{atom, Atom, AtomBuilder};

/*


*/
mod atoms {

    use super::*;
    pub struct AtomBuilder<T: Default + PartialEq> {
        pub key: String,
        pub manual_init: Option<Box<dyn Fn() -> T>>,
        _never: std::marker::PhantomData<T>,
    }

    impl<T: Default + PartialEq> AtomBuilder<T> {
        pub fn new() -> Self {
            Self {
                key: uuid::Uuid::new_v4().to_string(),
                manual_init: None,
                _never: std::marker::PhantomData {},
            }
        }

        pub fn init<A: Fn() -> T + 'static>(&mut self, f: A) {
            self.manual_init = Some(Box::new(f));
        }

        pub fn set_key(&mut self, _key: &'static str) {}
    }

    pub struct atom<T: Default + PartialEq>(pub fn(&mut AtomBuilder<T>));
    pub type Atom<T: Default + PartialEq> = atom<T>;
}

/*


*/
mod selectors {
    use super::*;
    use std::{collections::HashMap, rc::Rc};

    use uuid::Uuid;

    use super::{Atom, AtomValue};

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
}
