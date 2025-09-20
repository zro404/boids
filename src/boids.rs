use bevy::prelude::*;
use rand::random;

use crate::constants::{
    AVOIDANCE_THRESHOLD, BOID_COUNT, BOID_DIAMETER, BOID_SPEED, HALF_WORLD_SIZE,
    LOCAL_FLOCK_RADIUS, WORLD_SIZE,
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
        let vel = gen_boid_init_vel();
        commands.spawn((
            Boid { velocity: vel },
            Mesh3d(meshes.add(Sphere::new(BOID_DIAMETER / 2.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            // Transform::from_xyz(0.0, 0.5, 0.0),
            Transform {
                translation: gen_boid_init_pos(),
                ..default()
            },
        ));
    }
}

fn gen_boid_init_vel() -> Vec3 {
    let unit = || {
        return random::<f32>() * 2.0 - 1.0;
    };
    let mut v = Vec3::new(unit(), unit(), unit()).normalize();
    v *= BOID_SPEED;
    return v;
}

fn gen_boid_init_pos() -> Vec3 {
    let unit = || {
        return random::<f32>() * WORLD_SIZE - HALF_WORLD_SIZE;
    };
    return Vec3::new(unit(), unit(), unit());
}

pub fn move_boids(mut query: Query<(&mut Transform, &mut Boid), With<Boid>>) {
    for (mut t, mut b) in &mut query {
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

        // if t.translation.x > HALF_WORLD_SIZE || t.translation.x < -HALF_WORLD_SIZE {
        //     b.velocity.x = -b.velocity.x;
        // }
        // if t.translation.y > HALF_WORLD_SIZE || t.translation.y < -HALF_WORLD_SIZE {
        //     b.velocity.y = -b.velocity.y;
        // }
        // if t.translation.z > HALF_WORLD_SIZE || t.translation.z < -HALF_WORLD_SIZE {
        //     b.velocity.z = -b.velocity.z;
        // }

        t.translation += b.velocity;
    }
}

pub fn simulate(mut query: Query<(&mut Transform, &mut Boid), With<Boid>>) {
    let mut vel_vec = Vec::<Vec3>::new();

    for (t, b) in &query {
        let mut v_sum = Vec3::ZERO;
        let mut c = Vec3::ZERO;
        let mut t_sum = Vec3::ZERO;

        for (t1, b1) in &query {
            let dist = t.translation.distance(t1.translation);

            if dist <= LOCAL_FLOCK_RADIUS {
                v_sum += b1.velocity.normalize();
                t_sum += t1.translation.normalize();
            }

            if dist <= AVOIDANCE_THRESHOLD {
                c -= t1.translation - t.translation;
            }
        }
        let mean_v = (v_sum.normalize()) * BOID_SPEED;

        let mut new_vel = b.velocity.lerp(mean_v, 0.1);
        new_vel += c;
        new_vel += b.velocity.lerp(t_sum, 0.1);

        vel_vec.push(new_vel.normalize() * BOID_SPEED);
    }

    let mut i: usize = 0;
    for (_, mut b) in &mut query {
        b.velocity = vel_vec[i];
        i += 1;
    }
}
