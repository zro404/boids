use bevy::math::Vec3;

pub const WORLD_SIZE: f32 = 100.0;

pub const CAM_SPEED: f32 = 0.02;

pub const BOID_COUNT: usize = 10;
pub const BOID_SPEED: f32 = 0.5;
pub const BOID_DIAMETER: f32 = 1.0;

// Following exist for reducing computations
pub const CAMERA_POS: Vec3 = Vec3::new(
    WORLD_SIZE + 100.0,
    (WORLD_SIZE + 100.0) / 2.0,
    (WORLD_SIZE + 100.0) / 2.0,
);

pub const HALF_WORLD_SIZE: f32 = (WORLD_SIZE + BOID_DIAMETER) / 2.0;
pub const WORLD_VEC: Vec3 = Vec3::new(WORLD_SIZE, WORLD_SIZE, WORLD_SIZE);
