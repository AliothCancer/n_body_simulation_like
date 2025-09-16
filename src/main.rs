mod cube;

use bevy::prelude::*;
use cube::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cubes
    let cubes = [
        Cube::new(
            1.0,
            10.0,
            Vec3 {
                x: -2.0,
                y: 0.5,
                z: 0.0,
            },
        ),
        Cube::new(
            1.0,
            20.0,
            Vec3 {
                x: 2.0,
                y: 0.5,
                z: 0.0,
            },
        ),
    ];
    cubes
        .into_iter()
        .for_each(|c| c.spawn(&mut commands, &mut meshes, &mut materials));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
