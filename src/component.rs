use bevy::prelude::*;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Obstacle {
    pub scored: bool,
}

#[derive(Component, Default)]
pub struct Scroll {
    pub width: f32,
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct FinalResult;

#[derive(Component)]
pub struct HighScore;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Waiting,
    Running,
}

#[derive(Resource, Default)]
pub struct GameData {
    pub score: u64,
    pub highest_score: u64,
}

pub struct GameOverEvent;
