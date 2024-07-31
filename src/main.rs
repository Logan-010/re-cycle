#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod asset_embedding;
mod constants;
mod level;
mod rendering;
mod score;
mod state;
mod ui;

use avian2d::prelude::*;
#[cfg(feature = "dev-tools")]
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::WindowResolution,
};
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;
use bevy_pkv::PkvStore;
use winit::window::Icon;

fn set_window_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    let icon = Icon::from_rgba(include_bytes!("../icon.rgba").to_vec(), 512, 512).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "(Re)cycle".to_string(),
                        resizable: false,
                        focused: true,
                        resolution: WindowResolution::new(
                            constants::SCREEN_WIDTH,
                            constants::SCREEN_HEIGHT,
                        ),
                        #[cfg(debug_assertions)]
                        mode: WindowMode::Windowed,
                        #[cfg(not(debug_assertions))]
                        #[cfg(not(target_arch = "wasm32"))]
                        mode: WindowMode::Fullscreen,
                        prevent_default_event_handling: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    mode: AssetMode::Unprocessed,
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            asset_embedding::EmbeddedAssetPlugin,
            PhysicsPlugins::default().with_length_unit(8.0),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            #[cfg(feature = "dev-tools")]
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 20.0,
                        ..Default::default()
                    },
                },
            },
        ))
        .insert_resource(PkvStore::new("DimGames", "(Re)cycle"))
        .insert_resource(Msaa::Off)
        .add_plugins((
            rendering::PixelPerfectPlugin,
            state::StatePlugin,
            ui::ReCycleUiPlugin::default(),
            level::LevelPlugin,
            score::ScorePlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .run()
}
