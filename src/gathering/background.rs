// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::render::render_resource::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

pub fn spawn_stars(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    bounds: &ViewBounds,
) {
    let mut rng = rand::thread_rng();
    for layer in 0..NUM_STAR_LAYERS {
        let size = STAR_SIZES[layer];
        let brightness = STAR_BRIGHTNESS[layer];
        let depth = STAR_LAYER_DEPTHS[layer];
        // Spheres instead of cubes, emissive so they bloom
        let mesh = meshes.add(Sphere::new(size));
        let emissive = bevy::color::LinearRgba::new(
            brightness, brightness, brightness * 1.1, 1.0) * 3.0;
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(brightness, brightness, brightness * 1.1),
            emissive, unlit: true, ..default()
        });

        for _ in 0..STARS_PER_LAYER[layer] {
            let x = rng.gen_range(-bounds.half_width * 1.5..bounds.half_width * 1.5);
            let y = rng.gen_range(-bounds.half_height * 1.5..bounds.half_height * 1.5);
            commands.spawn((
                Star { layer },
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, y, depth),
            ));
        }
    }
}

/// Create a vignette texture for the Gathering (same as Bot Game)
pub fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
    let size = 256u32;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;
    let max_dist = center * 1.2;
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let t = (dist / max_dist).clamp(0.0, 1.0);
            let alpha = (t * t * 0.6 * 255.0) as u8;
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 0; data[idx + 1] = 0; data[idx + 2] = 0;
            data[idx + 3] = alpha;
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

pub fn spawn_nebula_planes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    bounds: &ViewBounds,
) {
    for &(z, alpha, r, g, b) in &NEBULA_CONFIGS {
        let tex = create_nebula_texture(images, r, g, b);
        let mesh = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(NEBULA_SIZE)));
        let mat = materials.add(StandardMaterial {
            base_color_texture: Some(tex),
            base_color: Color::srgba(1.0, 1.0, 1.0, alpha),
            emissive: LinearRgba::new(r, g, b, 1.0) * 0.5,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            double_sided: true,
            ..default()
        });
        // Two planes vertically for seamless scrolling
        for offset in [0.0, NEBULA_SIZE] {
            let x_off = (z * 0.3).sin() * 5.0;  // slight horizontal offset per layer
            commands.spawn((
                NebulaPlane,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_xyz(x_off, offset, z),
            ));
        }
    }
}

fn create_nebula_texture(images: &mut Assets<Image>, r: f32, g: f32, b: f32) -> Handle<Image> {
    let size = 128u32;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;
    let mut rng_val: u32 = ((r * 1000.0) as u32).wrapping_mul(7919) ^ ((b * 1000.0) as u32);
    for y in 0..size {
        for x in 0..size {
            let dx = (x as f32 - center) / center;
            let dy = (y as f32 - center) / center;
            let dist = (dx * dx + dy * dy).sqrt();
            // Radial gradient with simple noise
            rng_val = rng_val.wrapping_mul(1103515245).wrapping_add(12345);
            let noise = ((rng_val >> 16) as f32 / 65536.0) * 0.3;
            let gradient = (1.0 - dist).max(0.0);
            let value = (gradient * gradient + noise * gradient).clamp(0.0, 1.0);
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = (r * value * 255.0) as u8;
            data[idx + 1] = (g * value * 255.0) as u8;
            data[idx + 2] = (b * value * 255.0) as u8;
            data[idx + 3] = (value * 200.0) as u8;
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

pub fn scroll_stars(
    mut star_q: Query<(&Star, &mut Transform), Without<NebulaPlane>>,
    mut nebula_q: Query<&mut Transform, (With<NebulaPlane>, Without<Star>)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
    paused: Res<Paused>,
) {
    if !state.alive || paused.0 { return; }
    let dt = time.delta_secs();
    for (star, mut tf) in star_q.iter_mut() {
        let speed = STAR_LAYER_SPEEDS[star.layer] * SCROLL_SPEED;
        tf.translation.y -= speed * dt;
        let limit = bounds.half_height * 1.5;
        if tf.translation.y < -limit {
            tf.translation.y += limit * 2.0;
            tf.translation.x = rand::random::<f32>() * bounds.half_width * 3.0 - bounds.half_width * 1.5;
        }
    }

    // Scroll nebula planes
    let nebula_speed = SCROLL_SPEED * NEBULA_SCROLL_SPEED_MULT * dt;
    for mut tf in nebula_q.iter_mut() {
        tf.translation.y -= nebula_speed;
        if tf.translation.y < -NEBULA_SIZE {
            tf.translation.y += NEBULA_SIZE * 2.0;
        }
    }
}
