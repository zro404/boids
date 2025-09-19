use bevy::prelude::*;
use rand::random_range;

use crate::constants::{
    BOID_COUNT, BOID_DIAMETER, BOID_SPEED, HALF_WORLD_SIZE, WORLD_SIZE, WORLD_VEC,
};

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
}

pub fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..BOID_COUNT {
        let mut vel = Vec3::ZERO;
        // let mut vel = Vec3::new(BOID_SPEED, BOID_SPEED, BOID_SPEED);
        let i = random_range(0..5);

        match i {
            0 => {
                vel.x = BOID_SPEED;
            }
            1 => {
                vel.y = BOID_SPEED;
            }
            2 => {
                vel.z = BOID_SPEED;
            }
            3 => {
                vel.x = -BOID_SPEED;
            }
            4 => {
                vel.y = -BOID_SPEED;
            }
            5 => {
                vel.z = -BOID_SPEED;
            }
            _ => {}
        }
        commands.spawn((
            Boid { velocity: vel },
            Mesh3d(meshes.add(Sphere::new(BOID_DIAMETER / 2.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
    }
}

pub fn move_boids(mut query: Query<(&mut Transform, &Boid), With<Boid>>) {
    for (mut t, b) in &mut query {
        if t.translation.x > HALF_WORLD_SIZE {
            t.translation.x -= WORLD_SIZE;
        } else if t.translation.x < -HALF_WORLD_SIZE {
            t.translation.x += WORLD_SIZE;
        }
        if t.translation.y > HALF_WORLD_SIZE {
            t.translation.y -= WORLD_SIZE;
        } else if t.translation.y < -HALF_WORLD_SIZE {
            t.translation.y += WORLD_SIZE;
        }
        if t.translation.z > HALF_WORLD_SIZE {
            t.translation.z -= WORLD_SIZE;
        } else if t.translation.z < -HALF_WORLD_SIZE {
            t.translation.z += WORLD_SIZE;
        }
        t.translation += b.velocity;
    }
}
