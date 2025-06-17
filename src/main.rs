use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};
use camera::{CameraSettings, ParticleCamera};
use camera::camera_controls;
use compute::ComputePlugin;
use data::SimulationSettings;
use draw::DrawPlugin;
use events::ParticleEvent;

mod camera;
mod compute;
mod data;
mod draw;
mod events;
mod ui;

fn main() {
    App::new()
        .add_event::<ParticleEvent>()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin { enable_multipass_for_primary_context: true },
            // Used by ui to display the fps.
            FrameTimeDiagnosticsPlugin::default(),
            ComputePlugin,
            DrawPlugin,
        ))
        .add_systems(EguiContextPass, ui::ui)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controls)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        CameraSettings {
            pan_speed: 1.,
            scroll_speed: 1.,
        },
        ParticleCamera,
    ));

    let mut settings = SimulationSettings::default();
    settings.randomize_attractions();
    commands.insert_resource(settings);
}
