use super::types;
use crate::score;
use avian2d::prelude::*;
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use rand::{thread_rng, Rng};

pub fn despawn_trash(
    mut commands: Commands,
    mut score: ResMut<score::PlayerScore>,
    sensor_query: Query<&CollidingEntities, With<types::TrashCanSensor>>,
    trash_query: Query<&types::Trash>,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut types::TimeRemaining>,
) {
    for colliding_entities in sensor_query.iter() {
        let mut rng = thread_rng();

        for &entity in colliding_entities.0.iter() {
            if trash_query.get(entity).is_ok() {
                commands.entity(entity).despawn_recursive();

                score.score += 1;

                commands.spawn(AudioBundle {
                    source: asset_server.load(format!(
                        "sounds/trash/trashcan_{}.ogg",
                        rng.gen_range(1..=4)
                    )),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.7),
                        ..Default::default()
                    },
                });

                for mut timer in timer_query.iter_mut() {
                    timer.remaining += 2.5 * (timer.multiplier * 0.7);
                    timer.multiplier += 0.05;
                    timer.spawn_every -= 0.04;
                }
            }
        }
    }
}
