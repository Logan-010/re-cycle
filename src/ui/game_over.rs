use super::types;
use crate::{score, state};
use bevy::{
    audio::{PlaybackMode, Volume},
    color::palettes::basic,
    prelude::*,
};
use bevy_pkv::PkvStore;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<types::GameOverEvent>()
            .add_systems(
                FixedUpdate,
                game_over_event.run_if(in_state(state::PauseState::Running)),
            )
            .add_systems(OnEnter(state::PauseState::GameOver), spawn_game_over_screen)
            .add_systems(
                OnExit(state::PauseState::GameOver),
                |mut audio_query: Query<&mut PlaybackSettings, With<types::GlobalMusic>>| {
                    for mut playback_settings in audio_query.iter_mut() {
                        playback_settings.volume = Volume::new(0.2);
                    }
                },
            )
            .add_systems(
                Update,
                update_game_over_screen.run_if(in_state(state::PauseState::GameOver)),
            );
    }
}

pub fn game_over_event(
    mut game_over_event_reader: EventReader<types::GameOverEvent>,
    mut pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<state::PauseState>>,
) {
    for event in game_over_event_reader.read() {
        if pkv.get::<u64>("highscore").unwrap_or(0) < event.final_score {
            pkv.set("highscore", &event.final_score)
                .unwrap_or_else(|_| error!("Failed to store high score!"));
        }

        next_state.set(state::PauseState::GameOver);
    }
}

pub fn spawn_game_over_screen(
    mut commands: Commands,
    mut score: ResMut<score::PlayerScore>,
    asset_server: Res<AssetServer>,
    mut audio_query: Query<&mut PlaybackSettings, With<types::GlobalMusic>>,
) {
    for mut playback_settings in audio_query.iter_mut() {
        playback_settings.volume = Volume::new(0.08);
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                z_index: ZIndex::Global(2),
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            StateScoped(state::PauseState::GameOver),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "Game over!\n",
                    TextStyle {
                        font: asset_server.load("ui/fonts/font.otf"),
                        font_size: 45.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    format!("Final score: {}", score.score),
                    TextStyle {
                        font: asset_server.load("ui/fonts/font.otf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
            ]));

            parent
                .spawn((
                    ButtonBundle {
                        border_color: BorderColor(Color::from(basic::GREEN)),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        style: Style {
                            width: Val::Percent(16.0),
                            height: Val::Percent(6.5),
                            border: UiRect::all(Val::Px(5.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::all(Val::Percent(1.0)),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::from(basic::GREEN)),
                        ..Default::default()
                    },
                    types::ReturnToMenuButton(false),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font: asset_server.load("ui/fonts/font.otf"),
                            font_size: 20.0,
                            color: Color::from(basic::LIME),
                        },
                    ));
                });
        });

    score.score = 0;
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/game/gameover.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.5),
            ..Default::default()
        },
    });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_game_over_screen(
    mut commands: Commands,
    mut return_to_menu_query: Query<
        (
            &Interaction,
            &mut BorderColor,
            &mut types::ReturnToMenuButton,
        ),
        Without<types::QuitButton>,
    >,

    fade_to_black_query: Query<&types::FadeToBlack>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut border_color, mut play_button) in return_to_menu_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *border_color = BorderColor(Color::from(basic::GREEN));
                play_button.0 = false;
            }
            Interaction::Hovered => {
                *border_color = BorderColor(Color::from(basic::LIME));

                if !play_button.0 {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("ui/sounds/hover.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });
                    play_button.0 = true;
                }
            }
            Interaction::Pressed => {
                if fade_to_black_query.iter().count() == 0 {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("ui/sounds/select.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });

                    commands.spawn(types::FadeToBlack::new(0.5, state::GameState::Menu));
                }
            }
        }
    }
}
