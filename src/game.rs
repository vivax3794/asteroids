use bevy::prelude::*;

use super::{
    asteroids::AsteroidsPlugin, bullets::BulletPlugin, physics_engine::MovementPlugin,
    ship::PlayerPlugin,
};

/// main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MovementPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AsteroidsPlugin)
            .add_plugin(BulletPlugin);
    }
}
