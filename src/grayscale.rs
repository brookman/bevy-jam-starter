//! Example fullscreen effect. Uses `FullscreenMaterial`
//! Runs after bloom, before tonemapping (and its dither), i.e. on linear HDR color.

use crate::input::ToggleGrayscale;
use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;
use bevy_enhanced_input::prelude::Start;

pub fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<Grayscale>::default())
        .add_observer(toggle);
}

#[derive(Component, ExtractComponent, ShaderType, Clone, Copy)]
#[extract_app(RenderApp)]
pub struct Grayscale {
    /// 0 = original colors, 1 = fully gray
    pub strength: f32,
}

impl Default for Grayscale {
    fn default() -> Self {
        Self { strength: 1.0 }
    }
}

impl FullscreenMaterial for Grayscale {
    fn fragment_shader() -> ShaderRef {
        "shaders/grayscale.wesl".into()
    }
}

fn toggle(
    _: On<Start<ToggleGrayscale>>,
    mut commands: Commands,
    cameras: Query<(Entity, Has<Grayscale>), With<Camera3d>>,
) {
    for (camera, active) in &cameras {
        if active {
            commands.entity(camera).remove::<Grayscale>();
        } else {
            commands.entity(camera).insert(Grayscale::default());
        }
    }
}
