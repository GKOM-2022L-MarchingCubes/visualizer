mod flashlight;
//mod skybox;
mod speed_controller;
mod transparency;
mod wireframe;

use bevy::{
    asset::AssetServerSettings,
    math::Vec3A,
    prelude::{shape::Icosphere, *},
    render::primitives::Sphere,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_obj::*;
use flashlight::{FlashlightHolder, FlashlightPlugin};
//use skybox::SkyboxMaterial;
use speed_controller::SpeedController;

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
        //.add_plugin(VariableTransparencyPlugin)
        .add_plugin(SpeedController)
        .add_plugin(ObjPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut skybox_materials: ResMut<Assets<SkyboxMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("mesh.obj"),
            material: std_materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("mesh.png")),
                ..Default::default()
            }),
            ..Default::default()
        })
        //.insert(VariableTransparency)
        .insert(ToggleWireframe);
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 3.0, 3.0)),
        ..Default::default()
    });
    let sphere = meshes.add(
        Icosphere {
            radius: 0.5,
            subdivisions: 16,
        }
        .into(),
    );
    let white_material = std_materials.add(Color::WHITE.into());
    commands.spawn_bundle(PbrBundle {
        mesh: sphere.clone(),
        material: white_material.clone(),
        transform: Transform::from_translation(Vec3::new(3.0, 3.0, 3.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-3.0, -3.0, -3.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: sphere,
        material: white_material,
        transform: Transform::from_translation(Vec3::new(-3.0, -3.0, -3.0)),
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

    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Button 1",
            TextStyle {
                font: font_handle,
                font_size: 40.0,
                // Alpha channel of the color controls transparency.
                color: Color::rgba(1.0, 1.0, 1.0, 0.2),
            },
            Default::default(),
        ),
        ..default()
    });
    /*commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: asset_server.load("skybox.obj"),
        material: skybox_materials.add(SkyboxMaterial {
            texture: asset_server.load("skybox.png"),
        }),
        ..default()
    });*/
}

/*
spawn_bundle(PbrBundle {
    mesh: asset_server.load("skybox.obj"),
    material: materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("skybox.png")),
        cull_mode: Some(Face::Front),
        unlit: true,
        ..Default::default()
    }),
    ..Default::default()
})
.insert(NotShadowCaster)
.insert(NotShadowReceiver)
.insert(ToggleWireframe)
.insert(VariableTransparency);
*/
