// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

pub fn update_shield_regen(
    mut state: ResMut<ShipState>,
    time: Res<Time>,
    paused: Res<Paused>,
) {
    if !state.alive || paused.0 { return; }
    if state.shield < SHIELD_MAX {
        state.shield = (state.shield + SHIELD_REGEN_RATE * time.delta_secs()).min(SHIELD_MAX);
    }
}

pub fn update_hit_flash(
    mut hit_flash: ResMut<HitFlash>,
    mut flash_q: Query<&mut BackgroundColor, With<HitFlashOverlay>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if hit_flash.timer <= 0.0 {
        // Remove overlay if exists and timer done
        for mut bg in flash_q.iter_mut() { bg.0 = Color::NONE; }
        return;
    }
    hit_flash.timer = (hit_flash.timer - time.delta_secs()).max(0.0);
    let alpha = (hit_flash.timer / HIT_FLASH_DURATION).clamp(0.0, 1.0) * 0.25;
    if flash_q.is_empty() {
        // Spawn screen-space red overlay
        commands.spawn((HitFlashOverlay, Node {
            position_type: PositionType::Absolute, width: Val::Percent(100.0),
            height: Val::Percent(100.0), ..default()
        }, BackgroundColor(Color::srgba(0.9, 0.1, 0.05, alpha)), ZIndex(5)));
    } else {
        for mut bg in flash_q.iter_mut() { bg.0 = Color::srgba(0.9, 0.1, 0.05, alpha); }
    }
}

/// Near-miss teal flash on ship (brief shield absorption effect).
pub fn update_near_miss_flash(
    mut near_miss_flash: ResMut<NearMissFlash>,
    ship_q: Query<Entity, With<Ship>>,
    children_q: Query<&Children>,
    material_q: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if near_miss_flash.timer <= 0.0 { return; }
    near_miss_flash.timer = (near_miss_flash.timer - time.delta_secs()).max(0.0);
    let flash_t = near_miss_flash.timer / NEAR_MISS_FLASH_DURATION;

    let Ok(ship_entity) = ship_q.single() else { return; };
    let mut stack = vec![ship_entity];
    while let Some(entity) = stack.pop() {
        if let Ok(mat_handle) = material_q.get(entity) {
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                let (r, g, b) = NEAR_MISS_FLASH_COLOR;
                let intensity = flash_t * 6.0;
                mat.emissive = LinearRgba::new(r * intensity, g * intensity, b * intensity, 1.0);
            }
        }
        if let Ok(children) = children_q.get(entity) {
            for child in children.iter() { stack.push(child); }
        }
    }
}

pub fn update_screen_shake(
    mut shake: ResMut<ScreenShake>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) -> Result {
    let dt = time.delta_secs();
    if shake.intensity > 0.01 {
        let rng = || (rand::random::<f32>() - 0.5) * 2.0;
        shake.offset = Vec3::new(rng() * shake.intensity, rng() * shake.intensity, 0.0);
        shake.intensity *= (-SCREEN_SHAKE_DECAY * dt).exp();
    } else {
        shake.offset = Vec3::ZERO;
        shake.intensity = 0.0;
    }

    let mut cam_tf = camera_q.single_mut()?;
    cam_tf.translation.x = shake.offset.x;
    cam_tf.translation.y = shake.offset.y;
    Ok(())
}

pub fn spawn_bars(commands: &mut Commands, font: Handle<Font>) {
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(BAR_MARGIN_PX),
        top: Val::Px(BAR_TOP_PX),
        bottom: Val::Px(BAR_BOTTOM_PX),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(BAR_GAP_PX),
        align_items: AlignItems::Stretch,
        ..default()
    }).with_children(|parent| {
        spawn_bar(parent, true, &font);
        spawn_bar(parent, false, &font);
    });
}

fn spawn_bar(parent: &mut ChildSpawnerCommands, is_shield: bool, font: &Handle<Font>) {
    let label = if is_shield { "S" } else { "L" };
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        width: Val::Px(BAR_WIDTH_PX),
        ..default()
    }).with_children(|col| {
        col.spawn((
            Text::new(label),
            TextFont { font: font.clone(), font_size: HUD_FONT, ..default() },
            TextColor(Color::srgba(HUD_LABEL_COLOR.0, HUD_LABEL_COLOR.1, HUD_LABEL_COLOR.2, HUD_LABEL_COLOR.3)),
        ));
        col.spawn((Node {
            width: Val::Px(BAR_WIDTH_PX),
            flex_grow: 1.0,
            border: UiRect::all(Val::Px(BAR_STROKE_PX)),
            margin: UiRect::top(Val::Px(4.0)),
            ..default()
        }, BackgroundColor(Color::srgba(BAR_BG_COLOR.0, BAR_BG_COLOR.1, BAR_BG_COLOR.2, BAR_BG_COLOR.3)),
           BorderColor::all(Color::srgba(BAR_STROKE_COLOR.0, BAR_STROKE_COLOR.1, BAR_STROKE_COLOR.2, BAR_STROKE_COLOR.3)),
        )).with_children(|bar| {
            let full_color = if is_shield { SHIELD_FULL_COLOR } else { LIFE_FULL_COLOR };
            bar.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            }).with_children(|fill_parent| {
                if is_shield {
                    fill_parent.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0),
                            border_radius: BorderRadius::all(Val::Px(3.0)), ..default() },
                        BackgroundColor(Color::srgb(full_color.0, full_color.1, full_color.2)),
                        ShieldBarFill,
                    ));
                } else {
                    fill_parent.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0),
                            border_radius: BorderRadius::all(Val::Px(3.0)), ..default() },
                        BackgroundColor(Color::srgb(full_color.0, full_color.1, full_color.2)),
                        LifeBarFill,
                    ));
                }
            });
        });
    });
}

pub fn update_bars(
    state: Res<ShipState>,
    mut shield_q: Query<(&mut Node, &mut BackgroundColor), (With<ShieldBarFill>, Without<LifeBarFill>)>,
    mut life_q: Query<(&mut Node, &mut BackgroundColor), (With<LifeBarFill>, Without<ShieldBarFill>)>,
) -> Result {
    let shield_pct = (state.shield / SHIELD_MAX).clamp(0.0, 1.0);
    let (mut s_node, mut s_bg) = shield_q.single_mut()?;
    let new_h = Val::Percent(shield_pct * 100.0);
    if s_node.height != new_h { s_node.height = new_h; }
    let t = shield_pct;
    let new_color = Color::srgb(
        SHIELD_LOW_COLOR.0 + (SHIELD_FULL_COLOR.0 - SHIELD_LOW_COLOR.0) * t,
        SHIELD_LOW_COLOR.1 + (SHIELD_FULL_COLOR.1 - SHIELD_LOW_COLOR.1) * t,
        SHIELD_LOW_COLOR.2 + (SHIELD_FULL_COLOR.2 - SHIELD_LOW_COLOR.2) * t,
    );
    if s_bg.0 != new_color { s_bg.0 = new_color; }

    let life_pct = (state.life / LIFE_MAX).clamp(0.0, 1.0);
    let (mut l_node, mut l_bg) = life_q.single_mut()?;
    let new_h = Val::Percent(life_pct * 100.0);
    if l_node.height != new_h { l_node.height = new_h; }
    let t = life_pct;
    let new_color = Color::srgb(
        LIFE_LOW_COLOR.0 + (LIFE_FULL_COLOR.0 - LIFE_LOW_COLOR.0) * t,
        LIFE_LOW_COLOR.1 + (LIFE_FULL_COLOR.1 - LIFE_LOW_COLOR.1) * t,
        LIFE_LOW_COLOR.2 + (LIFE_FULL_COLOR.2 - LIFE_LOW_COLOR.2) * t,
    );
    if l_bg.0 != new_color { l_bg.0 = new_color; }
    Ok(())
}
