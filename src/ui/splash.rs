use crate::state;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Splash), setup_splash)
            .add_systems(
                Update,
                update_splash.run_if(in_state(state::GameState::Splash)),
            );
    }
}

#[derive(Component)]
pub struct SplashScreen(pub f32);

pub fn setup_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for mut window in query.iter_mut() {
        window.cursor.visible = false;
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            StateScoped(state::GameState::Splash),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    image: UiImage {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                        texture: asset_server.load("embedded://splash.png"),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                SplashScreen(3.0),
            ));
        });
}

pub fn update_splash(
    time: Res<Time>,
    mut color_query: Query<(&mut UiImage, &mut SplashScreen)>,
    mut next_state: ResMut<NextState<state::GameState>>,
    mut elapsed: Local<f32>,
) {
    for (mut ui_image, mut splash_screen) in color_query.iter_mut() {
        if splash_screen.0 <= 0.0 {
            next_state.set(state::GameState::Menu);
            return;
        } else {
            let delta = time.delta_seconds();

            splash_screen.0 -= delta;
            *elapsed += delta;

            ui_image.color.set_alpha((*elapsed * 1.5).sin());
        }
    }
}
