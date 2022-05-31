use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

/// main game plugin
pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::DeadScreen, create_death_menu)
            .add_system(handle_button_events.run_in_state(GameState::DeadScreen));
    }
}

#[derive(Component)]
struct DeathButton;

fn create_death_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Restart",
                    TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 20.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(DeathButton);
}

fn handle_button_events(
    mut commands: Commands,
    mut int_query: Query<
        (&Interaction, &mut UiColor, Entity),
        (Changed<Interaction>, With<DeathButton>),
    >,
) {
    for (interaction, mut color, entity) in int_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = Color::rgb(0.5, 0.5, 0.2).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
            Interaction::Clicked => {
                commands.entity(entity).despawn_recursive();
                commands.insert_resource(NextState(GameState::Gameplay))
            }
        }
    }
}
