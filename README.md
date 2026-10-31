# Bevy Jam starter
A starter/example Bevy project for the [Bevy Jam #8](https://itch.io/jam/bevy-jam-8).
It contains a few useful plugins and samples to get started.
The contents were mostly compiled from private projects and the official [Bevy examples](https://bevy.org/examples/).

## Blender plugin
- Watches .blend files for changes and converts them to .glb by invoking Blender.
- Bevy then hot-reloads the .glb file.
- Derived .glb files are not commited.
- Support for Skein.

## Debug plugin
A super basic editor-like UI.
- The plugin is enabled when launching with `cargo dev` .
- Uses bevy_egui and bevy_inspector_egui to show the WorldInspector, an object inspector (hover and press T) as well as a config UI.
- The UI elements are docked to the sides of the screen using egui_dock.
- Shows AABBs when hovering over and object (using bevy::picking)
- Enables some bevy::diagnostics

## Camera
- Uses the new Bevy-native PanOrbitCamera with inputs inspired by [bevy_panorbit_camera](https://github.com/Plonq/bevy_panorbit_camera).
- Enables a few nice-looking defaults and effects: HDR/tonemapping, SSAO, TAA, Bloom, DebandDither

## Config
- Structs with `#[reflect(Resource, crate::config::Config)]` can be modified at runtime using the debug UI.

## Physics
- The main scene of the project is an [Avian](https://github.com/avianphysics/avian) collision sample with a rotating cube.
- Spheres/balls are spawned in above the cube. Press space to drop a bunch of balls.
- Hovering with the mouse over the cube changes its material to emissive.

## Input
- A simple bevy_enhanced_input setup.

## Skein
- Uses [Bevy Skein](https://bevyskein.dev/) to load Bevy components from Blender.
- Requires the Skein Blender addon, see: https://bevyskein.dev/docs/installation
- The physics objects `cube` and `ball` (including their Avian components) are imported from Blender.

## Build
- The project should build for MacOS, Linux and Web (requires WebGPU)
- Build scripts for `release` and `web`.
- The web bundle size is reduced by using `wasm-opt`.
- A Tracy setup for profiling. (`tracy_debug.sh` and `tracy_release.sh`)
- Uses a Nix dev shell for trayc and binaryen (wasm-opt).

## Misc
- Loading: Waits for all objects to be loaded (shows "Loading...") before the game starts.
- Pause/unpause with ESC.
- Audio: Plays an audio effect on physics collisions. The audio volume is modulated by the collision velocity.
- Shader: An example of a fullscreen grayscale shader effect using wesl. Toggle the effect with the G key.

