use bevy::{ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*};
use bevy_4x_camera::{CameraRig, CameraRigBundle, FourXCameraPlugin, KeyboardConf};

fn main() {
    App::new()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugins(DefaultPlugins)
        .add_plugin(FourXCameraPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(CameraRigBundle {
            camera_rig: CameraRig {
                keyboard: KeyboardConf {
                    move_sensitivity: (1., 1.),
                    rotate_sensitivity: 0.5,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|cb| {
            cb.spawn_bundle(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(-20.0, 20., 0.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
        });
}
