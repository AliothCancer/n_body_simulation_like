use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const DEG_TO_RAD: f32 = 2.0 * PI / 360.0;

#[derive(Component)]
pub struct Cube {
    size: f32,
    energy: f32,
    position: Vec3,
}

impl Cube {
    pub fn new(size: f32, energy: f32, position: Vec3) -> Self {
        Self {
            size,
            energy,
            position,
        }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut position = Transform::from_translation(self.position);
        position.rotate_y(30.0 * DEG_TO_RAD);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(self.size, self.size, self.size))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            RigidBody::Dynamic,
            position,
            GravityScale(0.0),
            ExternalForce::default(),
            Collider::cuboid(self.size / 2.0, self.size / 2.0, self.size / 2.0),
            self,
        ));
    }
}

pub fn apply_cube_forces(
    mut query: Query<(&Transform, &mut ExternalForce, &Cube)>,
    query_other: Query<(&Transform, &Cube)>,
) {
    query.iter_mut().for_each(|(trans, mut ext_force, cube)| {
        ext_force.force = query_other
            .iter()
            .map(|(trans_other, cube_other)| {
                let distance_vec = trans_other.translation - trans.translation;
                let distance = distance_vec.length();
                match distance {
                    ..10_f32 => calc_force(
                        distance,
                        distance_vec,
                        (cube.energy - cube_other.energy).abs(),
                    ),
                    _ => Vec3::ZERO,
                }
            })
            .sum::<Vec3>();
    });
}

fn calc_force(distance: f32, distance_vec: Vec3, energy_diff: f32) -> Vec3 {
    match distance {
        _ => distance.recip() * distance_vec * energy_diff / (1.0 + distance),
        0.0 => panic!("distance must not be zero")  
    }
}

pub fn apply_boundary_forces(mut query: Query<(&Transform, &mut ExternalForce), With<Cube>>) {
    let bounds = Vec3::new(50.0, 50.0, 50.0);
    let boundary_strength = 100.0;

    for (transform, mut force) in query.iter_mut() {
        let pos = transform.translation;
        let mut boundary_force = Vec3::ZERO;

        // Forza repulsiva per ogni asse
        if pos.x > bounds.x * 0.8 {
            boundary_force.x -= boundary_strength * (pos.x - bounds.x * 0.8).powi(2);
        } else if pos.x < -bounds.x * 0.8 {
            boundary_force.x += boundary_strength * (-pos.x - bounds.x * 0.8).powi(2);
        }

        // Ripeti per Y e Z...

        force.force += boundary_force;
    }
}
