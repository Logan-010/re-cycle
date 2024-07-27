pub mod main_level;
pub mod timer;
pub mod trash;
pub mod trash_can;
pub mod trash_spawner;
pub mod types;

use crate::{state, ui::loading};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<types::TrashGizmoGroup>()
            .add_event::<types::TrashSpawnEvent>()
            .add_systems(
                OnEnter(state::GameState::InGame),
                (
                    main_level::spawn_level,
                    timer::spawn_timer,
                    |mut loading_data: ResMut<loading::LoadingData>,
                     asset_server: Res<AssetServer>| {
                        loading_data
                            .assets
                            .push(asset_server.load_untyped("sprites/level/level.png").into());
                        loading_data.assets.push(
                            asset_server
                                .load_untyped("sprites/misc/trash_can.png")
                                .into(),
                        );

                        for num in 1..=9 {
                            loading_data.assets.push(
                                asset_server
                                    .load_untyped(format!("sprites/trash/trash_{}.png", num))
                                    .into(),
                            );
                        }

                        loading_data
                            .assets
                            .push(asset_server.load_untyped("sounds/game/pickup.ogg").into());
                        loading_data
                            .assets
                            .push(asset_server.load_untyped("sounds/game/letgo.ogg").into());
                        loading_data
                            .assets
                            .push(asset_server.load_untyped("sounds/game/hit.ogg").into());
                        loading_data
                            .assets
                            .push(asset_server.load_untyped("sounds/game/gameover.ogg").into());

                        for num in 1..=4 {
                            loading_data.assets.push(
                                asset_server
                                    .load_untyped(format!("sounds/trash/trashcan_{}.ogg", num))
                                    .into(),
                            );
                        }
                    },
                ),
            )
            .add_systems(
                Update,
                (
                    trash::update_trash_border,
                    trash::grab_object,
                    trash::release_object,
                )
                    .run_if(in_state(state::PauseState::Running)),
            )
            .add_systems(
                FixedUpdate,
                (
                    trash::load_trash_collisions,
                    trash::play_hit_sound,
                    trash_can::despawn_trash,
                    timer::update_timer,
                    trash_spawner::spawn_trash_event,
                )
                    .run_if(in_state(state::PauseState::Running)),
            );
    }
}
