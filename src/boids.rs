use std::f32::consts::{FRAC_PI_2, PI};

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
    // Bounding box
    // Make a box of planes facing inward
    let plane_mesh = meshes.add(Plane3d {
        half_size: Vec2::new(HALF_WORLD_SIZE, HALF_WORLD_SIZE),
        ..default()
    });
    let plane_material = materials.add(Color::srgba_u8(255, 255, 255, 10));
    let create_plane = move |translation, rotation| {
        (
            Transform::from_translation(translation * HALF_WORLD_SIZE)
                .with_rotation(Quat::from_scaled_axis(rotation)),
            Mesh3d(plane_mesh.clone()),
            MeshMaterial3d(plane_material.clone()),
        )
    };

    commands.spawn(create_plane(vec3(0.0, 1.0, 0.0), Vec3::X * PI));
    commands.spawn(create_plane(vec3(0.0, -1.0, 0.0), Vec3::ZERO));
    commands.spawn(create_plane(vec3(1.0, 0.0, 0.0), Vec3::Z * FRAC_PI_2));
    commands.spawn(create_plane(vec3(-1.0, 0.0, 0.0), Vec3::Z * -FRAC_PI_2));
    commands.spawn(create_plane(vec3(0.0, 0.0, 1.0), Vec3::X * -FRAC_PI_2));
    commands.spawn(create_plane(vec3(0.0, 0.0, -1.0), Vec3::X * FRAC_PI_2));

    for _ in 0..BOID_COUNT {
        let vel = gen_boid_init_vel();
        commands.spawn((
            Boid { velocity: vel },
            Mesh3d(meshes.add(Cone::new(BOID_DIAMETER/2.0, BOID_DIAMETER*1.5))),
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

        // let filter = | pos: Vec3 | pos.x > HALF_WORLD_SIZE || pos.x < -HALF_WORLD_SIZE || pos.y > HALF_WORLD_SIZE || pos.y < -HALF_WORLD_SIZE || pos.z > HALF_WORLD_SIZE || pos.z < -HALF_WORLD_SIZE;

        t.translation += b.velocity;
        t.rotation = Quat::from_rotation_arc(Vec3::Y, b.velocity);
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
                t_sum -= t1.translation.normalize();
            }

            if dist <= AVOIDANCE_THRESHOLD {
                let dir = t1.translation - t.translation;
                let dot = Vec3::dot(dir.normalize(), b.velocity.normalize());
                // Omit boid not in vision
                if dot > -0.5 {
                    c -= t1.translation - t.translation;
                }
            }
        }
        let mean_v = (v_sum.normalize()) * BOID_SPEED;

        let mut new_vel = b.velocity.lerp(mean_v, 0.1);
        new_vel += c;
        new_vel += b.velocity.lerp(t_sum, 0.0003);

        vel_vec.push(new_vel.normalize() * BOID_SPEED);
    }

    let mut i: usize = 0;
    for (_, mut b) in &mut query {
        b.velocity = vel_vec[i];
        i += 1;
    }
}
