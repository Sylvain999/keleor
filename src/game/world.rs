use crate::character::characteristic::Characteristic;

pub struct World {
    characteristics: Vec<Characteristic>,
}

impl World {
    fn new(characteristics: Vec<Characteristic>) -> Self {
        World { characteristics }
    }
}
