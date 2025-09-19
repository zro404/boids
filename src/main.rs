use bevy::prelude::*;

use crate::camera::{BoidsCamera, rotate_camera};

mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Component)]
struct Boid {}

#[derive(Component)]
struct Direction {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        BoidsCamera,
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    // boid
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(1.0, 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(1.0, 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));
}
