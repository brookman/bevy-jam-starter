use super::SelectedEntity;
use crate::camera::CameraInputBlocked;
use crate::config::ReflectConfig;
use crate::input::KeyboardCaptured;
use bevy::picking::PickingSystems;
use bevy::picking::backend::{HitData, PointerHits};
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPrimaryContextPass, PrimaryEguiContext};
use bevy_inspector_egui::{bevy_inspector, egui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

pub fn plugin(app: &mut App) {
    app.init_resource::<DockUi>()
        .add_systems(EguiPrimaryContextPass, show_dock)
        .add_systems(
            PreUpdate,
            block_picking_outside_game.in_set(PickingSystems::Backend),
        );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    World,
    Game,
    Inspector,
    Configs,
}

impl Tab {
    const fn title(self) -> &'static str {
        match self {
            Self::World => "World",
            Self::Game => "Game",
            Self::Inspector => "Inspector",
            Self::Configs => "Configs",
        }
    }
}

#[derive(Resource)]
struct DockUi {
    state: DockState<Tab>,
    game_rect: egui::Rect,
}

impl Default for DockUi {
    fn default() -> Self {
        Self {
            state: default_layout(),
            game_rect: egui::Rect::NOTHING,
        }
    }
}

fn default_layout() -> DockState<Tab> {
    let mut state = DockState::new(vec![Tab::Game]);
    let tree = state.main_surface_mut();
    let [rest, _world] = tree.split_left(NodeIndex::root(), 0.2, vec![Tab::World]);
    let [_game, right] = tree.split_right(rest, 0.72, vec![Tab::Inspector]);
    tree.split_below(right, 0.5, vec![Tab::Configs]);
    state
}

fn show_dock(world: &mut World) {
    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single_mut(world)
    else {
        return;
    };
    let ctx = egui_context.get_mut().clone();
    let mut ui = egui::Ui::new(
        ctx.clone(),
        "dock".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    world.resource_scope::<DockUi, _>(|world, mut dock| {
        let dock = &mut *dock;
        let mut viewer = TabViewer {
            world,
            game_rect: &mut dock.game_rect,
        };
        DockArea::new(&mut dock.state)
            .style(dock_style(&ui))
            .show_inside(&mut ui, &mut viewer);
    });

    let game_rect = world.resource::<DockUi>().game_rect;
    let over_game = ctx.rect_contains_pointer(egui::LayerId::background(), game_rect);
    world
        .resource_mut::<CameraInputBlocked>()
        .set_if_neq(CameraInputBlocked(!over_game));

    // Typing into an inspector field shouldn't trigger game actions.
    let captured = KeyboardCaptured(ctx.egui_wants_keyboard_input());
    world
        .resource_mut::<KeyboardCaptured>()
        .set_if_neq(captured);
}

fn dock_style(ui: &egui::Ui) -> Style {
    let mut style = Style::from_egui(&ui.global_style());
    style.tab.tab_body.stroke = egui::Stroke::NONE;
    style
}

struct TabViewer<'a> {
    world: &'a mut World,
    game_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn id(&mut self, tab: &mut Tab) -> egui::Id {
        egui::Id::new(tab.title())
    }

    fn title(&mut self, tab: &mut Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Tab) {
        match tab {
            Tab::World => bevy_inspector::ui_for_world(self.world, ui),
            Tab::Game => *self.game_rect = ui.clip_rect(),
            Tab::Inspector => inspector_tab(self.world, ui),
            Tab::Configs => configs_tab(self.world, ui),
        }
    }

    fn is_closeable(&self, _tab: &Tab) -> bool {
        false
    }

    fn allowed_in_windows(&self, tab: &mut Tab) -> bool {
        *tab != Tab::Game
    }

    fn clear_background(&self, tab: &Tab) -> bool {
        *tab != Tab::Game
    }

    fn scroll_bars(&self, tab: &Tab) -> [bool; 2] {
        [false, *tab != Tab::Game]
    }
}

fn block_picking_outside_game(
    blocked: Res<CameraInputBlocked>,
    egui_context: Single<Entity, With<PrimaryEguiContext>>,
    mut hits: MessageWriter<PointerHits>,
) {
    if blocked.0 {
        let entity = *egui_context;
        hits.write(PointerHits::new(
            PointerId::Mouse,
            vec![(entity, HitData::new(entity, 0.0, None, None))],
            f32::MAX,
        ));
    }
}

fn inspector_tab(world: &mut World, ui: &mut egui::Ui) {
    match world.resource::<SelectedEntity>().0 {
        Some(entity) if world.get_entity(entity).is_ok() => {
            ui.label(format!("{entity:?}"));
            ui.separator();
            bevy_inspector::ui_for_entity_with_children(world, entity, ui);
        }
        Some(entity) => {
            ui.label(format!("Selected entity {entity:?} no longer exists"));
        }
        None => {
            ui.label("Hover an entity and press T.");
        }
    }
}

fn configs_tab(world: &mut World, ui: &mut egui::Ui) {
    let registry = world.resource::<AppTypeRegistry>().0.clone();

    let mut configs: Vec<_> = registry
        .read()
        .iter_with_data::<ReflectConfig>()
        .filter(|(registration, _)| registration.data::<ReflectResource>().is_some())
        .map(|(registration, _)| {
            (
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
            )
        })
        .filter(|(_, type_id)| {
            world
                .components()
                .get_id(*type_id)
                .is_some_and(|id| world.get_resource_by_id(id).is_some())
        })
        .collect();
    configs.sort_by_key(|(name, _)| *name);

    if configs.is_empty() {
        ui.label("No resources are tagged with #[reflect(Config)].");
        return;
    }

    let type_registry = registry.read();
    for (name, type_id) in configs {
        egui::CollapsingHeader::new(name)
            .default_open(true)
            .show(ui, |ui| {
                bevy_inspector::by_type_id::ui_for_resource(
                    world,
                    type_id,
                    ui,
                    name,
                    &type_registry,
                );
            });
    }
}
