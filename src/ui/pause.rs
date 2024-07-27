use super::types;
use crate::state;
use avian2d::prelude::*;
use bevy::{
    audio::{PlaybackMode, Volume},
    color::palettes::basic,
    prelude::*,
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                open_pause_menu.run_if(in_state(state::PauseState::Running)),
                (close_pause_menu, update_pause_menu).run_if(in_state(state::PauseState::Paused)),
            ),
        )
        .add_systems(
            OnEnter(state::PauseState::Paused),
            (spawn_pause_menu, |mut time: ResMut<Time<Physics>>| {
                time.pause()
            }),
        )
        .add_systems(
            OnExit(state::PauseState::Paused),
            |mut time: ResMut<Time<Physics>>| time.unpause(),
        );
    }
}

pub fn open_pause_menu(
    key: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<state::PauseState>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(state::PauseState::Paused);
    }
}

pub fn close_pause_menu(
    key: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<state::PauseState>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(state::PauseState::Running);
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.3)),
                ..Default::default()
            },
            StateScoped(state::PauseState::Paused),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Percent(70.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    border_radius: BorderRadius::all(Val::Px(15.0)),
                    border_color: BorderColor(Color::BLACK),

                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                border_color: BorderColor(Color::from(basic::GREEN)),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                style: Style {
                                    width: Val::Percent(25.0),
                                    height: Val::Percent(10.0),
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
                                "Return to menu",
                                TextStyle {
                                    font: asset_server.load("ui/fonts/font.otf"),
                                    font_size: 20.0,
                                    color: Color::from(basic::LIME),
                                },
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                border_color: BorderColor(Color::from(basic::GREEN)),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                style: Style {
                                    width: Val::Percent(25.0),
                                    height: Val::Percent(10.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::all(Val::Percent(1.0)),
                                    ..Default::default()
                                },
                                background_color: BackgroundColor(Color::from(basic::GREEN)),
                                ..Default::default()
                            },
                            types::QuitButton(false),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit to desktop",
                                TextStyle {
                                    font: asset_server.load("ui/fonts/font.otf"),
                                    font_size: 20.0,
                                    color: Color::from(basic::LIME),
                                },
                            ));
                        });
                });
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_pause_menu(
    mut commands: Commands,
    mut return_to_menu_query: Query<
        (
            &Interaction,
            &mut BorderColor,
            &mut types::ReturnToMenuButton,
        ),
        Without<types::QuitButton>,
    >,
    mut quit_button_query: Query<
        (&Interaction, &mut BorderColor, &mut types::QuitButton),
        Without<types::PlayButton>,
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

    for (interaction, mut border_color, mut quit_button) in quit_button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *border_color = BorderColor(Color::from(basic::GREEN));
                quit_button.0 = false;
            }
            Interaction::Hovered => {
                *border_color = BorderColor(Color::from(basic::LIME));

                if !quit_button.0 {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("ui/sounds/hover.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });

                    quit_button.0 = true;
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

                    commands.spawn(types::FadeToBlack::new(0.5, state::GameState::Quitting));
                }
            }
        }
    }
}
