use bevy::prelude::*;

#[derive(Component)]
pub struct Trash {
    pub points: Vec<Vec2>,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct TrashGizmoGroup;

#[derive(Component)]
pub struct HeldObject;

#[derive(Component)]
pub struct HoldingObject;

#[derive(Component)]
pub struct HeldObjectJoint;

#[derive(Component)]
pub struct TrashCanSensor;

#[derive(Component)]
pub struct FirstClick;

#[derive(Component)]
pub struct HitObjectAudio;

#[derive(Component)]
pub struct TimeRemaining {
    pub remaining: f32,
    pub multiplier: f32,
    pub spawn_every: f32,
    pub last_spawn: f32,
}

impl Default for TimeRemaining {
    fn default() -> Self {
        Self {
            remaining: 30.0,
            multiplier: 1.0,
            spawn_every: 2.8,
            last_spawn: 0.0,
        }
    }
}

#[derive(Event)]
pub struct TrashSpawnEvent;
