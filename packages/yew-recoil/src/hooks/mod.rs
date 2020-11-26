use crate::prelude::RecoilRoot;
use std::any::TypeId;
use std::iter;
use yew::html::{AnyScope, Scope};

pub mod selector;
pub(crate) mod state;
pub mod value;

pub(crate) fn find_context_provider_scope(scope: &AnyScope) -> Option<Scope<RecoilRoot>> {
    let expected_type_id = TypeId::of::<RecoilRoot>();
    iter::successors(Some(scope), |scope| scope.get_parent())
        .filter(|scope| scope.get_type_id() == &expected_type_id)
        .cloned()
        .map(AnyScope::downcast::<RecoilRoot>)
        .next()
}

pub(crate) fn with_provider_component<T, F, R>(
    provider_scope: &Option<Scope<RecoilRoot>>,
    f: F,
) -> Option<R>
where
    T: Clone + PartialEq,
    F: FnOnce(&RecoilRoot) -> R,
{
    provider_scope
        .as_ref()
        .and_then(|scope| scope.get_component().map(|comp| f(&*comp)))
}

pub use self::state::use_recoil_state;
pub use value::use_recoil_value;
