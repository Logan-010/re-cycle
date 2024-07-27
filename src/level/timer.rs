use super::types;
use crate::{score, state, ui};
use bevy::prelude::*;

pub fn spawn_timer(mut commands: Commands) {
    commands.spawn((
        types::TimeRemaining::default(),
        StateScoped(state::GameState::InGame),
    ));
}

pub fn update_timer(
    time: Res<Time>,
    mut timer_query: Query<&mut types::TimeRemaining>,
    mut game_over_event_writer: EventWriter<ui::types::GameOverEvent>,
    mut spawn_every_event_writer: EventWriter<types::TrashSpawnEvent>,
    score: Res<score::PlayerScore>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.remaining -= time.delta_seconds() * timer.multiplier;
        timer.last_spawn += time.delta_seconds();

        if timer.remaining <= 0.0 {
            game_over_event_writer.send(ui::types::GameOverEvent {
                final_score: score.score,
            });
        }

        if timer.last_spawn >= timer.spawn_every {
            spawn_every_event_writer.send(types::TrashSpawnEvent);
            timer.last_spawn = 0.0;
        }
    }
}
