pub mod credits;
pub mod cursor;
pub mod fade_to_black;
pub mod game_over;
pub mod loading;
pub mod menu;
pub mod pause;
pub mod score;
pub mod splash;
pub mod types;

use bevy::prelude::*;

pub struct ReCycleUiPlugin {
    confirmation_frames_target: usize,
}

impl ReCycleUiPlugin {
    pub fn new(confirmation_frames_target: usize) -> Self {
        Self {
            confirmation_frames_target,
        }
    }
}

impl Default for ReCycleUiPlugin {
    fn default() -> Self {
        Self::new(5)
    }
}

impl Plugin for ReCycleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loading::LoadingScreenPlugin::new(self.confirmation_frames_target),
            splash::SplashPlugin,
            menu::MenuPlugin,
            cursor::CursorPlugin,
            score::ScoreUiPlugin,
            pause::PausePlugin,
            fade_to_black::FadeToBlackPlugin,
            game_over::GameOverPlugin,
            credits::CreditsPlugin,
        ));
    }
}
