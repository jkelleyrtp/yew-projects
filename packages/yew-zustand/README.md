
# ! not functional, dummy don't use this !



# Yew + Zustand = Yustand

Efficient state management for functional Yew applications

## How's it work?


The foundation of this crate is selectors + reducers. For a component in your Yew application to efficiently be updated, it must subscribe to some selection of the state provided via a Context component.

A component subscribing to a part of the App's state would look like:



```rust
#[derive(PartialEq, Debug, Clone)]
pub struct ZewStore {
    count: f32,
}

#[functional_component]
pub fn my_app() -> Html {
    let context = ZewStore { count: 32.0 };
    html! {
        <ContextProvider<ZewStore> context=context>
        
        </ContextProvider<ZewStore>>
    }
}


struct SelectCount {}
impl Selector for SelectCount {
    fn select(state: appstate) -> {
        state.get_in()
    }
}
#[selector]
fn select_count(state: AppState) {
    state.get_in()
}


#



#[functional_component]
pub fn display() -> Html {
    // currently a fake selector
    let count = use_selector::<SelectCount>();

    html! {
        <p>{"Value:"}{count}</p>
    }
}
