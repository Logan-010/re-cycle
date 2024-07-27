use super::types;
use crate::ui;
use avian2d::prelude::*;
use bevy::{
    audio::{PlaybackMode, Volume},
    color::palettes::basic,
    prelude::*,
};

#[allow(clippy::type_complexity)]
pub fn load_trash_collisions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut types::Trash, &Handle<Image>), Without<Collider>>,
    images: ResMut<Assets<Image>>,
) {
    for (entity, mut trash, handle) in query.iter_mut() {
        if let Some(image) = images.get(handle) {
            let mut points: Vec<Vec2> = edges::Edges::from(image)
                .single_image_edge_translated()
                .iter()
                .map(|point| Vec2::new(point.x / (512.0 / 30.0), point.y / (512.0 / 30.0)))
                .collect();

            commands.entity(entity).insert((
                RigidBody::Dynamic,
                Collider::convex_hull(points.clone()).unwrap(),
                Restitution::new(0.25).with_combine_rule(CoefficientCombine::Min),
                GravityScale(4.0),
                Friction::new(100.0).with_combine_rule(CoefficientCombine::Min),
            ));

            trash.points.append(&mut points);
        }
    }
}

pub fn update_trash_border(
    time: Res<Time>,
    query: Query<(&mut types::Trash, &Transform, &Rotation)>,
    mut gizmos: Gizmos<types::TrashGizmoGroup>,
    first_click_query: Query<&types::FirstClick>,
) {
    let unclicked = first_click_query.iter().count() == 0;

    if unclicked {
        for (trash, transform, rotation) in query.iter() {
            gizmos.primitive_2d(
                &BoxedPolygon::new(trash.points.clone()),
                transform.translation.xy(),
                rotation.as_radians(),
                Color::from(basic::AQUA).with_alpha((time.elapsed_seconds() * 1.5).sin() + 1.0),
            );
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn grab_object(
    mut commands: Commands,
    space_query: SpatialQuery,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_pos_query: Query<
        (Entity, &Transform),
        (With<ui::types::GhostCursor>, Without<types::HoldingObject>),
    >,
    is_trash: Query<&types::Trash, Without<types::HeldObject>>,
    asset_server: Res<AssetServer>,
    first_click_query: Query<&types::FirstClick>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (entity, transform) in mouse_pos_query.iter() {
            if let Some(ray_hit_data) = space_query.cast_ray_predicate(
                transform.translation.xy(),
                Dir2::X,
                1.0,
                true,
                SpatialQueryFilter::default(),
                &|entity| is_trash.get(entity).is_ok(),
            ) {
                commands
                    .entity(ray_hit_data.entity)
                    .insert(types::HeldObject);
                commands.entity(entity).insert(types::HoldingObject);

                commands.spawn((
                    types::HeldObjectJoint,
                    RevoluteJoint::new(entity, ray_hit_data.entity)
                        .with_local_anchor_2(Vec2::X * 1.0)
                        .with_compliance(0.001),
                ));

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/game/pickup.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                });

                if first_click_query.iter().count() == 0 {
                    commands.spawn(types::FirstClick);
                }
                return;
            }

            if let Some(ray_hit_data) = space_query.cast_ray_predicate(
                transform.translation.xy(),
                Dir2::Y,
                1.0,
                true,
                SpatialQueryFilter::default(),
                &|entity| is_trash.get(entity).is_ok(),
            ) {
                commands
                    .entity(ray_hit_data.entity)
                    .insert(types::HeldObject);
                commands.entity(entity).insert(types::HoldingObject);

                commands.spawn((
                    types::HeldObjectJoint,
                    RevoluteJoint::new(entity, ray_hit_data.entity)
                        .with_local_anchor_2(Vec2::X * 1.0)
                        .with_compliance(0.001),
                ));

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/game/pickup.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                });

                if first_click_query.iter().count() == 0 {
                    commands.spawn(types::FirstClick);
                }
                return;
            }

            if let Some(ray_hit_data) = space_query.cast_ray_predicate(
                transform.translation.xy(),
                -Dir2::X,
                1.0,
                true,
                SpatialQueryFilter::default(),
                &|entity| is_trash.get(entity).is_ok(),
            ) {
                commands
                    .entity(ray_hit_data.entity)
                    .insert(types::HeldObject);
                commands.entity(entity).insert(types::HoldingObject);

                commands.spawn((
                    types::HeldObjectJoint,
                    RevoluteJoint::new(entity, ray_hit_data.entity)
                        .with_local_anchor_2(Vec2::X * 1.0)
                        .with_compliance(0.001),
                ));

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/game/pickup.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                });

                if first_click_query.iter().count() == 0 {
                    commands.spawn(types::FirstClick);
                }
                return;
            }

            if let Some(ray_hit_data) = space_query.cast_ray_predicate(
                transform.translation.xy(),
                -Dir2::Y,
                1.0,
                true,
                SpatialQueryFilter::default(),
                &|entity| is_trash.get(entity).is_ok(),
            ) {
                commands
                    .entity(ray_hit_data.entity)
                    .insert(types::HeldObject);
                commands.entity(entity).insert(types::HoldingObject);

                commands.spawn((
                    types::HeldObjectJoint,
                    RevoluteJoint::new(entity, ray_hit_data.entity)
                        .with_local_anchor_2(Vec2::X * 1.0)
                        .with_compliance(0.001),
                ));

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/game/pickup.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                });

                if first_click_query.iter().count() == 0 {
                    commands.spawn(types::FirstClick);
                }
                return;
            }
        }
    }
}

pub fn release_object(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    held_objects_query: Query<Entity, With<types::HeldObject>>,
    holding_objects_query: Query<Entity, With<types::HoldingObject>>,
    held_object_joint: Query<Entity, With<types::HeldObjectJoint>>,
    asset_server: Res<AssetServer>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        for holding_entity in holding_objects_query.iter() {
            commands
                .entity(holding_entity)
                .remove::<types::HoldingObject>();
            for held_entity in held_objects_query.iter() {
                commands.entity(held_entity).remove::<types::HeldObject>();

                for object_joint_entity in held_object_joint.iter() {
                    commands.entity(object_joint_entity).despawn_recursive();

                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/game/letgo.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new(0.5),
                            ..Default::default()
                        },
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn play_hit_sound(
    mut commands: Commands,
    trash_velocity_query: Query<(&LinearVelocity, &CollidingEntities), With<types::Trash>>,
    audio_object_query: Query<&types::HitObjectAudio>,
    asset_server: Res<AssetServer>,
) {
    for (velocity, colliding_entities) in trash_velocity_query.iter() {
        if colliding_entities.0.iter().count() != 0
            && (velocity.0.x.abs() + velocity.0.y.abs()) / 2.0 >= 20.0
            && audio_object_query.iter().count() == 0
        {
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("sounds/game/hit.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.5),
                        ..Default::default()
                    },
                },
                types::HitObjectAudio,
            ));
        }
    }
}
