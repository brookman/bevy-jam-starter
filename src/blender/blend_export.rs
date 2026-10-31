use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Paths are passed via `sys.argv`, never interpolated into Python source.
///
/// The skein addon turns its component data into glTF extras during export;
/// without it the export would silently drop every component. Background
/// Blender only loads add-ons enabled in the *saved* preferences, so enable an
/// installed skein for this run (in memory, prefs aren't saved) and fail if it
/// isn't installed at all.
pub const EXPORT_SCRIPT: &str = "\
import addon_utils, bpy, sys
out = sys.argv[sys.argv.index('--') + 1]

def has_skein_data(item):
    # Blender 5+ keeps addon-defined properties in separate system storage.
    props = item.bl_system_properties_get() if hasattr(item, 'bl_system_properties_get') else item
    return bool(props and props.get('skein_two'))

tagged = [item.name
          for data in (bpy.data.objects, bpy.data.meshes, bpy.data.materials,
                       bpy.data.scenes, bpy.data.cameras, bpy.data.lights)
          for item in data if has_skein_data(item)]
if tagged and not hasattr(bpy.types.Scene, 'skein_extension_properties'):
    module = next((m.__name__ for m in addon_utils.modules()
                   if m.__name__.rsplit('.', 1)[-1] == 'bevy_skein'), None)
    if module is None:
        raise RuntimeError(
            f'{bpy.data.filepath} has skein components (on {tagged}) but the skein '
            'addon is not installed in this Blender; the components would be dropped')
    # default_set: the glTF exporter only runs hooks of add-ons listed as enabled.
    addon_utils.enable(module, default_set=True)
    # Its load_post handler already missed the file load; rebuild the registry.
    bpy.ops.wm.reload_skein_registry()
bpy.ops.export_scene.gltf(filepath=out, export_format='GLB', export_apply=True)
";

#[derive(Debug, thiserror::Error)]
pub enum BlenderExportError {
    #[error("failed to run Blender ({0}); install it or set the BLENDER env var")]
    Spawn(#[source] std::io::Error),
    #[error("Blender export failed for {blend}:\n{stderr}")]
    Failed { blend: PathBuf, stderr: String },
    #[error("Blender reported success but produced no file at {0}")]
    MissingOutput(PathBuf),
    #[error("failed to read Blender output {path}")]
    ReadOutput {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn blender_binary() -> OsString {
    std::env::var_os("BLENDER").unwrap_or_else(|| "blender".into())
}

pub fn export_args(blend: &Path, glb: &Path) -> Vec<OsString> {
    vec![
        blend.into(),
        "--background".into(),
        // Make a Python exception fail the process instead of exiting 0.
        "--python-exit-code".into(),
        "1".into(),
        "--python-expr".into(),
        EXPORT_SCRIPT.into(),
        "--".into(),
        glb.into(),
    ]
}

pub fn run_blender_export(blend: &Path, glb: &Path) -> Result<(), BlenderExportError> {
    let output = Command::new(blender_binary())
        .args(export_args(blend, glb))
        .output()
        .map_err(BlenderExportError::Spawn)?;

    if !output.status.success() {
        return Err(BlenderExportError::Failed {
            blend: blend.to_path_buf(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    if !glb.is_file() {
        return Err(BlenderExportError::MissingOutput(glb.to_path_buf()));
    }
    Ok(())
}
