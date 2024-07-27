use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
    Credits,
    Quitting,
}

#[derive(Debug, Default, SubStates, Clone, PartialEq, Eq, Hash)]
#[source(GameState = GameState::InGame)]
pub enum PauseState {
    #[default]
    Loading,
    Running,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<PauseState>()
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<PauseState>()
            .add_systems(
                OnEnter(GameState::Quitting),
                |mut event_writer: EventWriter<AppExit>| {
                    event_writer.send(AppExit::Success);
                },
            );
    }
}
