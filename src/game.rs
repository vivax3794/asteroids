use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    asteroids::AsteroidsPlugin, bullets::BulletPlugin, death_screen::DeathPlugin,
    mainmenu::MainmenuPlugin, physics_engine::MovementPlugin, score::ScorePlugin,
    ship::PlayerPlugin, GameState,
};

/// main game plugin
pub struct GamePlugin;

pub struct GameScore(i32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::MainMenu);
        app.add_plugin(MovementPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AsteroidsPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(MainmenuPlugin)
            .add_plugin(DeathPlugin);
    }
}
