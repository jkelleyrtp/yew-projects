fn main() {}

pub struct GlobalStore {
    items: Vec<u32>,
}

pub enum StoreDispatch {}

static SELECT_NUMB: Selector<GlobalStore, u32> = selector(|store| store.items.get(0).unwrap());

struct selector<T, O>(fn(&T) -> &O);
type Selector<T, O> = selector<T, O>;
