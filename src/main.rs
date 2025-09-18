mod cube;
mod physical_bounds;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;
use cube::*;

use crate::physical_bounds::spawn_bounds;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FlyCameraPlugin)
        .add_systems(Update, apply_boundary_forces)
        //.add_systems(Update, apply_cube_forces)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut window: Query<&mut Window>,
) {


    (window.single_mut().unwrap().cursor_options.visible) = false;


    // SPAWNING INVISIBLE BOUNDS
    spawn_bounds(&mut commands);

    // cubes
    let cubes = (1..=4)
        .map(|i| {
            let k = 2;
            let rnd_val = rand::random_range(0_f32..20_f32);
            let x = (k % i) as f32;
            let y = (k % i) as f32;
            let z = 1.0;
            Cube::new(1.0, rnd_val, Vec3 { x, y, z })
        })
        .collect::<Vec<_>>();
    
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
        FlyCamera {
            sensitivity: 6.0,
            ..Default::default()
        },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            Vec3::Y,
        ),
    ));
}


// let cubes = [
    //     Cube::new(
    //         1.0,
    //         14.0,
    //         Vec3 {
    //             x: -2.0,
    //             y: 1.5,
    //             z: 0.0,
    //         },
    //     ),
    //     Cube::new(
    //         1.0,
    //         30.0,
    //         Vec3 {
    //             x: 2.0,
    //             y: 2.0,
    //             z: 0.0,
    //         },
    //     ),
    //     Cube::new(
    //         1.0,
    //         20.0,
    //         Vec3 {
    //             x: 7.0,
    //             y: 2.0,
    //             z: 0.0,
    //         },
    //     ),
    //     Cube::new(
    //         1.0,
    //         10.0,
    //         Vec3 {
    //             x: 4.0,
    //             y: 2.0,
    //             z: 0.0,
    //         },
    //     ),
    // ];