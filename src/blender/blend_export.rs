use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const EXPORT_SCRIPT: &str = include_str!("blender_export.py");

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
