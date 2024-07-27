use bevy::{
    audio::{PlaybackMode, Volume},
    color::palettes::basic,
    prelude::*,
};

use crate::state;

use super::types;

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Credits), spawn_credits)
            .add_systems(
                Update,
                update_credits.run_if(in_state(state::GameState::Credits)),
            );
    }
}

pub fn spawn_credits(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            StateScoped(state::GameState::Credits),
        ))
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
                "a (first) game by seedse",
                TextStyle {
                    font_size: 25.0,
                    font: asset_server.load("ui/fonts/font.otf"),
                    color: Color::WHITE,
                },
            ));

            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "\nMusic by Bensound.com/royalty-free-music\n",
                    TextStyle {
                        font_size: 15.0,
                        font: asset_server.load("ui/fonts/font.otf"),
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "License code: DFSWP3QQ3VIFDGYR\n",
                    TextStyle {
                        font_size: 15.0,
                        font: asset_server.load("ui/fonts/font.otf"),
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
                            width: Val::Percent(18.0),
                            height: Val::Percent(7.0),
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
                        "Back to menu",
                        TextStyle {
                            font: asset_server.load("ui/fonts/font.otf"),
                            font_size: 20.0,
                            color: Color::from(basic::LIME),
                        },
                    ));
                });
        });
}

pub fn update_credits(
    mut commands: Commands,
    mut return_to_menu_query: Query<(
        &Interaction,
        &mut BorderColor,
        &mut types::ReturnToMenuButton,
    )>,
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
