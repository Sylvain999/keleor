use super::characteristic::Characteristic;

pub trait Character<'a> {
    fn add_characteristic(&mut self, characteristic: &'a Characteristic, value: u8);

    fn get_characteristic_value(&self, characteristic: &Characteristic) -> u8;
}
