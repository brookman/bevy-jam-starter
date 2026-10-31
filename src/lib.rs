mod audio;
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
mod blender;
mod camera;
mod config;
#[cfg(feature = "dev")]
mod debug;
mod grayscale;
mod input;
mod loading;
mod pause;
mod physics;
mod scene;
mod skein;

use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMode;
use bevy::prelude::*;
use bevy::window::PresentMode;

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub use blender::process_only::run as process_assets_and_exit;

#[must_use]
pub fn default_plugins() -> PluginGroupBuilder {
    let plugins = DefaultPlugins
        .set(AssetPlugin {
            mode: AssetMode::Processed,
            use_asset_processor_override: Some(cfg!(all(
                debug_assertions,
                not(target_arch = "wasm32")
            ))),
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                // Render into the page's canvas (see ndex.html)
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy".into()),
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        });

    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    let plugins = plugins.add(blender::plugin);

    plugins
}

/// after [`default_plugins`].
pub fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_plugins(debug::plugin);

    app.add_plugins((
        loading::plugin, // must be first
        audio::plugin,
        camera::plugin,
        grayscale::plugin,
        input::plugin,
        pause::plugin,
        physics::plugin,
        scene::plugin,
        skein::plugin,
    ));
}
