use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use crate::camera::{BoidsCamera, rotate_camera};

mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 15.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                },
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .insert_resource(ClearColor(Color::srgb_u8(20, 20, 20)))
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
    mut ambient_light: ResMut<AmbientLight>,
) {
    // camera
    commands.spawn((
        BoidsCamera,
        Camera3d::default(),
        Transform::from_xyz(200.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // lighting
    ambient_light.brightness = 1000.0;

    // bounding box
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(100.0, 100.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 10))),
    ));

    // boid
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));
}
