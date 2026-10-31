use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::Hdr;
use bevy::camera_controller::pan_orbit_camera::controller::smoothing::Smoothing;
use bevy::camera_controller::pan_orbit_camera::prelude::*;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use std::time::Duration;

mod input;

#[cfg(feature = "dev")]
pub use input::CameraInputBlocked;

pub fn plugin(app: &mut App) {
    app.add_plugins((DefaultPanOrbitCameraPlugins, input::plugin));
    app.add_systems(Startup, spawn);
}

fn spawn(mut commands: Commands) {
    let translation = Vec3::new(0.0, 4.0, 4.0);
    let focus = Vec3::new(0.0, 0.5, 0.0);
    let anchor_depth = -f64::from((translation - focus).length());

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(translation).looking_at(focus, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            near: 0.1,
            far: 20.0,
            ..default()
        }),
        PanOrbitCamera {
            last_anchor_depth: anchor_depth,
            smoothing: Smoothing {
                pan: Duration::ZERO,
                orbit: Duration::ZERO,
                zoom: Duration::ZERO,
            },
            ..default()
        },
        Hdr,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        TemporalAntiAliasing::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    ));
}
