use bevy;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::asteroids::AsteroidBundle;
use crate::score::Score;
use crate::ship::Player;
use crate::{asteroids::Asteroid, labels::AstroidSystemLabel};

use super::physics_engine::{HitBox, Velocity};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_bullets.label(AstroidSystemLabel::Input))
            .add_system(handle_despawning_bullets)
            .add_system(detect_collison);
    }
}

#[derive(Component, Debug)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    _m: Bullet,

    vel: Velocity,
    hit: HitBox,

    #[bundle]
    shape: ShapeBundle,
}

impl BulletBundle {
    pub fn new(pos: Vec3, direction: Vec2) -> Self {
        let shape = shapes::Circle {
            radius: 5.0,
            center: Vec2::ZERO,
        };

        BulletBundle {
            _m: Bullet,
            vel: Velocity(direction * 400.0),
            hit: HitBox(5.0),
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(Color::YELLOW)),
                Transform::from_translation(pos),
            ),
        }
    }
}

fn shoot_bullets(
    mut commands: Commands,
    click_events: Res<Input<MouseButton>>,
    query: Query<&Transform, With<Player>>,
) {
    if click_events.just_pressed(MouseButton::Left) {
        let trans = query.single();
        let pos = trans.translation;
        let direction = trans.up().truncate();
        commands.spawn_bundle(BulletBundle::new(pos, direction));
    }
}

fn handle_despawning_bullets(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform, &HitBox), With<Bullet>>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let left_edge = -width / 2.0;
    let right_edge = width / 2.0;
    let top_edge = height / 2.0;
    let bottom_edge = -height / 2.0;

    for (entity, trans, HitBox(hitbox)) in query.iter() {
        if trans.translation.x + hitbox <= left_edge
            || trans.translation.x - hitbox >= right_edge
            || trans.translation.y + hitbox <= bottom_edge
            || trans.translation.y - hitbox >= top_edge
        {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collison(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform, &HitBox), With<Bullet>>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
) {
    for (bullet_entity, bullet_pos, HitBox(bullet_hitbox)) in bullet_query.iter() {
        for (
            ast_entity,
            ast_pos,
            Asteroid {
                size: ast_size,
                points,
            },
        ) in asteroid_query.iter()
        {
            if AsteroidBundle::detect_collison(ast_pos, &points, bullet_pos, *bullet_hitbox) {
                commands.entity(bullet_entity).despawn();
                AsteroidBundle::break_apart(&mut commands, ast_entity, *ast_size, ast_pos);
                score.increment(*ast_size as i32);
                break;
            }
        }
    }
}
