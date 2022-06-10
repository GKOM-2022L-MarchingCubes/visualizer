use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

pub struct ToggleWireframeSettings {
    pub kb_toggle: KeyCode,
}

impl Default for ToggleWireframeSettings {
    fn default() -> Self {
        Self {
            kb_toggle: KeyCode::G,
        }
    }
}

#[derive(Default)]
pub struct ToggleWireframeState {
    pub enabled: bool,
}

#[derive(Component)]
pub struct ToggleWireframe;

fn update_wireframe(
    mut commands: Commands,
    settings: Res<ToggleWireframeSettings>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<ToggleWireframeState>,
    entities: Query<Entity, With<ToggleWireframe>>,
) {
    if keys.just_pressed(settings.kb_toggle) {
        state.enabled = !state.enabled;
        if state.enabled {
            info!("Enabled wireframe.");
            for entity in entities.iter() {
                commands.entity(entity).insert(Wireframe);
            }
        } else {
            info!("Disabled wireframe.");
            for entity in entities.iter() {
                commands.entity(entity).remove::<Wireframe>();
            }
        }
    }
}

pub struct ToggleWireframePlugin;

impl Plugin for ToggleWireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WireframePlugin)
            .insert_resource(WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            })
            .init_resource::<ToggleWireframeSettings>()
            .init_resource::<ToggleWireframeState>()
            .add_system(update_wireframe);
    }
}
