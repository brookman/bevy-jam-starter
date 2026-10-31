use bevy::prelude::*;

fn main() -> AppExit {
    if std::env::args().any(|arg| arg == "--process-assets") {
        #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
        return bevy_starter::process_assets_and_exit();
        #[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
        {
            eprintln!("--process-assets is only available in native debug builds");
            return AppExit::error();
        }
    }

    App::new()
        .add_plugins((bevy_starter::default_plugins(), bevy_starter::plugin))
        .run()
}
