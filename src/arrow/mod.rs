mod cone;

use bevy::prelude::{shape::Capsule, *};

use self::cone::Cone;

fn create_arrow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arrow_head = meshes.add(
        Cone {
            radius: 1.0,
            height: 4.0,
            subdivisions: 16,
        }
        .into(),
    );
    let arrow_shaft = meshes.add(
        Capsule {
            radius: 1.0,
            rings: 4,
            depth: 4.0,
            latitudes: 4,
            longitudes: 4,
            ..Default::default()
        }
        .into(),
    );
    let red = materials.add(Color::RED.into());
    let green = materials.add(Color::GREEN.into());
    let blue = materials.add(Color::BLUE.into());

    commands.spawn().with_children(|b| {
        b.spawn_bundle(PbrBundle {
            mesh: todo!(),
            material: todo!(),
            ..Default::default()
        });
    });
}

pub struct AxisArrowsPlugin;

impl Plugin for AxisArrowsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}
