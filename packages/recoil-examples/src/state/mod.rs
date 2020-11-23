use recoil_yew::readable::atom::{atom, Atom};

pub static COUNTER: Atom<usize> = atom(|_| {});
pub static TITLE: Atom<String> = atom(|b| b.init(|| "Hello World!".to_string()));
