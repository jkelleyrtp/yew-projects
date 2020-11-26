use self::{atom::Atom, selector::Selector};

pub mod atom;
pub mod selector;

/// We use this set of constraints *everywhere* in the codebase, which gets very noisy
/// To make things simpler, we can just use a trait alias
pub trait AtomValue: Default + PartialEq + std::fmt::Debug {}
impl<T> AtomValue for T where T: Default + PartialEq + std::fmt::Debug {}

mod tests {
    use std::rc::Rc;

    use crate::{
        hooks::selector::use_selector,
        prelude::use_recoil_value,
        readable::atom::{atom, Atom},
        readable::selector::{selector, Selector},
    };

    #[derive(Default, PartialEq, Debug)]
    pub struct ExampleStruct {
        text: String,
        count: i32,
    }

    static COUNT: Atom<Vec<Rc<ExampleStruct>>> = atom(|_| {});

    static DOUBLE: Selector<Rc<ExampleStruct>> = selector(|s| {
        s.getter(|f| {
            f.get(&COUNT)
                .get(0)
                .and_then(|f| Some(f.clone()))
                .unwrap_or_else(|| Rc::new(ExampleStruct::default()))
        });
    });

    fn test() {
        let g = use_recoil_value(&DOUBLE);
    }
}
