use crate::input::TogglePause;
use crate::loading::{AppState, spawn_message};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<PauseState>()
        .add_systems(OnEnter(PauseState::Paused), (pause_time, spawn_overlay))
        .add_systems(OnExit(PauseState::Paused), resume_time)
        .add_observer(toggle_pause);
}

/// Pausing also stops `Time<Virtual>`, which freezes fixed-timestep systems
/// (physics) and game-time timers. Only exists while running
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(AppState = AppState::Running)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}

fn toggle_pause(
    _: On<Start<TogglePause>>,
    state: Option<Res<State<PauseState>>>,
    mut next: ResMut<NextState<PauseState>>,
) {
    // `PauseState` doesn't exist outside `AppState::Running`.
    let Some(state) = state else {
        return;
    };
    next.set(match state.get() {
        PauseState::Running => PauseState::Paused,
        PauseState::Paused => PauseState::Running,
    });
}

fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn spawn_overlay(commands: Commands) {
    spawn_message(commands, "Paused", PauseState::Paused);
}
