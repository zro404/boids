use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Quat, Vec3, VectorSpace},
    transform::components::Transform,
};

const CAM_SPEED: f32 = 0.02;

#[derive(Component)]
pub struct BoidsCamera;

pub fn rotate_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<BoidsCamera>>,
) {
    for mut transform in &mut camera_query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-CAM_SPEED));
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(CAM_SPEED));
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(CAM_SPEED));
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-CAM_SPEED));
        };
    }
}
