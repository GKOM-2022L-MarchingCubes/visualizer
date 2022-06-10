mod cone;

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::{
    shape::{Capsule, Icosphere},
    *,
};

use self::cone::Cone;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arrow_head = meshes.add(
        Cone {
            radius: 0.5,
            height: 1.0,
            subdivisions: 16,
        }
        .into(),
    );
    let arrow_shaft = meshes.add(
        Capsule {
            radius: 0.2,
            rings: 4,
            depth: 3.0,
            latitudes: 4,
            longitudes: 10,
            ..Default::default()
        }
        .into(),
    );
    let mut red: StandardMaterial = Color::RED.into();
    red.unlit = true;
    let red_material = materials.add(red);

    let mut green: StandardMaterial = Color::GREEN.into();
    green.unlit = true;
    let green_material = materials.add(green);

    let mut blue: StandardMaterial = Color::BLUE.into();
    blue.unlit = true;
    let blue_material = materials.add(blue);

    let sphere = meshes.add(
        Icosphere {
            radius: 0.5,
            subdivisions: 16,
        }
        .into(),
    );
    let mut white: StandardMaterial = Color::WHITE.into();
    white.unlit = true;
    let white_material = materials.add(white);

    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::splat(-1.5)),
            GlobalTransform::default(),
        ))
        .with_children(|b| {
            b.spawn_bundle(PbrBundle {
                transform: Transform::default(),
                mesh: sphere.clone(),
                material: white_material.clone(),
                ..Default::default()
            });
            // X
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))
                    .with_translation(Vec3::new(1.5, 0.0, 0.0)),
                mesh: arrow_shaft.clone(),
                material: red_material.clone(),
                ..Default::default()
            });
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(-FRAC_PI_2))
                    .with_translation(Vec3::new(3.0, 0.0, 0.0)),
                mesh: arrow_head.clone(),
                material: red_material.clone(),
                ..Default::default()
            });
            // Y
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2))
                    .with_translation(Vec3::new(0.0, 1.5, 0.0)),
                mesh: arrow_shaft.clone(),
                material: green_material.clone(),
                ..Default::default()
            });
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_y(-FRAC_PI_2))
                    .with_translation(Vec3::new(0.0, 3.0, 0.0)),
                mesh: arrow_head.clone(),
                material: green_material.clone(),
                ..Default::default()
            });
            // Z
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
                    .with_translation(Vec3::new(0.0, 0.0, 1.5)),
                mesh: arrow_shaft.clone(),
                material: blue_material.clone(),
                ..Default::default()
            });
            b.spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
                    .with_translation(Vec3::new(0.0, 0.0, 3.0)),
                mesh: arrow_head.clone(),
                material: blue_material.clone(),
                ..Default::default()
            });
        });
}

pub struct AxisArrowsPlugin;

impl Plugin for AxisArrowsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup);
    }
}
