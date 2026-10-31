use bevy::asset::AssetPath;
use bevy::asset::processor::{AssetProcessor, ProcessStatus};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, futures::check_ready};
use std::path::{Path, PathBuf};

#[derive(Resource)]
struct ProcessingTask(Task<Vec<PathBuf>>);

#[must_use]
pub fn run() -> AppExit {
    use bevy::window::ExitCondition;

    App::new()
        .add_plugins(crate::default_plugins().set(WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..default()
        }))
        .add_plugins(plugin)
        .run()
}

fn plugin(app: &mut App) {
    app.add_systems(Startup, start)
        .add_systems(Update, poll.run_if(resource_exists::<ProcessingTask>));
}

fn start(mut commands: Commands, processor: Res<AssetProcessor>) {
    let processor = processor.clone();
    let blends = find_blends(&super::assets_root());
    let task = IoTaskPool::get().spawn(async move {
        processor.data().wait_until_finished().await;
        let mut failed = Vec::new();
        for rel in blends {
            let path = AssetPath::from_path(&rel).into_owned();
            if !matches!(
                processor.data().wait_until_processed(path).await,
                ProcessStatus::Processed
            ) {
                failed.push(rel);
            }
        }
        failed
    });
    commands.insert_resource(ProcessingTask(task));
}

fn poll(
    mut commands: Commands,
    mut task: ResMut<ProcessingTask>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(failed) = check_ready(&mut task.0) else {
        return;
    };
    // A completed task must not be polled again.
    commands.remove_resource::<ProcessingTask>();
    if failed.is_empty() {
        info!("All assets processed");
        exit.write(AppExit::Success);
    } else {
        for path in &failed {
            error!("Failed to process {}", path.display());
        }
        exit.write(AppExit::error());
    }
}

fn find_blends(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for path in entries.flatten().map(|e| e.path()) {
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "blend") {
                out.push(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
            }
        }
    }
    out
}
