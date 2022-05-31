/// Main game plugin
pub mod game;

pub mod labels;
pub use labels::{AstroidSystemLabel, GameState};

pub mod asteroids;
pub mod bullets;
pub mod death_screen;
pub mod mainmenu;
pub mod physics_engine;
pub mod score;
pub mod ship;
