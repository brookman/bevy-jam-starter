mod blend_export;
pub mod process_only;
mod processor;

use bevy::asset::io::file::FileAssetReader;
use bevy::prelude::*;
use std::path::PathBuf;

pub use processor::BlendProcessor;

pub fn assets_root() -> PathBuf {
    FileAssetReader::get_base_path().join("assets")
}

/// Must be added after `DefaultPlugins`.
pub fn plugin(app: &mut App) {
    app.register_asset_processor(BlendProcessor {
        assets_root: assets_root(),
    })
    .set_default_asset_processor::<BlendProcessor>("blend");
}
