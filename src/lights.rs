use bevy::{
    prelude::{shape::Icosphere, *},
    render::render_resource::Face,
};

#[derive(Component)]
struct LightCenter;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh_handle = meshes.add(
        Icosphere {
            radius: 0.5,
            subdivisions: 16,
        }
        .into(),
    );
    let mut material: StandardMaterial = Color::rgb(1.0, 1.0, 0.7).into();
    material.unlit = true;
    let material_handle = materials.add(material);
    commands
        .spawn_bundle((
            LightCenter,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .with_children(|b| {
            b.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(3.0, 3.0, 3.0)),
                ..Default::default()
            });
            b.spawn_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(3.0, 3.0, 3.0)),
                ..Default::default()
            });
            b.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, -3.0, -3.0)),
                ..Default::default()
            });
            b.spawn_bundle(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(Vec3::new(-3.0, -3.0, -3.0)),
                ..Default::default()
            });
        });
}

fn rotate(mut query: Query<&mut Transform, With<LightCenter>>) {
    for mut t in query.iter_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Y, 0.01));
    }
}

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(rotate);
    }
}
