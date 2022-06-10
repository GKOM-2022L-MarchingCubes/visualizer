use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_flycam::MovementSettings;

pub struct SpeedControllerSettings {
    modes: &'static [f32],
}

impl Default for SpeedControllerSettings {
    fn default() -> Self {
        Self {
            modes: &[12.0, 6.0, 3.0, 1.5, 0.75],
        }
    }
}

#[derive(Default)]
pub struct SpeedControllerState {
    mode: usize,
}

fn update_speed(
    settings: Res<SpeedControllerSettings>,
    mut state: ResMut<SpeedControllerState>,
    mut speed_settings: ResMut<MovementSettings>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let len = settings.modes.len() as isize;
    for event in mouse_wheel_events.iter() {
        state.mode = (state.mode as isize - event.y as isize).clamp(0, len - 1) as usize;
        speed_settings.speed = settings.modes[state.mode];
    }
}

pub struct SpeedController;

impl Plugin for SpeedController {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeedControllerSettings>()
            .init_resource::<SpeedControllerState>()
            .add_system(update_speed);
    }
}
