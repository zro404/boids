use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use crate::{
    boids::{move_boids, simulate, spawn_boids},
    camera::{rotate_camera, BoidsCamera},
    constants::CAMERA_POS, spherecast::{calc_cast_sphere, spawn_cast_spheres},
};

mod boids;
mod camera;
mod constants;
mod spherecast;

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
        .add_systems(Startup, (calc_cast_sphere, spawn_boids).chain())
        .add_systems(Update, (simulate, move_boids).chain())
        // .add_systems(Startup, spawn_cast_spheres)
        .insert_resource(ClearColor(Color::srgb_u8(20, 20, 20)))
        .run();
}

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    // camera
    commands.spawn((
        BoidsCamera,
        Camera3d::default(),
        Transform::from_translation(CAMERA_POS).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // lighting
    ambient_light.brightness = 1000.0;
}
