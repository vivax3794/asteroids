use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::new())
            .add_startup_system(create_score_text)
            .add_system(update_score.run_in_state(GameState::Gameplay))
            .add_enter_system(GameState::Gameplay, reset_score);
    }
}

pub struct Score(i32);

impl Score {
    fn new() -> Self {
        Score(0)
    }

    pub fn increment(&mut self, amount: i32) {
        self.0 += amount;
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
struct ScoreText;

fn create_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        color: Color::WHITE,
        font_size: 20.0,
        font: asset_server.load("fonts/font.ttf"),
    };

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "score:".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            },
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let mut score_text = query.single_mut();
        score_text.sections[1].value = format!("{}", score.value());
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}
