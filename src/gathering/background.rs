// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
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
        let mesh = meshes.add(Cuboid::new(size, size, size));
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(brightness, brightness, brightness * 1.1),
            unlit: true,
            ..default()
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

pub fn scroll_stars(
    mut query: Query<(&Star, &mut Transform)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    for (star, mut tf) in query.iter_mut() {
        let speed = STAR_LAYER_SPEEDS[star.layer] * SCROLL_SPEED;
        tf.translation.y -= speed * dt;
        let limit = bounds.half_height * 1.5;
        if tf.translation.y < -limit {
            tf.translation.y += limit * 2.0;
            tf.translation.x = rand::random::<f32>() * bounds.half_width * 3.0 - bounds.half_width * 1.5;
        }
    }
}
