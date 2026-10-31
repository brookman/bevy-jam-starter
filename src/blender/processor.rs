use super::blend_export::{BlenderExportError, run_blender_export};
use bevy::asset::io::Writer;
use bevy::asset::processor::{Process, ProcessContext, ProcessError};
use bevy::gltf::{GltfLoader, GltfLoaderSettings};
use bevy::prelude::*;
use bevy::tasks::futures_lite::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default, Serialize, Deserialize)]
pub struct BlendProcessorSettings;

#[derive(TypePath)]
pub struct BlendProcessor {
    pub assets_root: PathBuf,
}

impl Process for BlendProcessor {
    type Settings = BlendProcessorSettings;
    type OutputLoader = GltfLoader;

    async fn process(
        &self,
        context: &mut ProcessContext<'_>,
        _settings: &Self::Settings,
        writer: &mut Writer,
    ) -> Result<GltfLoaderSettings, ProcessError> {
        // Use the real source path so relative links (textures, libraries) resolve.
        let blend = self.assets_root.join(context.path().path());
        let glb = unique_temp_glb(&blend);

        let export_glb = glb.clone();
        let bytes = blocking::unblock(move || -> Result<Vec<u8>, BlenderExportError> {
            run_blender_export(&blend, &export_glb)?;
            std::fs::read(&export_glb).map_err(|source| BlenderExportError::ReadOutput {
                path: export_glb.clone(),
                source,
            })
        })
        .await;
        let _ = std::fs::remove_file(&glb);

        let bytes = bytes.map_err(|e| ProcessError::AssetTransformError(Box::new(e)))?;
        writer
            .write_all(&bytes)
            .await
            .map_err(|e| ProcessError::AssetTransformError(Box::new(e)))?;
        Ok(GltfLoaderSettings::default())
    }
}

fn unique_temp_glb(blend: &Path) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let stem = blend.file_stem().unwrap_or_default().to_string_lossy();
    std::env::temp_dir().join(format!(
        "bevy-jam-starter-{}-{}-{stem}.glb",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed),
    ))
}
