use std::f32::consts::PI;

use bevy::{math::ops::sqrt, prelude::*};

#[derive(Component)]
pub struct CastSphereVecs(Vec<Vec3>);

pub fn spawn_cast_spheres(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const R: f32 = 50.0;

    // bounding box
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(R))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 10))),
    ));

    // Spawn Rays
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 0, 0, 255))),
        Transform::from_translation(Vec3::new(0.0, R, 0.0)),
    ));

    const N: f32 = 10000.0;
    let mut n_c = 0;
    let a = 4.0 * PI * R / N;
    let d = sqrt(a);
    let m_theta = f32::round(PI / d);
    let d_theta = PI / m_theta;
    let d_phi = a / d_theta;

    let mut m = 0.0;
    while m < m_theta*0.6 {
        let theta = PI * (m + 1.0) / m_theta; // Original -> m + 0.5
        let m_phi = f32::round(2.0 * PI * f32::sin(theta) / d_phi);
        let mut n = 0.0;
        while n < m_phi {
            let phi = 2.0 * PI * n / m_phi;

            let x =  f32::sin(theta) * f32::cos(phi);
            let z =  f32::sin(theta) * f32::sin(phi);
            let y =  f32::cos(theta);

            let pos = Vec3::new(R*x, R*y, R*z);

            // Spawn Points
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(1.0))),
                MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
                Transform::from_translation(pos),
            ));

            n_c += 1;
            n += 1.0;
        }
        m += 1.0;
    }
}

pub fn calc_cast_sphere(
    mut commands: Commands,
) {
    const R: f32 = 50.0;

    let mut castSphereVecs: Vec<Vec3> = Vec::new();
    castSphereVecs.push(Vec3::new(0.0, R, 0.0));

    const N: f32 = 10000.0;
    let a = 4.0 * PI * R / N;
    let d = sqrt(a);
    let m_theta = f32::round(PI / d);
    let d_theta = PI / m_theta;
    let d_phi = a / d_theta;

    let mut m = 0.0;
    while m < m_theta*0.6 {
        let theta = PI * (m + 1.0) / m_theta; // Original -> m + 0.5
        let m_phi = f32::round(2.0 * PI * f32::sin(theta) / d_phi);
        let mut n = 0.0;
        while n < m_phi {
            let phi = 2.0 * PI * n / m_phi;

            let x =  f32::sin(theta) * f32::cos(phi);
            let z =  f32::sin(theta) * f32::sin(phi);
            let y =  f32::cos(theta);

            let pos = Vec3::new(R*x, R*y, R*z);

            castSphereVecs.push(pos);

            n += 1.0;
        }
        m += 1.0;
    }

    commands.spawn(CastSphereVecs(castSphereVecs));
}
