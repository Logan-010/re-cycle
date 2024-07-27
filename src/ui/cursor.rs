use super::types;
use crate::{rendering, state};
use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Menu), spawn_cursor)
            .add_systems(
                Update,
                update_cursor.run_if(not(in_state(state::GameState::Splash))),
            );
    }
}

pub fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(3),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("ui/sprites/cursor.png"),
                        ..Default::default()
                    },
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        top: Val::Px(50.0),
                        left: Val::Px(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                types::Cursor,
            ));
        });

    commands.spawn((
        TransformBundle {
            local: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        RigidBody::Kinematic,
        types::GhostCursor,
    ));
}

pub fn update_cursor(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<rendering::types::OuterCamera>>,
    mut ghost_cursor_query: Query<&mut Transform, With<types::GhostCursor>>,
    mut cursor_query: Query<&mut Style, With<types::Cursor>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(pos) = window.cursor_position() else {
        return;
    };

    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) else {
        return;
    };

    for mut transform in ghost_cursor_query.iter_mut() {
        transform.translation = world_pos.extend(1.0);
    }

    for mut style in cursor_query.iter_mut() {
        style.top = Val::Px(pos.y);
        style.left = Val::Px((pos.x - 6.0).abs());
    }
}
