use bevy::prelude::*;

use super::AstroidSystemLabel;

/// Movement plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_entity.label(AstroidSystemLabel::Physics))
            .add_system(
                handle_forward_acceleration
                    .label(AstroidSystemLabel::Physics)
                    .before(move_entity),
            )
            .add_system(apply_friction.label(AstroidSystemLabel::Physics))
            .add_system(handle_screenwraping);
    }
}

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct ForwardAcceleration(pub f32);

#[derive(Component, Default, Debug)]
pub struct Friction(pub f32);

#[derive(Component, Default, Debug)]
pub struct HitBox(pub f32);

#[derive(Component, Default, Debug)]
pub struct ScreenWrap(pub f32);

fn move_entity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut trans, vel) in query.iter_mut() {
        trans.translation.x += vel.0.x * time.delta_seconds();
        trans.translation.y += vel.0.y * time.delta_seconds();
    }
}

fn handle_forward_acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Transform, &ForwardAcceleration)>,
) {
    for (mut vel, trans, acc) in query.iter_mut() {
        vel.0 += trans.up().truncate() * acc.0 * time.delta_seconds();
    }
}

fn apply_friction(time: Res<Time>, mut query: Query<(&mut Velocity, &Friction)>) {
    for (mut vel, Friction(friction)) in query.iter_mut() {
        vel.0 = vel.0.lerp(Vec2::ZERO, friction * time.delta_seconds());
    }
}

fn handle_screenwraping(windows: Res<Windows>, mut query: Query<(&mut Transform, &ScreenWrap)>) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let left_edge = -width / 2.0;
    let right_edge = width / 2.0;
    let top_edge = height / 2.0;
    let bottom_edge = -height / 2.0;

    for (mut trans, ScreenWrap(treshhold)) in query.iter_mut() {
        if trans.translation.x + treshhold <= left_edge {
            trans.translation.x = right_edge + treshhold;
        } else if trans.translation.x - treshhold >= right_edge {
            trans.translation.x = left_edge - treshhold;
        }

        if trans.translation.y + treshhold <= bottom_edge {
            trans.translation.y = top_edge + treshhold;
        } else if trans.translation.y - treshhold >= top_edge {
            trans.translation.y = bottom_edge - treshhold;
        }
    }
}
