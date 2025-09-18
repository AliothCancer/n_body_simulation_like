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
    game_state: Res<GameState>,
    mut query: Query<(&Transform, &mut ExternalForce, &Cube)>,
    query_other: Query<(&Transform, &Cube)>,
) {
    if game_state.gravity_enabled {
        query.iter_mut().for_each(|(trans, mut ext_force, cube)| {
            ext_force.force = query_other
                .iter()
                .map(|(trans_other, cube_other)| {
                    let distance_vec = trans_other.translation - trans.translation;
                    let distance = distance_vec.length();
                    match distance {
                        1.0..60_f32 => calc_force(
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
}

fn calc_force(distance: f32, distance_vec: Vec3, energy_diff: f32) -> Vec3 {
    match distance {
        0.0 => panic!("distance must not be zero"),
        _ => distance.recip() * distance_vec * energy_diff * 0.0001 / (1.0 + distance),
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    gravity_enabled: bool,
}

pub fn toggle_forces(keyboard: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        game_state.gravity_enabled = !game_state.gravity_enabled;
        println!("Gravity: {}", game_state.gravity_enabled);
    }
}



pub fn update_cube_colors(
    mut cubes: Query<(&mut MeshMaterial3d<StandardMaterial>, &Cube), Changed<Cube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (material_handle, cube) in cubes.iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let color = iridescent_gradient(cube.energy);
            material.base_color = color;
        }
    }
}

fn iridescent_gradient(energy: f32) -> Color {
    // Normalizza energia (assumendo range 0-100)
    let t = (energy / 100.0).clamp(0.0, 1.0);
    
    // Usa HSV per creare gradiente iridescente
    // Hue va da 0 (rosso) a 300 (magenta), saltando il verde per più colori saturi
    let hue = t * 300.0; // 0° = rosso, 60° = giallo, 120° = verde, 180° = ciano, 240° = blu, 300° = magenta
    let saturation = 0.8; // Alta saturazione per colori vivaci
    let value = 0.9; // Luminosità alta
    
    Color::hsl(hue, saturation, value)
}