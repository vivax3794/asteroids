use bevy::prelude::*;

use super::{
    asteroids::AsteroidsPlugin, bullets::BulletPlugin, physics_engine::MovementPlugin,
    score::ScorePlugin, ship::PlayerPlugin,
};

/// main game plugin
pub struct GamePlugin;

pub struct GameScore(i32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MovementPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AsteroidsPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(ScorePlugin);
    }
}
