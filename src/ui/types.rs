use bevy::prelude::*;

use crate::state;

#[derive(Component)]
pub struct PlayButton(pub bool);

#[derive(Component)]
pub struct QuitButton(pub bool);

#[derive(Component)]
pub struct ReturnToMenuButton(pub bool);

#[derive(Component)]
pub struct CreditsButton(pub bool);

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct GhostCursor;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct FadeToBlack {
    duration: f32,
    pub remianing: f32,
    pub next_state: state::GameState,
}

impl FadeToBlack {
    pub fn new(duration: f32, next_state: state::GameState) -> Self {
        Self {
            duration,
            remianing: duration,
            next_state,
        }
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }
}

#[derive(Event)]
pub struct GameOverEvent {
    pub final_score: u64,
}

#[derive(Component)]
pub struct GlobalMusic;
