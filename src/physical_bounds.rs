use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_bounds(commands: &mut Commands) {
    let bounds_size = 50.0;

    // Crea muri invisibili
    commands.spawn((
        Transform::from_xyz(bounds_size, 0.0, 0.0),
        Collider::cuboid(10.0, bounds_size, bounds_size),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Transform::from_xyz(-bounds_size, 0.0, 0.0),
        Collider::cuboid(10.0, bounds_size, bounds_size),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Transform::from_xyz(0.0, bounds_size, 0.0),
        Collider::cuboid(bounds_size, 10.0, bounds_size),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -bounds_size, 0.0),
        Collider::cuboid(bounds_size, 10.0, bounds_size),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, bounds_size),
        Collider::cuboid(bounds_size, bounds_size, 10.0),
        RigidBody::Fixed,
    ));
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -bounds_size),
        Collider::cuboid(bounds_size, bounds_size, 10.0),
        RigidBody::Fixed,
    ));
}
