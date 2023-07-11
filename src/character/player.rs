use std::collections::HashMap;

use super::{character::Character, characteristic::Characteristic};

pub struct Player<'a> {
    name: &'static str,
    characteristics: HashMap<&'a Characteristic, u8>,
}

impl Player<'_> {
    pub fn new(name: &'static str) -> Self {
        Player {
            name,
            characteristics: HashMap::new(),
        }
    }
}

impl<'a, 'b: 'a> Character<'b> for Player<'a> {
    fn add_characteristic(&mut self, characteristic: &'b Characteristic, value: u8) {
        self.characteristics.insert(characteristic, value);
    }

    fn get_characteristic_value(&self, characteristic: &Characteristic) -> u8 {
        *self.characteristics.get(characteristic).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use crate::character::{character::Character, characteristic::Characteristic};

    use super::Player;

    #[test]
    fn create_and_add_characteristic_to_player() {
        let mut player = Player::new("Robin");
        let characteristic = Characteristic::new("Strength");

        player.add_characteristic(&characteristic, 5);

        assert_eq!(player.get_characteristic_value(&characteristic), 5);
    }

    #[test]
    fn get_charactertic_without_adding_it_before() {
        let mut player = Player::new("Robin");
        let characteristic = Characteristic::new("Strength");

        assert_eq!(player.get_characteristic_value(&characteristic), 0);
    }
}
