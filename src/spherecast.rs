use std::{f32::consts::PI};

use bevy::{math::ops::sqrt, prelude::*};

use crate::{constants::{WORLD_SIZE}};


pub fn spawn_sphere(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const R: f32 = WORLD_SIZE/2.0;

    // bounding box
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(R))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 10))),
    ));

    // Spawn Rays
    const N: f32 = 50000.0;
    let mut n_c = 0;
    let a = 4.0*PI*R/N;
    let d = sqrt(a);
    let m_theta = f32::round(PI/d);
    let d_theta = PI/m_theta;
    let d_phi = a/d_theta;
    let mut m = 0.0;
    while m <= m_theta{
        let theta = PI*(m+0.5)/m_theta;
        let m_phi = f32::round(2.0*PI*f32::sin(theta)/d_phi);
        let mut n = 0.0;
        while n <= m_phi {
            let phi = 2.0*PI*n/m_phi;
            // println!("yes");
            // Create Points
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(1.0))),
                MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
                Transform::from_xyz(R*f32::sin(theta)*f32::cos(phi), R*f32::sin(theta)*f32::sin(phi), R*f32::cos(theta))
            ));
            n_c += 1;
            n += 1.0;
        }
        m += 1.0;
    }
}
