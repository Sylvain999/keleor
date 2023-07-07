use rand::Rng;

pub struct Dice {
    nb_faces: u8,
}

impl Dice {
    ///
    /// # Example
    ///
    /// ```
    /// let dice = Dice::new(6);
    /// ```
    pub fn new(nb_faces: u8) -> Dice {
        Dice { nb_faces: nb_faces }
    }

    ///
    /// # Example
    ///
    /// ```
    /// let dice = Dice::new(6);
    /// let random = dice.get_new_value();
    /// ```
    pub fn get_new_value(&self) -> u8 {
        rand::thread_rng().gen_range(1..=self.nb_faces)
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;

    #[test]
    fn create_dice_and_get_random_value() {
        let dice = Dice::new(6);
        assert!(dice.get_new_value() <= 6);
    }
}
