pub mod camera;
pub mod types;

use bevy::prelude::*;

pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera)
            .add_systems(Update, camera::fit_canvas);
    }
}
