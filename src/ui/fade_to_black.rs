use crate::state;

use super::types;
use bevy::prelude::*;

pub struct FadeToBlackPlugin;

impl Plugin for FadeToBlackPlugin {
    fn build(&self, app: &mut App) {
        app.observe(spawn_fade_to_black)
            .add_systems(Update, update_fade_to_black);
    }
}

pub fn spawn_fade_to_black(trigger: Trigger<OnAdd, types::FadeToBlack>, mut commands: Commands) {
    let entity = trigger.entity();

    commands.entity(entity).insert(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        z_index: ZIndex::Global(3),
        background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn update_fade_to_black(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut types::FadeToBlack, &mut BackgroundColor)>,
    mut next_state: ResMut<NextState<state::GameState>>,
) {
    for (entity, mut fade_to_black, mut background_color) in query.iter_mut() {
        if fade_to_black.remianing <= 0.0 {
            commands.entity(entity).despawn_recursive();
            next_state.set(fade_to_black.next_state.clone());
            return;
        } else {
            fade_to_black.remianing -= time.delta_seconds();

            let progress =
                (fade_to_black.duration() - fade_to_black.remianing) / fade_to_black.duration();

            background_color.0 = Color::srgba(0.0, 0.0, 0.0, progress);
        }
    }
}
