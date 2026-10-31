use bevy::prelude::*;
use bevy_asset_loader::prelude::*;


pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Running)
                .on_failure_continue_to_state(AppState::LoadFailed),
        )
        .add_systems(OnEnter(AppState::Loading), |commands: Commands| {
            spawn_message(commands, "Loading...", AppState::Loading);
        })
        .add_systems(OnEnter(AppState::LoadFailed), |commands: Commands| {
            spawn_message(
                commands,
                "Failed to load assets, see log",
                AppState::LoadFailed,
            );
        });
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Running,
    LoadFailed,
}

pub fn spawn_message<S: States>(mut commands: Commands, text: &'static str, state: S) {
    commands.spawn((
        Name::new(text),
        DespawnOnExit(state),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Text::new(text),
            TextFont::from_font_size(48.0),
            Pickable::IGNORE,
        )],
    ));
}
