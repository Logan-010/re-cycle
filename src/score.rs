use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerScore {
    pub score: u64,
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerScore::default());
    }
}
