use super::types;
use crate::{state, ui};
use bevy::{
    audio::{PlaybackMode, Volume},
    color::palettes::basic,
    prelude::*,
};
use bevy_pkv::PkvStore;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Menu), setup_menu)
            .add_systems(Update, update_menu.run_if(in_state(state::GameState::Menu)));
    }
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>, pkv: Res<PkvStore>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/game/nothingon.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new(0.2),
                mode: PlaybackMode::Loop,
                ..Default::default()
            },
        },
        types::GlobalMusic,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            StateScoped(state::GameState::Menu),
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("ui/sprites/menu.png"),
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                        ..Default::default()
                    },
                    style: Style {
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(20.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        image: UiImage {
                                            texture: asset_server.load("ui/sprites/icon.png"),
                                            ..Default::default()
                                        },
                                        style: Style {
                                            width: Val::Percent(8.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });

                                    parent.spawn(TextBundle::from_section(
                                        " (Re)cycle",
                                        TextStyle {
                                            font: asset_server.load("ui/fonts/title_font.TTF"),
                                            font_size: 45.0,
                                            color: Color::from(basic::GREEN),
                                        },
                                    ));
                                });

                            parent.spawn(TextBundle::from_section(
                                format!("Highscore: {}", pkv.get::<u64>("highscore").unwrap_or(0)),
                                TextStyle {
                                    font: asset_server.load("ui/fonts/title_font.TTF"),
                                    font_size: 28.0,
                                    color: Color::from(basic::GREEN),
                                },
                            ));

                            parent
                                .spawn((
                                    ButtonBundle {
                                        border_color: BorderColor(Color::from(basic::GREEN)),
                                        border_radius: BorderRadius::all(Val::Px(3.0)),
                                        style: Style {
                                            width: Val::Percent(10.0),
                                            height: Val::Percent(5.0),
                                            border: UiRect::all(Val::Px(5.0)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Percent(1.0)),
                                            ..Default::default()
                                        },
                                        background_color: BackgroundColor(Color::from(
                                            basic::GREEN,
                                        )),
                                        ..Default::default()
                                    },
                                    types::PlayButton(false),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Play",
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
                                            width: Val::Percent(10.0),
                                            height: Val::Percent(5.0),
                                            border: UiRect::all(Val::Px(5.0)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Percent(1.0)),
                                            ..Default::default()
                                        },
                                        background_color: BackgroundColor(Color::from(
                                            basic::GREEN,
                                        )),
                                        ..Default::default()
                                    },
                                    types::CreditsButton(false),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Credits",
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
                                            width: Val::Percent(10.0),
                                            height: Val::Percent(5.0),
                                            border: UiRect::all(Val::Px(5.0)),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Percent(1.0)),
                                            ..Default::default()
                                        },
                                        background_color: BackgroundColor(Color::from(
                                            basic::GREEN,
                                        )),
                                        ..Default::default()
                                    },
                                    types::QuitButton(false),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Quit",
                                        TextStyle {
                                            font: asset_server.load("ui/fonts/font.otf"),
                                            font_size: 20.0,
                                            color: Color::from(basic::LIME),
                                        },
                                    ));
                                });
                        });
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn update_menu(
    mut commands: Commands,
    mut play_button_query: Query<
        (&Interaction, &mut BorderColor, &mut types::PlayButton),
        (Without<types::QuitButton>, Without<types::CreditsButton>),
    >,
    mut credits_button_query: Query<
        (&Interaction, &mut BorderColor, &mut types::CreditsButton),
        (Without<types::QuitButton>, Without<types::PlayButton>),
    >,
    mut quit_button_query: Query<
        (&Interaction, &mut BorderColor, &mut types::QuitButton),
        (Without<types::CreditsButton>, Without<types::PlayButton>),
    >,

    fade_to_black_query: Query<&ui::types::FadeToBlack>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut border_color, mut play_button) in play_button_query.iter_mut() {
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

                    commands.spawn(ui::types::FadeToBlack::new(0.5, state::GameState::InGame));
                }
            }
        }
    }

    for (interaction, mut border_color, mut credits_button) in credits_button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *border_color = BorderColor(Color::from(basic::GREEN));
                credits_button.0 = false;
            }
            Interaction::Hovered => {
                *border_color = BorderColor(Color::from(basic::LIME));

                if !credits_button.0 {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("ui/sounds/hover.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });
                    credits_button.0 = true;
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

                    commands.spawn(ui::types::FadeToBlack::new(0.5, state::GameState::Credits));
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

                    commands.spawn(ui::types::FadeToBlack::new(0.5, state::GameState::Quitting));
                }
            }
        }
    }
}
