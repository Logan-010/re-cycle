use super::types;
use crate::state;
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use rand::{thread_rng, Rng};

pub fn spawn_trash_event(
    mut commands: Commands,
    mut event_reader: EventReader<types::TrashSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    for _ in event_reader.read() {
        let mut rng = thread_rng();
        let x = loop {
            let candidate_x = rng.gen_range(-70.0..70.0);
            if !(-8.0..=9.0).contains(&candidate_x) {
                break candidate_x;
            }
        };

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/game/spawn.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(0.75),
                ..Default::default()
            },
        });

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, 50.0, 1.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                },
                texture: asset_server
                    .load(format!("sprites/trash/trash_{}.png", rng.gen_range(1..=9))),
                ..Default::default()
            },
            types::Trash { points: Vec::new() },
            StateScoped(state::GameState::InGame),
        ));
    }
}
