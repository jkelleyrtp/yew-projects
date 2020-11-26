// Naming this file use_context could be confusing. Not least to the IDE.
use crate::{
    hooks::state::UpdateAction,
    readable::{atom::Atom, atom::AtomBuilder, AtomValue},
};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;
use yew::html;

use yew::{Children, Component, ComponentLink, Html, Properties};

pub type ConsumerCallback<T> = Box<dyn Fn(UpdateAction<T>)>;
pub type OpaqueConsumerCallback = Box<dyn Any>;

struct Consumer {
    callback: OpaqueConsumerCallback,
}

pub struct RecoilRoot {
    /// Children components that get rendered by Yew
    pub children: Children,

    // Map the reference of the static atom to a list of consumers
    /// Any component currently subscribed to the root
    /// consumer callbacks can only be Weak<ConsumerCallback<T>>
    /// This is enforced at runtime, however, so be careful
    consumers: RefCell<HashMap<u8, HashMap<Uuid, Consumer>>>,
}

impl RecoilRoot {
    /// This is run by hooks when they register as a listener for a given atom
    /// Their ID is stored in the `atom_consumers` map which lets us know which consumers to update
    /// When the hook is dropped, they are unregistered from this map
    ///
    /// If the Atom they're registering as a listener for doesn't exist, then the atom
    /// is initialized. Otherwise, the most recent value of the atom is returned
    pub fn register_atom<T: AtomValue>(&self, atom: &'static Atom<T>) -> Rc<T> {
        // Run the atom's initialization
        let mut b = AtomBuilder::<T>::new();
        atom.0(&mut b);

        // Get the raw static reference of the atom
        let atom_ptr = get_atom_raw_ref(atom);
        let mut consumers = self.consumers.borrow_mut();
        if !consumers.contains_key(&atom_ptr) {
            consumers.insert(atom_ptr, HashMap::new());
        }

        // Use the init override if it exists
        if let Some(initer) = b.manual_init {
            Rc::new(initer())
        } else {
            Rc::new(T::default())
        }
    }

    // Called when consumers drop their subscription
    pub fn subscribe_consumer<T: AtomValue>(
        &self,
        atom: &'static Atom<T>,
        updater: Rc<Box<dyn Fn(UpdateAction<T>)>>,
        hook_id: uuid::Uuid,
    ) {
        // Get the raw static reference of the atom
        let atom_ptr = get_atom_raw_ref(atom);

        // Subcribe this hook_id to this atom
        let mut consumers = self.consumers.borrow_mut();
        let hooks = consumers
            .get_mut(&atom_ptr)
            .expect("Atom must be initialized before being subscribed to");

        // Coerce into any by wrapping the updater in a box
        // (yes it's some weird indirection)
        let any_updater: OpaqueConsumerCallback = Box::new(updater);
        let consumer = Consumer {
            callback: any_updater,
        };

        // Insert into the map, booting out the old consumer
        // TODO @Jon, make the "Consumer" more efficient, patching its update mechanism in-place
        hooks.insert(hook_id, consumer);
    }

    pub fn drop_consumer<T: AtomValue>(&self, atom: &'static Atom<T>, hook_id: &uuid::Uuid) {
        let mut consumers = self.consumers.borrow_mut();
        let atom_ptr = get_atom_raw_ref(atom);
        let atoms = consumers.get_mut(&atom_ptr);

        if let Some(consumers) = atoms {
            let entry = consumers.remove_entry(&hook_id);
            if let Some(_) = entry {
                log::debug!("successfully unsubscribed listener");
            } else {
                log::debug!("Failure to unsubscribe");
            }
        } else {
            log::debug!("Strange error, atoms should be registed if the consumer is being dropped");
        }
    }

    pub fn update_atom<T: AtomValue>(&self, atom: &'static Atom<T>, new_val: T) {
        // Get the raw static reference of the atom
        let atom_ptr = get_atom_raw_ref(atom);

        let mut consumers = self.consumers.borrow_mut();
        let hooks = consumers
            .get_mut(&atom_ptr)
            .expect("Atom needs to be registered before trying to update it");

        let new_val = Rc::new(new_val);
        for hook in hooks.values_mut() {
            let callback: &mut Rc<ConsumerCallback<T>> = hook
                .callback
                .downcast_mut::<_>()
                .expect("Wrong type of atom stored, internal error");

            callback(UpdateAction::Regenerate(new_val.clone()));
        }
    }
}

/// Basic properties for the recoil root
#[derive(Clone, PartialEq, Properties)]
pub struct RecoilContextProps {
    pub children: Children,
}

impl Component for RecoilRoot {
    type Message = ();
    type Properties = RecoilContextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // TODO @Jon
        // setup the initializers here beyond the consumers/children/readables
        Self {
            consumers: RefCell::new(HashMap::new()),
            children: props.children,
        }
    }

    /// The link cannot update us, so nbd
    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    /// The RecoilRoot should only ever re-render if the children change
    /// It is not possible to externally drive the recoil state
    ///
    /// TODO @Jon https://recoiljs.org/docs/api-reference/core/RecoilRoot
    /// Setup an initializer for hydration
    fn change(&mut self, props: Self::Properties) -> bool {
        if self.children == props.children {
            false
        } else {
            self.children = props.children;
            true
        }
    }

    /// Always just render out the children, nothing special
    fn view(&self) -> Html {
        html! { <>{ self.children.clone() }</> }
    }
}

fn get_atom_raw_ref<T: AtomValue>(atom: &'static Atom<T>) -> u8 {
    let atom_ptr = atom as *const _;
    let atom_ptr = atom_ptr as *const u8;
    atom_ptr as u8
}
