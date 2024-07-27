use bevy::{prelude::*, render::view::RenderLayers};

pub const RES_WIDTH: u32 = 16 * 10;
pub const RES_HEIGHT: u32 = 9 * 10;

pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

//Pixel perfect image
#[derive(Component)]
pub struct Canvas;

//Pixel perfect camera
#[derive(Component)]
pub struct InGameCamera;

//Normal camera (renders pixel perfect one)
#[derive(Component)]
pub struct OuterCamera;
