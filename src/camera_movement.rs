use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera;

#[derive(Default)]
pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CameraInputState>()
            .add_system(camera_movement.system());
    }
}

#[derive(Default)]
struct CameraInputState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    left_mouse_button_down: bool,
    right_mouse_button_down: bool,
}

fn camera_movement(
    mut camera_input_state: ResMut<CameraInputState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&Camera, &mut Translation, &mut Scale)>,
) {
    let mut move_delta = Vec2::zero();
    let mut zoom_delta = 0.0;

    // Collate all the mouse movements since last time
    for event in camera_input_state
        .mouse_motion_event_reader
        .iter(&mouse_motion_events)
    {
        let event_delta = event.delta;

        if camera_input_state.left_mouse_button_down {
            move_delta += event_delta;
        }
        if camera_input_state.right_mouse_button_down {
            let change = event_delta.y() / 500.0;
            if change != 0.0 {
                zoom_delta += change;
            }
        }
    }

    // Handle keyboard input
    if keyboard_input.pressed(KeyCode::A) {
        *move_delta.x_mut() += 10.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        *move_delta.x_mut() -= 10.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        *move_delta.y_mut() += 10.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        *move_delta.y_mut() -= 10.0;
    }

    if keyboard_input.pressed(KeyCode::Add) {
        zoom_delta -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::Subtract) {
        zoom_delta += 0.1;
    }

    for (_, mut translation, mut scale) in &mut query.iter() {
        // Move the camera according to the delta.
        if move_delta.length() > 0.0 {
            *translation.x_mut() -= move_delta.x();
            *translation.y_mut() += move_delta.y();
        }

        // Scale the camera according to the delta.
        if (zoom_delta - 0.0).abs() > 0.01 {
            scale.0 += zoom_delta;
            if scale.0 < 1.0 {
                scale.0 = 1.0;
            }
        }
    }

    // Update our mouse button state for next time
    if mouse_button_input.just_pressed(MouseButton::Left) {
        camera_input_state.left_mouse_button_down = true;
    } else if mouse_button_input.just_released(MouseButton::Left) {
        camera_input_state.left_mouse_button_down = false;
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        camera_input_state.right_mouse_button_down = true;
    } else if mouse_button_input.just_released(MouseButton::Right) {
        camera_input_state.right_mouse_button_down = false;
    }
}
