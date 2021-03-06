use bevy::prelude::*;

/// Label for grouping systems
#[derive(SystemLabel, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AstroidSystemLabel {
    Input,
    Physics,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    MainMenu,
    Gameplay,
    DeadScreen,
}
