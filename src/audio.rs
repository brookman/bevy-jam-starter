//! Uses Bevy's built-in audio until `bevy_seedling` supports Bevy 0.20.

use crate::loading::AppState;
use crate::physics::Ball;
use avian3d::prelude::{Collisions, ComputedMass, PhysicsSchedule, PhysicsStepSystems};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

const MIN_IMPACT_STRENGTH: f32 = 1.0;
const FULL_VOLUME_STRENGTH: f32 = 25.0;

pub fn plugin(app: &mut App) {
    // `PhysicsSchedule` only runs when physics actually steps; outside it, a
    // paused simulation would replay the last step's impulses every tick.
    app.configure_loading_state(
        LoadingStateConfig::new(AppState::Loading).load_collection::<AudioAssets>(),
    )
    .add_systems(
        PhysicsSchedule,
        play_impacts
            .in_set(PhysicsStepSystems::Last)
            .run_if(in_state(AppState::Running)),
    );
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "sounds/impact.ogg")]
    impact: Handle<AudioSource>,
}

/// Uses the impulse rather than `CollisionStart`: that event also fires
/// for speculative contacts (before the hit is visible) and is sent after the
/// solver has already changed the velocities.
fn play_impacts(
    mut commands: Commands,
    sound: Res<AudioAssets>,
    collisions: Collisions,
    balls: Query<&ComputedMass, With<Ball>>,
) {
    for pair in collisions.iter() {
        let mass = [pair.body1, pair.body2]
            .into_iter()
            .flatten()
            .find_map(|body| balls.get(body).ok());
        let Some(mass) = mass else {
            continue;
        };
        let strength = pair.max_normal_impulse_magnitude() * mass.inverse();
        if strength < MIN_IMPACT_STRENGTH {
            continue;
        }

        commands.spawn((
            AudioPlayer(sound.impact.clone()),
            PlaybackSettings::DESPAWN
                .with_volume(Volume::Linear((strength / FULL_VOLUME_STRENGTH).min(1.0)))
                .with_speed(fastrand::f32().mul_add(0.3, 0.85)),
        ));
    }
}
