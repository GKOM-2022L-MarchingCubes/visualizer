mod flashlight;
mod transparency;
mod wireframe;

use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_obj::*;
use flashlight::{FlashlightHolder, FlashlightPlugin};
use transparency::{VariableTransparency, VariableTransparencyPlugin};
use wireframe::{ToggleWireframe, ToggleWireframePlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugin(NoCameraPlayerPlugin)
        .init_resource::<MovementSettings>()
        .add_plugin(FlashlightPlugin)
        .add_plugin(ToggleWireframePlugin)
        .add_plugin(VariableTransparencyPlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("cube.obj"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("cube.png")),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ToggleWireframe)
        .insert(VariableTransparency);
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(5.0, 5.0, 5.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-5.0, -5.0, -5.0)),
        ..Default::default()
    });
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(FlashlightHolder {
            ..Default::default()
        });
}
