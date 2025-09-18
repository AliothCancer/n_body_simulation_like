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
        .init_resource::<GameState>()
        .add_systems(Update, toggle_forces)
        .add_systems(Update, apply_cube_forces)
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
    let cube_array_size = 4;
    let iterator_x = 1..=cube_array_size;
    let iterator_z = iterator_x.clone();
    let iterator_y = iterator_x.clone();

    let cubes = iterator_x.clone().flat_map(|x| {
        let x = x % cube_array_size;
        iterator_z.clone().flat_map({
            let value = iterator_y.clone();
            move |z| {
                let z = z % cube_array_size;

                value.clone().map(move |y| {
                    let position = vec3(x as f32, (y % cube_array_size) as f32, z as f32);

                    let rnd_val = rand::random_range(0_f32..20_f32);
                    Cube::new(0.2, rnd_val, position)
                })
            }
        })
    });

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
