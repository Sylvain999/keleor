use crate::character::team::Team;

use super::world::World;

pub struct Game<'a> {
    world: World,
    player_team: Team<'a>,
}

impl<'a> Game<'a> {
    pub fn new(world: World, player_team: Team<'a>) -> Self {
        Game { world, player_team }
    }

    pub fn start(&self) {
        self.init();

        self.run();
    }

    fn init(&self) {}

    fn run(&self) {}
}
