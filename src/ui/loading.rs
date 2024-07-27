use bevy::{
    asset::LoadState,
    prelude::*,
    render::{render_resource::PipelineCache, MainWorld},
};

use crate::{rendering, state};

#[derive(Resource)]
pub enum LoadingState {
    Loading,
    Ready,
}

#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    pub assets: Vec<UntypedHandle>,
    confirmation_frames_target: usize,
    confirmation_frames_count: usize,
}

impl LoadingData {
    pub fn new(confirmation_frames_target: usize) -> Self {
        Self {
            confirmation_frames_target,
            ..Default::default()
        }
    }
}

#[derive(Resource, Default)]
pub struct PipelinesReady(pub bool);

pub struct PipelinesReadyPlugin;

impl Plugin for PipelinesReadyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipelinesReady::default());

        app.sub_app_mut(bevy::render::RenderApp)
            .add_systems(ExtractSchedule, update_pipelines_ready);
    }
}

pub struct LoadingScreenPlugin {
    confirmation_frames_target: usize,
}

impl LoadingScreenPlugin {
    pub fn new(confirmation_frames_target: usize) -> Self {
        Self {
            confirmation_frames_target,
        }
    }
}

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PipelinesReadyPlugin)
            .insert_resource(LoadingData::new(self.confirmation_frames_target))
            .insert_resource(LoadingState::Loading)
            .add_systems(Update, update_loading_data)
            .add_systems(OnEnter(state::GameState::InGame), setup_loading_screen)
            .add_systems(
                Update,
                display_loading_screen.run_if(in_state(state::GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingCamera;

fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
        pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0;
    }
}

pub fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<LoadingState>,
    pipelines_ready: Res<PipelinesReady>,
    mut next_state: ResMut<NextState<state::PauseState>>,
    asset_server: Res<AssetServer>,
) {
    if !loading_data.assets.is_empty() || !pipelines_ready.0 {
        next_state.set(state::PauseState::Loading);
        loading_data.confirmation_frames_count = 0;

        let mut pop_list: Vec<usize> = Vec::new();
        for (index, asset) in loading_data.assets.iter().enumerate() {
            if let Some(LoadState::Loaded) = asset_server.get_load_state(asset) {
                pop_list.push(index);
            }
        }

        for i in pop_list.iter().rev() {
            loading_data.assets.remove(*i);
        }
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
            *loading_state = LoadingState::Ready;
            next_state.set(state::PauseState::Running);
        }
    }
}

pub fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        LoadingCamera,
    ));

    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                z_index: ZIndex::Global(3),
                ..Default::default()
            },
            LoadingScreen,
            StateScoped(state::GameState::InGame),
            rendering::types::HIGH_RES_LAYERS,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 30.0,
                    ..Default::default()
                },
            ));
        });
}

fn display_loading_screen(
    mut commands: Commands,
    mut loading_screen: Query<&mut Visibility, With<LoadingScreen>>,
    loading_state: Res<LoadingState>,
    query: Query<Entity, With<LoadingCamera>>,
) {
    match loading_state.as_ref() {
        LoadingState::Loading => {
            *loading_screen.get_single_mut().unwrap() = Visibility::Visible;
        }
        LoadingState::Ready => {
            *loading_screen.get_single_mut().unwrap() = Visibility::Hidden;
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        }
    };
}
