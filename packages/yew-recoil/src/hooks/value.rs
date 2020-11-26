pub fn use_recoil_value<R: Readable, T: AtomValue>(atom: &'static Atom<T>) -> Rc<T> {
    todo!()
    // // The setter doesn't change the hook's subscription to the recoil root
    // let (value, _) = use_recoil_state(atom);
    // value
}
