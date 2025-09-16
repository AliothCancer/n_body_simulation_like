use bevy::prelude::*;

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
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(self.size, self.size, self.size))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_translation(self.position),
        ));
    }
}
