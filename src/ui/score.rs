use super::types;
use crate::{level, score, state};
use bevy::{color::palettes::basic, prelude::*};

pub struct ScoreUiPlugin;

impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::InGame), spawn_score_ui)
            .add_systems(
                Update,
                update_score_ui.run_if(in_state(state::GameState::InGame)),
            );
    }
}

pub fn spawn_score_ui(
    mut commands: Commands,
    score: Res<score::PlayerScore>,
    asset_server: Res<AssetServer>,
    time_query: Query<&level::types::TimeRemaining>,
) {
    let time = match time_query.get_single() {
        Ok(v) => v.remaining,
        Err(_) => level::types::TimeRemaining::default().remaining,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    margin: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        format!("Score: {}\n", score.score),
                        TextStyle {
                            font: asset_server.load("ui/fonts/font.otf"),
                            font_size: 35.0,
                            color: Color::from(basic::MAROON),
                        },
                    ),
                    TextSection::new(
                        format!("Time: {}", time),
                        TextStyle {
                            font: asset_server.load("ui/fonts/font.otf"),
                            font_size: 35.0,
                            color: Color::from(basic::MAROON),
                        },
                    ),
                ]),
                types::ScoreText,
            ));
        });
}

pub fn update_score_ui(
    mut text_query: Query<&mut Text, With<types::ScoreText>>,
    score: Res<score::PlayerScore>,
    time_query: Query<&level::types::TimeRemaining>,
) {
    for mut text in text_query.iter_mut() {
        let time = match time_query.get_single() {
            Ok(v) => v.remaining,
            Err(_) => level::types::TimeRemaining::default().remaining,
        };

        text.sections[0].value = format!("Score: {}\n", score.score);
        text.sections[1].value = format!("Time: {:.0}", time);
    }
}
