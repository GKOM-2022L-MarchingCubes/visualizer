use bevy::prelude::*;

pub struct VariableTransparencySettings {
    kb_change: KeyCode,
    modes: &'static [f32],
}

impl Default for VariableTransparencySettings {
    fn default() -> Self {
        Self {
            kb_change: KeyCode::T,
            modes: &[1.0, 0.5, 0.0],
        }
    }
}

#[derive(Default)]
pub struct VariableTransparencyState {
    mode: usize,
}

#[derive(Component)]
pub struct VariableTransparency;

fn update_transparency(
    settings: Res<VariableTransparencySettings>,
    mut state: ResMut<VariableTransparencyState>,
    keys: Res<Input<KeyCode>>,
    entities: Query<&Handle<StandardMaterial>, With<VariableTransparency>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(settings.kb_change) {
        state.mode = (state.mode + 1) % settings.modes.len();
        let mode = settings.modes[state.mode];
        info!("Changed mode to {} transparency.", mode);
        for handle in entities.iter() {
            debug!("Handle {:?}", handle);
            if let Some(mut material) = materials.get_mut(handle) {
                material.base_color = Color::rgba(1.0, 1.0, 1.0, mode);
                material.double_sided = true;
            }
        }
    }
}

pub struct VariableTransparencyPlugin;

impl Plugin for VariableTransparencyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VariableTransparencySettings>()
            .init_resource::<VariableTransparencyState>()
            .add_system(update_transparency);
    }
}
