mod dock;

use bevy::camera::primitives::Aabb;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::gizmos::aabb::ShowAabbGizmo;
use bevy::picking::hover::{HoverMap, Hovered};
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use std::iter::once;

const SELECTION_COLOR: Color = Color::linear_rgb(0.0, 0.75, 0.0);

pub fn plugin(app: &mut App) {
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        EntityCountDiagnosticsPlugin::default(),
        SystemInformationDiagnosticsPlugin,
        EguiPlugin::default(),
        DefaultInspectorConfigPlugin,
        dock::plugin,
    ))
    .init_resource::<HoveredEntity>()
    .init_resource::<SelectedEntity>()
    .add_systems(
        Update,
        (
            track_hovered_entity,
            select_hovered_on_keypress,
            update_gizmos,
        )
            .chain(),
    );
}

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct HoveredEntity(pub Option<Entity>);

#[derive(Resource, Default, Debug)]
pub struct SelectedEntity(pub Option<Entity>);

#[derive(Component)]
struct DebugGizmo;

/// A closest hit without a [`Hovered`] owner hides everything behind it
fn track_hovered_entity(
    hover_map: Res<HoverMap>,
    parents: Query<&ChildOf>,
    hoverables: Query<(), With<Hovered>>,
    mut hovered: ResMut<HoveredEntity>,
) {
    let closest = hover_map.get(&PointerId::Mouse).and_then(|hits| {
        hits.iter()
            .min_by(|(_, a), (_, b)| a.depth.total_cmp(&b.depth))
            .map(|(entity, _)| *entity)
    });
    let new = closest.and_then(|hit| {
        once(hit)
            .chain(parents.iter_ancestors(hit))
            .find(|e| hoverables.contains(*e))
    });
    hovered.set_if_neq(HoveredEntity(new));
}

fn select_hovered_on_keypress(
    keys: Res<ButtonInput<KeyCode>>,
    hovered: Res<HoveredEntity>,
    mut selected: ResMut<SelectedEntity>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        selected.0 = hovered.0;
    }
}

fn update_gizmos(
    hovered: Res<HoveredEntity>,
    selected: Res<SelectedEntity>,
    mut commands: Commands,
    current: Query<Entity, With<DebugGizmo>>,
    children: Query<&Children>,
    aabbs: Query<(), With<Aabb>>,
) {
    if !hovered.is_changed() && !selected.is_changed() {
        return;
    }

    for entity in &current {
        commands
            .entity(entity)
            .remove::<(ShowAabbGizmo, DebugGizmo)>();
    }

    let targets = [(hovered.0, None), (selected.0, Some(SELECTION_COLOR))];
    for (root, color) in targets {
        let Some(root) = root else { continue };
        for entity in once(root)
            .chain(children.iter_descendants(root))
            .filter(|e| aabbs.contains(*e))
        {
            commands
                .entity(entity)
                .insert((ShowAabbGizmo { color }, DebugGizmo));
        }
    }
}
