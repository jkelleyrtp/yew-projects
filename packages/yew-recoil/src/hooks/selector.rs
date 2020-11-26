use std::rc::Rc;

use yew_functional::{use_hook, Hook};

use crate::readable::{atom::Atom, AtomValue};

pub fn use_selector<T: AtomValue + 'static>(reader: &'static Atom<T>) {
    todo!()
    // use_hook((), || UseSelectorState { atom });
}

struct UseSelectorState<T: AtomValue + 'static> {
    atom: &'static Atom<T>,
}

impl<T: AtomValue + 'static> Hook for UseSelectorState<T> {
    type Args = ();
    type Output = Rc<T>;

    fn runner(&mut self, _: Self::Args, updater: yew_functional::HookUpdater) -> Self::Output {
        todo!()
    }
    fn tear_down(&mut self) {}
}
