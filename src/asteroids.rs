use bevy;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use super::physics_engine::{ScreenWrap, Velocity};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_asteroids);
    }
}

#[derive(Component, Debug)]
pub struct Asteroid {
    pub size: f32,
    pub points: Vec<Vec2>,
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    _m: Asteroid,
    _w: ScreenWrap,
    vel: Velocity,

    #[bundle]
    shape: ShapeBundle,
}

fn vector_from_angle(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

impl AsteroidBundle {
    pub fn new(trans: Transform, vel: Vec2, size: f32) -> Self {
        // generate points for the shape
        let mut points: Vec<Vec2> = Vec::new();
        let amount_of_points = (3.14 * size * 2.0 / 30.0) as i32;
        let roughnes = size / 2.0;

        let mut max_length: f32 = 0.0;
        for i in 0..amount_of_points {
            let angle = (i as f32) * 6.28 / (amount_of_points as f32);
            let unit_vector = vector_from_angle(angle);
            let length = size + rand::thread_rng().gen_range(-roughnes..roughnes);
            points.push(unit_vector * length);

            max_length = max_length.max(length);
        }

        // create shape
        let shape = shapes::Polygon {
            points: points.clone(),
            closed: true,
        };

        AsteroidBundle {
            _m: Asteroid { size, points },
            _w: ScreenWrap(max_length),
            vel: Velocity(vel),
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Stroke(StrokeMode::color(Color::ORANGE_RED)),
                trans,
            ),
        }
    }

    pub fn new_random((width, height): (f32, f32), size: f32) -> Self {
        let mut rng = rand::thread_rng();
        let trans = Transform::from_translation(Vec3::new(
            rng.gen_range(-width / 2.0..width / 2.0),
            rng.gen_range(-height / 2.0..height / 2.0),
            0.03,
        ));

        let vel = vector_from_angle(rng.gen_range(0.0..6.28)) * rng.gen_range(50.0..150.0);

        AsteroidBundle::new(trans, vel, size)
    }

    pub fn break_apart(commands: &mut Commands, entity: Entity, size: f32, pos: &Transform) {
        commands.entity(entity).despawn();

        if size >= 30.0 {
            let mut rng = rand::thread_rng();

            for _ in 0..rand::thread_rng().gen_range(2..4) {
                commands.spawn_bundle(AsteroidBundle::new(
                    pos.clone(),
                    vector_from_angle(rng.gen_range(0.0..6.28)) * rng.gen_range(50.0..150.0),
                    size / 2.0,
                ));
            }
        }
    }

    pub fn detect_collison(
        ast_trans: &Transform,
        points: &Vec<Vec2>,
        object_trans: &Transform,
        object_hitbox: f32,
    ) -> bool {
        let matrix = ast_trans.compute_matrix();

        for i in 0..points.len() {
            let pa = points[i];
            let pb = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };

            let pa = matrix.transform_point3(pa.extend(0.0)).truncate();
            let pb = matrix.transform_point3(pb.extend(0.0)).truncate();
            let pc = object_trans.translation.truncate();

            let distance_to_extended_line =
                f32::abs((pb.x - pa.x) * (pa.y - pc.y) - (pa.x - pc.x) * (pb.y - pa.y))
                    / pa.distance(pb);

            let min_dist = if Vec2::dot(pa - pc, pa - pb) > 0.0 && Vec2::dot(pb - pc, pb - pa) > 0.0
            {
                distance_to_extended_line
            } else {
                f32::min(pc.distance(pa), pc.distance(pb))
            };
            let max_dist = f32::max(pc.distance(pa), pc.distance(pb));

            if min_dist <= object_hitbox && max_dist >= object_hitbox {
                return true;
            }
        }

        return false;
    }
}

fn generate_asteroids(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    for _i in 0..5 {
        commands.spawn_bundle(AsteroidBundle::new_random(
            (width, height),
            rand::thread_rng().gen_range(60.0..90.0),
        ));
    }
}
