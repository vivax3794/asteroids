#![windows_subsystem = "windows"]

use bevy;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_prototype_lyon::prelude::*;

use astroids::game;

/// Create app
fn main() {
    let mut app = App::new();
    app.insert_resource(WinitSettings::game())
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: 1000.,
            height: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins);

    app.add_plugin(ShapePlugin)
        .add_plugin(game::GamePlugin)
        // .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_camera);

    app.run();
}

/// Add camera to world
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
