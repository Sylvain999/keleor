use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
pub struct Characteristic {
    name: &'static str,
}

impl Characteristic {
    pub fn new(name: &'static str) -> Self {
        Characteristic { name }
    }
}
