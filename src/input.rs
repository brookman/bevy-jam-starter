use crate::pause::PauseState;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<Global>()
        .add_input_context::<Sandbox>()
        .init_resource::<KeyboardCaptured>()
        .add_systems(Startup, spawn)
        .add_systems(Update, update_context_activity);
}

/// Set while another UI uses the keyboard
#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct KeyboardCaptured(pub bool);

/// Input context that stays active while paused.
#[derive(Component, Default, Clone)]
pub struct Global;

/// Input context, inactive while paused
#[derive(Component, Default, Clone)]
pub struct Sandbox;

#[derive(InputAction)]
#[action_output(bool)]
pub struct TogglePause;

#[derive(InputAction)]
#[action_output(bool)]
pub struct DropBurst;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ClearSpheres;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ToggleGrayscale;

fn spawn(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Name::new("Global Input")
        Global
        Actions<Global> [
            Action::<TogglePause>::new()
            Bindings [
                Binding(KeyCode::KeyP)
                --
                Binding(KeyCode::Escape)
                --
                Binding(GamepadButton::Start)
            ]
        ]
    });
    commands.spawn_scene(bsn! {
        Name::new("Sandbox Input")
        Sandbox
        Actions<Sandbox> [
            Action::<DropBurst>::new()
            Bindings [
                Binding(KeyCode::Space)
                --
                Binding(GamepadButton::South)
            ]
            --
            Action::<ClearSpheres>::new()
            Bindings [
                Binding(KeyCode::KeyR)
                --
                Binding(GamepadButton::West)
            ]
            --
            Action::<ToggleGrayscale>::new()
            Bindings [
                Binding(KeyCode::KeyG)
                --
                Binding(GamepadButton::North)
            ]
        ]
    });
}

fn update_context_activity(
    mut commands: Commands,
    captured: Res<KeyboardCaptured>,
    pause: Option<Res<State<PauseState>>>,
    global: Query<(Entity, &ContextActivity<Global>)>,
    sandbox: Query<(Entity, &ContextActivity<Sandbox>)>,
) {
    let (global_active, sandbox_active) =
        context_activity(captured.0, pause.map(|pause| *pause.get()));
    for (entity, activity) in &global {
        if **activity != global_active {
            commands
                .entity(entity)
                .insert(ContextActivity::<Global>::new(global_active));
        }
    }
    for (entity, activity) in &sandbox {
        if **activity != sandbox_active {
            commands
                .entity(entity)
                .insert(ContextActivity::<Sandbox>::new(sandbox_active));
        }
    }
}

const fn context_activity(keyboard_captured: bool, pause: Option<PauseState>) -> (bool, bool) {
    let global = !keyboard_captured && pause.is_some();
    (global, global && matches!(pause, Some(PauseState::Running)))
}
