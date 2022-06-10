use bevy::prelude::*;

pub struct FlashlightSettings {
    pub kb_toggle: KeyCode,
}

impl Default for FlashlightSettings {
    fn default() -> Self {
        Self {
            kb_toggle: KeyCode::F,
        }
    }
}

#[derive(Component, Default)]
pub struct FlashlightHolder {
    pub enabled: bool,
}

#[derive(Component)]
pub struct FlashlightLightSource;

fn update_flashlight(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    settings: Res<FlashlightSettings>,
    mut cam_query: Query<(Entity, &mut FlashlightHolder, Option<&Children>)>,
) {
    if keys.just_pressed(settings.kb_toggle) {
        if let Ok((entity, mut holder, children)) = cam_query.get_single_mut() {
            let mut entity = commands.entity(entity);
            holder.enabled = !holder.enabled;
            if holder.enabled {
                info!("Enabled flashlight.");
                entity.with_children(|b| {
                    b.spawn_bundle(PointLightBundle {
                        point_light: PointLight {
                            intensity: 100.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(FlashlightLightSource);
                });
            } else {
                info!("Disabled flashlight.");
                if let Some(children) = children {
                    entity.remove_children(children);
                    for child in children.iter() {
                        commands.entity(*child).despawn();
                    }
                }
            }
        }
    }
}

pub struct FlashlightPlugin;

impl Plugin for FlashlightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlashlightSettings>()
            .add_system(update_flashlight);
    }
}
