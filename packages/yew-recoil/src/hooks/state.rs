use super::find_context_provider_scope;
use crate::{
    prelude::RecoilRoot,
    readable::{atom::Atom, AtomValue},
};
use std::rc::Rc;
use yew::html::{AnyScope, Scope};
use yew_functional::{get_current_scope, use_hook, Hook};

// (return value, setter)
type UseRecoilStateOutput<T> = (Rc<T>, Rc<dyn Fn(T)>);

pub fn use_recoil_state<T: AtomValue + 'static>(atom: &'static Atom<T>) -> UseRecoilStateOutput<T> {
    // We need to get the scope here and not in the initializer
    // The initializer is ran under the use_scope context, and will try to mutably borrow the CURRENT_HOOK
    // Instead, we get the scope at callsite, and the pass the reference to the initializer via args
    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside function components");

    use_hook::<UseRecoilState<T>, _>((), move || {
        // Find ther RecoilRoot's scope
        let recoil_scope =
            find_context_provider_scope(&scope).expect("Unable to find RecoilRoot in scope");

        // Register this atom
        // The recoil scope will add it to its dataflow graph, evalute the selectors, and then give us a value back
        // If the atom is already registered, then we get the current value
        let current_value = recoil_scope
            .get_component()
            .expect("Recoil Root needs to be present for use_recoil_state to work")
            .register_atom(atom);

        UseRecoilState {
            recoil_scope,
            callback: None,
            atom,
            hook_id: uuid::Uuid::new_v4(),
            current_value,
        }
    })
}

struct UseRecoilState<T: AtomValue + 'static> {
    recoil_scope: Scope<RecoilRoot>,
    callback: Option<Rc<Box<dyn Fn(UpdateAction<T>)>>>,
    atom: &'static Atom<T>,
    hook_id: uuid::Uuid,
    pub current_value: Rc<T>,
}

#[derive(Debug)]
// Hooks can be updated from a variety of methods
pub enum UpdateAction<T: AtomValue> {
    // The user is putting in a new value and we need to update the registered atom value
    UpdateValue(T),
    // The atom is being modified from another atom and we're getting notified of the changes
    Regenerate(Rc<T>),
}

impl<T: AtomValue> Hook for UseRecoilState<T> {
    // (atom, callsite scope)
    type Args = ();
    // type Args = (&'static Atom<T>, AnyScope);
    type Output = UseRecoilStateOutput<T>;

    /// Is called when the component is unmounted and the hook needs to clean up
    fn tear_down(&mut self) {
        if let Some(callback) = self.callback.take() {
            drop(callback);
        }
    }

    /// Called every time the hook is ran
    fn runner(&mut self, _: Self::Args, updater: yew_functional::HookUpdater) -> Self::Output {
        // [1]
        // Create a dual-purpose callback that can be used for subscriptions
        // We do this because hook_callback can only really be called once because it's a special closure
        let callback: Rc<Box<dyn Fn(UpdateAction<T>)>> = Rc::new(Box::new(move |action| {
            // Update the hook state, re-rendering if the value actually changed
            // Notice that "setting" doesn't actually cause a re-render
            // Re-renders are only caused when the recoil root says we changed
            updater.callback(move |state: &mut Self| match action {
                // Updating should not cause a re-render
                // The update *should* propogate to a Regenerate, and the render will be complete
                UpdateAction::UpdateValue(new_value) => {
                    state
                        .recoil_scope
                        .get_component()
                        .map(|comp| comp.update_atom(state.atom, new_value));

                    // Don't re-render.
                    // The recoil root will process the update and call a re-render
                    // The recoil-root holds an Rc of our update_state function, only valid for this call
                    false
                }

                // Regenerating *should* be managed by the recoil root
                UpdateAction::Regenerate(new_value) => {
                    state.current_value = new_value;
                    true
                }
            });
        }));

        // Generate a setter
        let masterfn = callback.clone();
        let setter = Rc::new(move |new_val| {
            masterfn(UpdateAction::UpdateValue(new_val));
        });

        &self.recoil_scope.get_component().map(|comp| {
            comp.subscribe_consumer(self.atom, callback.clone(), self.hook_id.clone());
        });

        let atom_value = self.current_value.clone();

        (atom_value, setter)
    }
}
