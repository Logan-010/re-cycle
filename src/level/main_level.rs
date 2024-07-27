use super::types;
use crate::state;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Background
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/level/level.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0 * 17.0, 10.0 * 10.0)),
                color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(state::GameState::InGame),
    ));

    //Trash can
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -30.0, 2.0),
            texture: asset_server.load("sprites/misc/trash_can.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(109.0 * 0.18, 142.0 * 0.18)),
                color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(state::GameState::InGame),
    ));

    //Trash can colliders
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, -38.0, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(109.0 * 0.1, 1.0)));

    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(-6.0, -27.5, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(1.0, 142.0 * 0.13)));

    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(7.0, -27.5, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(1.0, 142.0 * 0.13)));

    //Trash can sensor
    commands.spawn((
        TransformBundle {
            local: Transform::from_xyz(0.0, -34.0, 0.0),
            ..Default::default()
        },
        Collider::ellipse(2.5, 2.0),
        Sensor,
        types::TrashCanSensor,
        StateScoped(state::GameState::InGame),
    ));

    //Outer walls
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, -40.0, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(160.0, 2.0)));

    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(-80.0, 2.0, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(2.0, 80.0)));

    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(80.0, 2.0, 0.0),
                ..Default::default()
            },
            StateScoped(state::GameState::InGame),
        ))
        .insert((RigidBody::Static, Collider::rectangle(2.0, 80.0)));

    let mut rng = thread_rng();
    for _ in 0..rng.gen_range(4..8) {
        let x = loop {
            let candidate_x = rng.gen_range(-70.0..70.0);
            if !(-8.0..=9.0).contains(&candidate_x) {
                break candidate_x;
            }
        };
        let y = -27.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 1.0),
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
