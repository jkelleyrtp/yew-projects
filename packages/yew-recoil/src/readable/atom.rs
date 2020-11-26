use uuid;



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
