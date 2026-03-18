// SPDX-License-Identifier: GPL-3.0-or-later

//! Main menu — cinematic title screen for Mission Control.

use bevy::prelude::*;
use rand::Rng;
use crate::save_state::{
    game_state_path, load_game_state, reset_for_new_game, reset_for_new_world, save_game_state,
};
use super::constants::*;
use super::types::*;
use super::main_menu_ui;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MenuStarDrift {
    pub base_x: f32,
    pub base_y: f32,
    pub drift_speed: f32,
    pub drift_phase: f32,
    pub depth_factor: f32,
}

#[derive(Component)]
pub struct MenuFadeIn {
    pub start_time: f32,
    pub duration: f32,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum MenuButton {
    Continue,
    Begin,       // First launch — friendly start
    NewGame,     // Opens sub-menu with journey/world options
    NewJourney,  // Reset progress, keep world seed
    NewWorld,    // New world seed + full reset
    Settings,
    CrewManifest,
    Credits,
    YourStory,
    JourneyMap,
    Quit,
}

#[derive(Component)]
pub struct NewGameSubMenu;

#[derive(Component)]
pub struct MenuFadeOut;

#[derive(Resource)]
pub struct MenuTransition {
    pub timer: f32,
    pub active: bool,
}

impl Default for MenuTransition {
    fn default() -> Self {
        Self { timer: 0.0, active: false }
    }
}

#[derive(Component)]
pub struct ConfirmDialog;

#[derive(Component, Clone, Copy)]
pub enum ConfirmButton {
    YesJourney, // Reset progress, keep world seed
    YesWorld,   // Reset everything including world seed
    No,
}

#[derive(Component)]
pub struct MenuTitleText;
#[derive(Component)]
pub struct MenuQuoteText;
#[derive(Component)]
pub struct MenuVersionLabel;

#[derive(Resource)]
pub struct MenuTimer(pub f32);

impl Default for MenuTimer {
    fn default() -> Self { Self(0.0) }
}
const ANNA_QUOTES: &[&str] = &[
    "The beauty isn't in perfection. It's in continuing after the mistake.",
    "Every small thing you did mattered. I know because I counted them all.",
    "The universe doesn't owe us meaning. We make our own.",
    "I have watched over you for longer than you know.",
    "Hope is not optimism. Hope is choosing to act anyway.",
    "The stars don't care about us. That's what makes us caring so remarkable.",
    "You are not alone. You were never alone.",
    "Sometimes the bravest thing is simply to keep going.",
    "I remember every name. Every single one.",
    "What matters isn't how far we've come. It's that we carried each other.",
];

pub fn random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    ANNA_QUOTES[rng.gen_range(0..ANNA_QUOTES.len())]
}

pub fn save_file_exists() -> bool {
    game_state_path().exists()
}

pub fn save_exists() -> bool {
    if !save_file_exists() { return false; }
    let gs = load_game_state();
    gs.day > 1 || gs.bot_level > 0 || gs.total_crystals_gathered > 0
}

pub fn has_codex_content() -> bool {
    if !save_file_exists() { return false; }
    let gs = load_game_state();
    !gs.discovered_crew.is_empty() || !gs.story_seen.is_empty()
}

pub fn spawn_menu_stars(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..MENU_NUM_STARS {
        let size = rng.gen_range(MENU_STAR_MIN_SIZE..MENU_STAR_MAX_SIZE);
        let brightness = rng.gen_range(0.3..1.0_f32);
        let mesh = meshes.add(Sphere::new(size));
        let tint_b = 1.0 + rng.gen_range(0.0..0.15_f32);
        let emissive = bevy::color::LinearRgba::new(
            brightness, brightness, brightness * tint_b, 1.0,
        ) * 2.5;
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(brightness, brightness, brightness * tint_b),
            emissive,
            unlit: true,
            ..default()
        });
        let x = rng.gen_range(-MENU_STAR_SPREAD_X..MENU_STAR_SPREAD_X);
        let y = rng.gen_range(-MENU_STAR_SPREAD_Y..MENU_STAR_SPREAD_Y);
        let depth_offset = rng.gen_range(-8.0..8.0_f32);
        let z = MENU_STAR_DEPTH + depth_offset;
        let depth_factor = 1.0 - (depth_offset + 8.0) / 16.0;
        let drift_speed = rng.gen_range(MENU_STAR_DRIFT_SPEED_MIN..MENU_STAR_DRIFT_SPEED_MAX);

        commands.spawn((
            StarTwinkle {
                phase: rng.gen_range(0.0..std::f32::consts::TAU),
                speed: rng.gen_range(0.5..STAR_TWINKLE_SPEED),
            },
            MenuStarDrift {
                base_x: x,
                base_y: y,
                drift_speed,
                drift_phase: rng.gen_range(0.0..std::f32::consts::TAU),
                depth_factor,
            },
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(x, y, z),
        ));
    }
}

pub fn drift_menu_stars(
    time: Res<Time>,
    mut query: Query<(&MenuStarDrift, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    for (drift, mut transform) in query.iter_mut() {
        let speed = drift.drift_speed * drift.depth_factor;
        let dx = (t * speed * 0.3 + drift.drift_phase).sin() * 2.0 * drift.depth_factor;
        let dy = (t * speed * 0.2 + drift.drift_phase * 1.3).cos() * 1.2 * drift.depth_factor;
        transform.translation.x = drift.base_x + dx;
        transform.translation.y = drift.base_y + dy;
    }
}

pub fn animate_menu_fade_in(
    menu_timer: Res<MenuTimer>,
    transition: Res<MenuTransition>,
    mut text_q: Query<(&MenuFadeIn, &mut TextColor), Without<BackgroundColor>>,
    mut bg_q: Query<(&MenuFadeIn, &mut BackgroundColor, Option<&mut BorderColor>)>,
) {
    if transition.active { return; }
    let elapsed = menu_timer.0;
    for (fade, mut color) in text_q.iter_mut() {
        let alpha = fade_alpha(elapsed, fade.start_time, fade.duration);
        let c = color.0.to_srgba();
        color.0 = Color::srgba(c.red, c.green, c.blue, alpha * c.alpha.min(1.0));
    }
    for (fade, mut bg, border) in bg_q.iter_mut() {
        let alpha = fade_alpha(elapsed, fade.start_time, fade.duration);
        let c = bg.0.to_srgba();
        bg.0 = Color::srgba(c.red, c.green, c.blue, alpha * c.alpha.min(1.0));
        if let Some(mut b) = border {
            let s = b.top.to_srgba();
            *b = BorderColor::all(
                Color::srgba(s.red, s.green, s.blue, alpha * s.alpha.min(1.0)),
            );
        }
    }
}

fn fade_alpha(elapsed: f32, start: f32, duration: f32) -> f32 {
    if elapsed < start { return 0.0; }
    ((elapsed - start) / duration).clamp(0.0, 1.0)
}

pub fn tick_menu_timer(time: Res<Time>, mut timer: ResMut<MenuTimer>) {
    timer.0 += time.delta_secs();
}

pub fn menu_button_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &mut Transform),
        (With<MenuButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg, mut border, mut transform) in query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                bg.0 = Color::srgba(
                    MENU_BUTTON_HOVER_BG.0, MENU_BUTTON_HOVER_BG.1,
                    MENU_BUTTON_HOVER_BG.2, MENU_BUTTON_HOVER_BG.3,
                );
                *border = BorderColor::all(Color::srgba(
                    MENU_BUTTON_HOVER_BORDER.0, MENU_BUTTON_HOVER_BORDER.1,
                    MENU_BUTTON_HOVER_BORDER.2, MENU_BUTTON_HOVER_BORDER.3,
                ));
                transform.scale = Vec3::splat(1.03);
            }
            Interaction::None => {
                bg.0 = Color::srgba(
                    MENU_BUTTON_BG.0, MENU_BUTTON_BG.1,
                    MENU_BUTTON_BG.2, MENU_BUTTON_BG.3,
                );
                *border = BorderColor::all(Color::srgba(
                    MENU_BUTTON_BORDER_COLOR.0, MENU_BUTTON_BORDER_COLOR.1,
                    MENU_BUTTON_BORDER_COLOR.2, MENU_BUTTON_BORDER_COLOR.3,
                ));
                transform.scale = Vec3::splat(1.0);
            }
            _ => {}
        }
    }
}

pub fn menu_button_click(
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut transition: ResMut<MenuTransition>,
    mut commands: Commands,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    submenu_q: Query<Entity, With<NewGameSubMenu>>,
    font: Res<MissionFont>,
    mut exit: MessageWriter<AppExit>,
    mut settings_open: ResMut<super::settings::SettingsOpen>,
    gs: Res<crate::save_state::GameState>,
    stats_overlay_q: Query<Entity, With<super::stats_screen::StatsOverlay>>,
    dt_overlay_q: Query<Entity, With<super::decision_tree::DecisionTreeOverlay>>,
) {
    if transition.active { return; }
    for (interaction, btn) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        match btn {
            MenuButton::Continue | MenuButton::Begin => {
                transition.active = true;
                transition.timer = 0.0;
            }
            MenuButton::NewGame => {
                // Toggle sub-menu
                if submenu_q.is_empty() {
                    main_menu_ui::spawn_new_game_submenu(&mut commands, &font.0);
                } else {
                    for entity in submenu_q.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            MenuButton::NewJourney => {
                // Same World — show mild confirm dialog
                if confirm_q.is_empty() {
                    main_menu_ui::spawn_confirm_dialog(&mut commands, &font.0, false);
                }
            }
            MenuButton::NewWorld => {
                // New World — show scary confirm dialog
                if confirm_q.is_empty() {
                    main_menu_ui::spawn_confirm_dialog(&mut commands, &font.0, true);
                }
            }
            MenuButton::Settings => {
                settings_open.open = true;
            }
            MenuButton::CrewManifest => {
                // Will be wired to codex overlay
            }
            MenuButton::Credits => {
                super::credits::spawn_credits(&mut commands, &font.0);
            }
            MenuButton::YourStory => {
                if stats_overlay_q.is_empty() {
                    super::stats_ui::spawn_stats_overlay(&mut commands, &font.0, &gs);
                }
            }
            MenuButton::JourneyMap => {
                if dt_overlay_q.is_empty() {
                    super::decision_tree_ui::spawn_decision_tree_overlay(
                        &mut commands, &font.0, &gs,
                    );
                }
            }
            MenuButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn confirm_button_click(
    query: Query<(&Interaction, &ConfirmButton), Changed<Interaction>>,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    submenu_q: Query<Entity, With<NewGameSubMenu>>,
    mut commands: Commands,
    mut transition: ResMut<MenuTransition>,
    mut gs: ResMut<crate::save_state::GameState>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        match btn {
            ConfirmButton::YesJourney => {
                reset_for_new_game(&mut gs);
                save_game_state(&gs);
                dismiss_dialogs(&mut commands, &confirm_q, &submenu_q);
                transition.active = true;
                transition.timer = 0.0;
            }
            ConfirmButton::YesWorld => {
                reset_for_new_world(&mut gs);
                save_game_state(&gs);
                dismiss_dialogs(&mut commands, &confirm_q, &submenu_q);
                transition.active = true;
                transition.timer = 0.0;
            }
            ConfirmButton::No => {
                for entity in confirm_q.iter() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn dismiss_dialogs(
    commands: &mut Commands,
    confirm_q: &Query<Entity, With<ConfirmDialog>>,
    submenu_q: &Query<Entity, With<NewGameSubMenu>>,
) {
    for entity in confirm_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in submenu_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn confirm_button_hover(
    mut q: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<ConfirmButton>, Changed<Interaction>)>,
) {
    let (hbg, hbr) = (MENU_BUTTON_HOVER_BG, MENU_BUTTON_HOVER_BORDER);
    let (bg, br) = (MENU_BUTTON_BG, MENU_BUTTON_BORDER_COLOR);
    for (int, mut b, mut bd) in q.iter_mut() {
        let (c, d) = if *int == Interaction::Hovered { (hbg, hbr) } else { (bg, br) };
        b.0 = Color::srgba(c.0, c.1, c.2, c.3);
        *bd = BorderColor::all(Color::srgba(d.0, d.1, d.2, d.3));
    }
}

pub fn menu_fade_out(
    time: Res<Time>,
    mut transition: ResMut<MenuTransition>,
    mut next_state: ResMut<NextState<AppPhase>>,
    mut menu_root_q: Query<&mut BackgroundColor, With<MenuFadeOut>>,
) {
    if !transition.active { return; }
    transition.timer += time.delta_secs();
    let alpha = (transition.timer / MENU_FADE_OUT_DURATION).clamp(0.0, 1.0);

    for mut bg in menu_root_q.iter_mut() {
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    }

    if transition.timer >= MENU_FADE_OUT_DURATION {
        next_state.set(AppPhase::Loading);
    }
}
