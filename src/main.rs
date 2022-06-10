mod arrow;
mod flashlight;
mod lights;
mod skybox;
mod speed_controller;
mod transparency;
mod wireframe;

use arrow::AxisArrowsPlugin;
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_obj::*;
use flashlight::{FlashlightHolder, FlashlightPlugin};
use lights::LightsPlugin;
use skybox::{SkyboxCenter, SkyboxPlugin};
use speed_controller::SpeedController;

use wireframe::{ToggleWireframe, ToggleWireframePlugin};

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<MovementSettings>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_assets)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(FlashlightPlugin)
        .add_plugin(ToggleWireframePlugin)
        .add_plugin(SpeedController)
        .add_plugin(ObjPlugin)
        .add_plugin(SkyboxPlugin)
        .add_plugin(LightsPlugin)
        .add_plugin(AxisArrowsPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .run();
}

#[derive(Component)]
struct Watcher(Handle<Mesh>, Handle<Image>);

#[derive(Component)]
struct Watched;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    let mesh = asset_server.load("mesh.obj");
    let image = asset_server.load("mesh.png");

    commands.spawn_bundle((Watcher(mesh, image),));

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(FlashlightHolder {
            ..Default::default()
        })
        .insert(SkyboxCenter);
}

fn update_assets(
    mut commands: Commands,
    watcher: Query<&Watcher, (With<Watcher>, Without<Watched>)>,
    watched: Query<Entity, (With<Watched>, Without<Watcher>)>,
    mut mesh_events: EventReader<AssetEvent<Mesh>>,
    mut image_events: EventReader<AssetEvent<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut reload = false;
    let watcher = watcher.single();
    if !mesh_events.is_empty() {
        for e in mesh_events.iter() {
            let h = match e {
                AssetEvent::Created { handle: h } => h,
                AssetEvent::Modified { handle: h } => h,
                _ => continue,
            };
            if h == &watcher.0 {
                reload = true;
            }
        }
    }
    if !image_events.is_empty() {
        for e in image_events.iter() {
            let h = match e {
                AssetEvent::Created { handle: h } => h,
                AssetEvent::Modified { handle: h } => h,
                _ => continue,
            };
            if h == &watcher.1 {
                reload = true;
            }
        }
    }
    if reload {
        if let Ok(watched) = watched.get_single() {
            commands.entity(watched).despawn();
        }

        commands
            .spawn_bundle(PbrBundle {
                mesh: watcher.0.clone(),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(watcher.1.clone()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(ToggleWireframe)
            .insert(Watched);
    }
}
