use std::sync::Arc;

use bevy;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

use crate::asteroids::{Asteroid, AsteroidBundle};
use crate::physics_engine::{ForwardAcceleration, HitBox};

use super::physics_engine;
use super::AstroidSystemLabel;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(
                handle_movement_inputs
                    .run_in_state(GameState::Gameplay)
                    .label(AstroidSystemLabel::Input)
                    .before(AstroidSystemLabel::Physics),
            )
            .add_system(
                handle_rotation_inputs
                    .run_in_state(GameState::Gameplay)
                    .label(AstroidSystemLabel::Input)
                    .before(AstroidSystemLabel::Physics),
            )
            .add_system(detect_defeat.run_in_state(GameState::Gameplay))
            .add_exit_system(GameState::DeadScreen, reset_player);
    }
}

fn setup_player(mut commands: Commands) {
    let line_shape = shapes::Line(Vec2::new(0., 10.), Vec2::new(0., 30.));

    Arc::new(
        commands
            .spawn_bundle(PlayerBundle::new())
            .with_children(|parent| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &line_shape,
                    DrawMode::Stroke(StrokeMode::color(Color::RED)),
                    Transform::default(),
                ));
            }),
    );
}

fn reset_player(mut query: Query<&mut Transform, With<Player>>) {
    let mut trans = query.single_mut();
    *trans = Transform::default();
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    _m: Player,
    _w: physics_engine::ScreenWrap,
    fric: physics_engine::Friction,
    vel: physics_engine::Velocity,
    acc: physics_engine::ForwardAcceleration,
    hit: physics_engine::HitBox,

    #[bundle]
    shape: ShapeBundle,
}

impl PlayerBundle {
    fn new() -> Self {
        let shape = shapes::RegularPolygon {
            sides: 3,
            feature: RegularPolygonFeature::Apothem(10.0),
            ..default()
        };

        PlayerBundle {
            _m: Player,
            _w: physics_engine::ScreenWrap(20.0),
            fric: physics_engine::Friction(0.7),
            vel: physics_engine::Velocity(Vec2::ZERO),
            acc: physics_engine::ForwardAcceleration(0.0),
            hit: physics_engine::HitBox(10.0),
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(Color::WHITE)),
                Transform::from_translation(Vec3::new(0., 0., 101.)),
            ),
        }
    }
}

fn handle_movement_inputs(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut physics_engine::ForwardAcceleration, With<Player>>,
) {
    let mut acc = query.single_mut();
    acc.0 = 0.0;
    if keyboard.pressed(KeyCode::W) {
        acc.0 += 200.0;
    }
    if keyboard.pressed(KeyCode::S) {
        acc.0 -= 150.0;
    }
}

fn handle_rotation_inputs(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut trans = query.single_mut();

    let mut angle = 0.0;
    if keyboard.pressed(KeyCode::D) {
        angle -= 3.14 * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        angle += 3.14 * time.delta_seconds();
    }

    if mouse.pressed(MouseButton::Right) {
        angle /= 2.0;
    }

    trans.rotate(Quat::from_axis_angle(Vec3::Z, angle))
}

fn detect_defeat(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &HitBox, &mut ForwardAcceleration), With<Player>>,
    asteroid_query: Query<(&Transform, &Asteroid), With<Asteroid>>,
) {
    let (player_trans, HitBox(player_hitbox), mut acc) = player_query.single_mut();

    for (astroid_trans, Asteroid { size: _, points }) in asteroid_query.iter() {
        if AsteroidBundle::detect_collison(astroid_trans, &points, player_trans, *player_hitbox) {
            commands.insert_resource(NextState(GameState::DeadScreen));
            acc.0 = 0.0;
            return;
        }
    }

    // println!("you alive");
}
