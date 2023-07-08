use super::{character::Character, player::Player};

pub struct Team<'a> {
    players: Vec<Box<dyn Character<'a>>>,
}

impl Team<'static> {
    pub fn new() -> Self {
        Team {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player<'static>) {
        self.players.push(Box::new(player));
    }

    pub fn get_number_players(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Player;
    use super::Team;

    #[test]
    pub fn create_a_new_team_of_players() {
        let mut team = Team::new();

        team.add_player(Player::new("Robin"));
        team.add_player(Player::new("Batman"));

        assert_eq!(team.get_number_players(), 2);
    }
}
