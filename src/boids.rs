use bevy::{math::VectorSpace, prelude::*, reflect::EnumInfo};
use rand::random;

use crate::constants::{BOID_COUNT, BOID_DIAMETER, BOID_SPEED, HALF_WORLD_SIZE, WORLD_SIZE};

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
            Transform::from_xyz(0.0, 0.5, 0.0),
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

pub fn simulate(mut query: Query<(&mut Transform, &mut Boid), With<Boid>>) {
    let mut velVec = Vec::<Vec3>::new();
    for (t, b) in &query {
        let mut v_sum = Vec3::ZERO;
        for (t1, b1) in &query {
            v_sum += b1.velocity.normalize();
        }
        let mean_v = (v_sum.normalize()) * BOID_SPEED;
        velVec.push(b.velocity.lerp(mean_v, 0.001));
        // b.velocity = b.velocity.lerp(mean_v, 0.001);

        // println!("{} {} {}", v_sum, mean_v, b.velocity.lerp(mean_v, 0.001));
    }
    let mut i: usize = 0;
    for (mut t, mut b) in &mut query {
        b.velocity = velVec[i];
        i += 1;
    }

    // let mut iter = query.iter_mut();
    // while let Some((mut t, mut b)) = iter.next() {
    //     let mut v_sum = Vec3::ZERO;
    //     for (t1, b1) in iter.remaining_mut() {
    //         v_sum += b1.velocity;
    //     }
    //     let mean_v = ((v_sum).normalize()) * BOID_SPEED;
    //     println!("{} {} {}", v_sum, mean_v, b.velocity.lerp(mean_v, 0.001));
    //     // break;
    //     // b.velocity = b.velocity.lerp(mean_v, 0.001);
    //     // b.velocity = b.velocity.lerp(mean_v, 0.001);
    // }

    // let mut entities: Vec<(Mut<Transform>, Mut<Boid>)> = query.iter_mut().collect();
    // for i in 0..BOID_COUNT {
    // let (left_vec, right_vec) = entities.split_at_mut(i);
    // do calc for left sub vec
    // b.velocity = alignment(&b.velocity, left_vec);

    // do calc for right sub vec
    // b.velocity = alignment(&b.velocity, right_vec);
    // }
    // while let Some([(mut t, mut b), (mut t1, mut b1)]) = combinations.fetch_next() {}
    // let mut v_sum = Vec3::ZERO;
    // for (t, b) in &mut query {
    //     v_sum += b.velocity.normalize();
    // }
    // let mean_v = ((v_sum / (BOID_COUNT as f32)).normalize()) * BOID_SPEED;
    // for (mut t, mut b) in &mut query {
    //     b.velocity = b.velocity.lerp(mean_v, 0.001);
    // }
}
