use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::render_resource::Face,
};

#[derive(Component)]
pub struct SkyboxCenter;

#[derive(Component)]
pub struct Skybox;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_scale(Vec3::new(100.0, 100.0, 100.0)),
            mesh: asset_server.load("skybox.obj"),
            material: std_materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("skybox.png")),
                cull_mode: Some(Face::Front),
                unlit: true,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(NotShadowCaster)
        .insert(NotShadowReceiver)
        .insert(Skybox);
}

fn track(
    cam: Query<&Transform, With<SkyboxCenter>>,
    mut skybox: Query<&mut Transform, (With<Skybox>, Without<SkyboxCenter>)>,
) {
    let center = cam.get_single().unwrap();
    let mut t = skybox.get_single_mut().unwrap();
    t.translation = center.translation;
}

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(track);
    }
}
