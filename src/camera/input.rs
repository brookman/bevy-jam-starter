use bevy::camera_controller::pan_orbit_camera::prelude::PanOrbitCamera;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::pointer::{PointerAction, PointerId, PointerInput};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<CameraInput>()
        .init_resource::<CameraInputBlocked>()
        .add_systems(
            PreUpdate,
            camera_input
                .after(bevy::picking::PickingSystems::Last)
                .before(PanOrbitCamera::update_camera_positions),
        );
}

const ORBIT_BUTTON: MouseButton = MouseButton::Right;
const PAN_BUTTON: MouseButton = MouseButton::Middle;
const PIXELS_PER_SCROLL_LINE: f32 = 150.0;

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct CameraInputBlocked(pub bool);

#[derive(Resource, Default)]
struct CameraInput {
    motion: Option<Motion>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Motion {
    Orbit,
    Pan,
    Zoom,
}

fn camera_input(
    blocked: Res<CameraInputBlocked>,
    mut state: ResMut<CameraInput>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut wheel: MessageReader<MouseWheel>,
    mut pointer_inputs: MessageReader<PointerInput>,
    mut cameras: Query<&mut PanOrbitCamera>,
) {
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    if blocked.0 && state.motion.is_none() {
        wheel.clear();
        pointer_inputs.clear();
        return;
    }

    if state.motion.is_none() {
        if mouse.just_pressed(PAN_BUTTON) {
            camera.start_pan(None);
            state.motion = Some(Motion::Pan);
        } else if mouse.just_pressed(ORBIT_BUTTON) {
            camera.start_orbit(None);
            state.motion = Some(Motion::Orbit);
        }
    }

    match state.motion {
        Some(Motion::Pan) if mouse.just_released(PAN_BUTTON) => {
            camera.end_move();
            state.motion = None;
        }
        Some(Motion::Orbit) if mouse.just_released(ORBIT_BUTTON) => {
            camera.end_move();
            state.motion = None;
        }
        _ => {}
    }

    let screenspace_input = pointer_inputs
        .read()
        .filter(|input| input.pointer_id == PointerId::Mouse)
        .filter_map(|input| match input.action {
            PointerAction::Move { delta } => Some(delta),
            _ => None,
        })
        .sum::<Vec2>();
    camera.send_screenspace_input(screenspace_input);

    let zoom_input = wheel
        .read()
        .map(|event| {
            let multiplier = match event.unit {
                MouseScrollUnit::Line => PIXELS_PER_SCROLL_LINE,
                MouseScrollUnit::Pixel => 1.0,
            };
            event.y * multiplier
        })
        .sum::<f32>();

    if state.motion.is_none() && zoom_input != 0.0 {
        camera.start_zoom(None);
        state.motion = Some(Motion::Zoom);
    }
    if state.motion == Some(Motion::Zoom) {
        camera.send_zoom_input(zoom_input);
        if zoom_input == 0.0 {
            camera.end_move();
            state.motion = None;
        }
    }
}
