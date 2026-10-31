use crate::loading::AppState;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .init_resource::<SceneConfig>()
        .insert_resource(GlobalAmbientLight {
            brightness: 500.,
            ..default()
        })
        .configure_loading_state(
            LoadingStateConfig::new(AppState::Loading).load_collection::<CubeAssets>(),
        )
        .add_systems(Startup, spawn_light)
        .add_systems(OnEnter(AppState::Running), spawn_cube)
        .register_type::<EmissiveOnHover>()
        .add_observer(setup_emissive_on_hover)
        .add_systems(
            Update,
            (
                show_hover_emission,
                apply_config.run_if(resource_changed::<SceneConfig>),
            ),
        );
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource, crate::config::Config)]
#[cfg_attr(feature = "dev", derive(InspectorOptions), reflect(InspectorOptions))]
pub struct SceneConfig {
    #[cfg_attr(
        feature = "dev",
        inspector(min = 0.0, max = 10_000_000.0, suffix = " lm")
    )]
    pub light_intensity: f32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            light_intensity: PointLight::default().intensity,
        }
    }
}

#[derive(AssetCollection, Resource)]
struct CubeAssets {
    #[asset(path = "models/demo.blend#Scene0")]
    scene: Handle<WorldAsset>,
}

#[derive(Component)]
struct SceneLight;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct EmissiveOnHover;

/// The emission exported from Blender, restored while hovered.
#[derive(Component, Debug, PartialEq)]
struct HoverEmission(LinearRgba);

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Light"),
        SceneLight,
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 4.0, 4.0),
    ));
}

fn spawn_cube(mut commands: Commands, assets: Res<CubeAssets>) {
    commands.spawn((
        Name::new("Cube Scene"),
        WorldAssetRoot(assets.scene.clone()),
    ));
}

/// Makes a copy of the material
fn setup_emissive_on_hover(
    add: On<Add<EmissiveOnHover>>,
    mut commands: Commands,
    mut meshes: Query<(&mut MeshMaterial3d<StandardMaterial>, Option<&ChildOf>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((mut handle, parent)) = meshes.get_mut(add.entity) else {
        warn!(
            "EmissiveOnHover on {} without a StandardMaterial",
            add.entity
        );
        return;
    };
    let Some(mut material) = materials.get(&handle.0).cloned() else {
        return;
    };
    let emission = material.emissive;
    material.emissive = LinearRgba::BLACK;
    handle.0 = materials.add(material);

    commands.entity(add.entity).insert(HoverEmission(emission));
    let target = parent.map_or(add.entity, ChildOf::parent);
    commands.entity(target).insert(Hovered::default());
}

fn show_hover_emission(
    hovered: Query<&Hovered>,
    changed: Query<(), Changed<Hovered>>,
    meshes: Query<(
        Entity,
        &HoverEmission,
        &MeshMaterial3d<StandardMaterial>,
        Option<&ChildOf>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, emission, handle, parent) in &meshes {
        let target = parent.map_or(entity, ChildOf::parent);
        if !changed.contains(target) {
            continue;
        }
        let Ok(hovered) = hovered.get(target) else {
            continue;
        };
        if let Some(mut material) = materials.get_mut(&handle.0) {
            material.emissive = if hovered.get() {
                emission.0
            } else {
                LinearRgba::BLACK
            };
        }
    }
}

fn apply_config(config: Res<SceneConfig>, mut lights: Query<&mut PointLight, With<SceneLight>>) {
    for mut light in &mut lights {
        light.intensity = config.light_intensity;
    }
}

