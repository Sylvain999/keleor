use super::player::Player;

pub struct Team {
    players: Vec<Player>
}

impl Team {
    pub fn new() -> Self {
        Team { players: Vec::new() }
    }

    pub fn add_player(&mut self, player:Player) {
        self.players.push(player);
    }

    pub fn get_number_players(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Team;
    use super::Player;
    
    #[test]
    pub fn create_a_new_team_of_players() {
        let mut team = Team::new();

        team.add_player(Player::new());
        team.add_player(Player::new());

        assert_eq!(team.get_number_players(), 2);        
    } 

}