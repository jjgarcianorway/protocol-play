// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu for the standalone bot puzzle game (player mode).
//! Shows an animated 3D board background with parallax stars and a clean title UI.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::spawn_tile;
use crate::ui_helpers::gf;
use crate::player_settings::MenuSettingsBtn;
use crate::i18n::Translations;

// ─── State ────────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerPhase {
    #[default]
    MainMenu,
    Playing,
}

// ─── Resources ────────────────────────────────────────────────────────────────

/// Current camera orbit angle in the main menu.
#[derive(Resource, Default)]
pub struct MenuCamAngle(pub f32);

/// Fade-in progress for menu elements (0.0 → 1.0).
#[derive(Resource, Default)]
pub struct MenuFade(pub f32);

// ─── Components ───────────────────────────────────────────────────────────────

/// Root entity for all menu UI — despawned when leaving MainMenu.
#[derive(Component)]
pub struct MenuUi;

#[derive(Component)]
pub struct MenuPlayBtn;

#[derive(Component)]
pub struct MenuQuitBtn;

/// A parallax star particle.
#[derive(Component)]
pub struct BgStar {
    pub pos_px: Vec2,      // position in pixels
    pub vel_px: Vec2,      // velocity in pixels/second
    pub size_px: f32,
    pub alpha: f32,
    pub layer: u8,
}

/// Marks tiles spawned for the menu background board (so they can be despawned).
#[derive(Component)]
pub struct MenuBgTile;

// ─── Constants ────────────────────────────────────────────────────────────────

const MENU_CAM_RADIUS: f32 = 6.5;
const MENU_CAM_HEIGHT: f32 = 5.2;
const MENU_CAM_SPEED: f32 = 0.18;   // radians/sec
const MENU_FADE_SPEED: f32 = 1.4;
const STAR_LAYERS: [(usize, f32, f32, f32); 3] = [
    // (count, speed px/s, size px, alpha)
    (90, 18.0, 1.5, 0.22),
    (55, 36.0, 2.5, 0.32),
    (28, 64.0, 3.5, 0.45),
];

// ─── Setup ────────────────────────────────────────────────────────────────────

/// OnEnter(MainMenu): spawn the menu UI, stars, and load the background board.
pub fn setup_main_menu(
    mut commands: Commands,
    font: Res<GameFont>,
    mut cam_angle: ResMut<MenuCamAngle>,
    mut fade: ResMut<MenuFade>,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    t: Res<Translations>,
) {
    cam_angle.0 = std::f32::consts::FRAC_PI_4; // start at 45°
    fade.0 = 0.0;

    spawn_bg_board(&mut commands, &assets, &mut board_size);
    spawn_menu_stars(&mut commands);
    spawn_menu_ui(&mut commands, &font.0, &t);
}

/// Spawn a solved showcase board as the 3D background.
fn spawn_bg_board(commands: &mut Commands, assets: &GameAssets, board_size: &mut BoardSize) {
    // Try to load the first campaign level for a nice background.
    // Falls back to a hardcoded 3x3 demo board if unavailable.
    let exe_dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let level = load_first_campaign_level(&exe_dir);
    let (size, tiles) = level.unwrap_or_else(|| demo_board());

    board_size.0 = size;

    for (col, row, kind) in tiles {
        let e = spawn_tile(commands, col, row, size, kind, assets);
        commands.entity(e).insert(MenuBgTile);
    }
}

/// Try loading the first campaign level JSON next to the executable.
fn load_first_campaign_level(dir: &std::path::Path) -> Option<(u32, Vec<(u32, u32, TileKind)>)> {
    let mut files: Vec<_> = std::fs::read_dir(dir).ok()?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.extension().is_some_and(|x| x == "json")
                && !p.file_name().unwrap_or_default().to_string_lossy().ends_with(".progress.json")
                && p.file_name().unwrap_or_default() != "stats.json"
        })
        .map(|e| e.path())
        .collect();
    files.sort();

    // Find a visually interesting level — prefer one from chapter 2 or 3 if available
    let path = files.iter()
        .find(|p| p.file_name().unwrap_or_default().to_string_lossy().starts_with("03_"))
        .or_else(|| files.iter().find(|p| p.file_name().unwrap_or_default().to_string_lossy().starts_with("02_")))
        .or_else(|| files.first())?;

    let json = std::fs::read_to_string(path).ok()?;
    let data: LevelData = serde_json::from_str(&json).ok()?;
    let size = data.board_size;

    // Show the solved state: use canonical solution placements (tiles are in their original positions)
    let tiles: Vec<(u32, u32, TileKind)> = data.tiles.iter()
        .map(|&(c, r, kind, _marked)| (c, r, kind))
        .collect();

    Some((size, tiles))
}

/// Hardcoded 4x4 demo board shown if no level files are found.
fn demo_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    use crate::types::Direction::*;
    let tiles = vec![
        (0, 0, TileKind::Source(0, South)),
        (0, 1, TileKind::Floor),
        (0, 2, TileKind::Turn(0, North)),
        (0, 3, TileKind::Goal(0)),
        (1, 0, TileKind::Floor),
        (1, 1, TileKind::Turn(1, East)),
        (1, 2, TileKind::Floor),
        (1, 3, TileKind::Floor),
        (2, 0, TileKind::Turn(1, North)),
        (2, 1, TileKind::Floor),
        (2, 2, TileKind::Floor),
        (2, 3, TileKind::Goal(1)),
        (3, 0, TileKind::Source(1, West)),
        (3, 1, TileKind::Floor),
        (3, 2, TileKind::Floor),
        (3, 3, TileKind::Floor),
    ];
    (4, tiles)
}

/// Spawn 3 layers of drifting star dots.
fn spawn_menu_stars(commands: &mut Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Use a fixed seed window size estimate for initial placement
    let w = 1280.0f32;
    let h = 800.0f32;

    for (layer, &(count, speed, size, alpha)) in STAR_LAYERS.iter().enumerate() {
        for _ in 0..count {
            let x = rng.gen_range(0.0..w);
            let y = rng.gen_range(0.0..h);
            let angle = rng.gen_range(0.0f32..std::f32::consts::TAU);
            // Stars drift slowly mostly downward with slight horizontal drift
            let vel = Vec2::new(
                angle.cos() * speed * 0.3,
                speed * 0.6 + angle.sin() * speed * 0.2,
            );
            commands.spawn((
                MenuUi, // cleaned up with menu
                BgStar { pos_px: Vec2::new(x, y), vel_px: vel, size_px: size, alpha, layer: layer as u8 },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x),
                    top: Val::Px(y),
                    width: Val::Px(size),
                    height: Val::Px(size),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.78, 0.85, 1.0, 0.0)),
                GlobalZIndex(layer as i32 + 2),
            ));
        }
    }
}

/// Spawn the title screen UI: overlay + title + subtitle + buttons.
fn spawn_menu_ui(commands: &mut Commands, font: &Handle<Font>, t: &Translations) {
    // Full-screen dim overlay
    commands.spawn((
        MenuUi,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.04, 0.04, 0.08, 0.0)), // fades in
        GlobalZIndex(10),
    ));

    // Content panel — centered column
    commands.spawn((
        MenuUi,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(0.0),
            ..default()
        },
        GlobalZIndex(20),
    )).with_children(|root| {
        // Main title
        root.spawn((
            Text::new("protocol play: repairing"),
            TextFont { font: font.clone(), font_size: 48.0, ..default() },
            TextColor(Color::srgba(0.96, 0.96, 0.99, 0.0)), // fades in
        ));

        // Tagline
        root.spawn((
            Text::new(t.ui_or("tagline", "route the bots. repair the ship.").to_string()),
            TextFont { font: font.clone(), font_size: 15.0, ..default() },
            TextColor(Color::srgba(0.60, 0.65, 0.75, 0.0)),
            Node { margin: UiRect::top(Val::Px(8.0)), ..default() },
        ));

        // Spacer
        root.spawn(Node { height: Val::Px(52.0), ..default() });

        // Play button
        root.spawn((
            Button, MenuPlayBtn,
            Node {
                padding: UiRect::axes(Val::Px(56.0), Val::Px(15.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                margin: UiRect::bottom(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.25, 0.45, 0.70, 0.0)),
        )).with_child((
            Text::new(t.ui_or("play", "Play").to_string()),
            TextFont { font: font.clone(), font_size: 20.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        ));

        // Quit button
        root.spawn((
            Button, MenuQuitBtn,
            Node {
                padding: UiRect::axes(Val::Px(56.0), Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).with_child((
            Text::new(t.ui_or("quit", "Quit").to_string()),
            TextFont { font: font.clone(), font_size: 15.0, ..default() },
            TextColor(Color::srgba(0.55, 0.58, 0.65, 0.0)),
        ));

        // Settings link
        root.spawn(Node { height: Val::Px(28.0), ..default() });
        root.spawn((
            Button, MenuSettingsBtn,
            Node { padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)), ..default() },
            BackgroundColor(Color::NONE),
        )).with_child((
            Text::new(t.ui_or("settings", "Settings").to_string()),
            TextFont { font: font.clone(), font_size: 12.0, ..default() },
            TextColor(Color::srgba(0.38, 0.42, 0.50, 0.0)),
        ));
    });
}

// ─── Update systems ───────────────────────────────────────────────────────────

/// Animate: camera orbit, star drift, fade-in.
pub fn animate_menu(
    time: Res<Time>,
    mut cam_angle: ResMut<MenuCamAngle>,
    mut fade: ResMut<MenuFade>,
    mut cam_q: Query<&mut Transform, With<Camera3d>>,
    mut star_q: Query<(&mut BgStar, &mut Node, &mut BackgroundColor)>,
    mut ui_q: Query<(&mut BackgroundColor, Option<&MenuPlayBtn>, Option<&MenuQuitBtn>), Without<BgStar>>,
    mut text_q: Query<&mut TextColor>,
    window_q: Query<&Window>,
) {
    let dt = time.delta_secs();
    fade.0 = (fade.0 + dt * MENU_FADE_SPEED).min(1.0);
    let alpha = ease_in_out(fade.0);

    // Camera orbit
    cam_angle.0 += MENU_CAM_SPEED * dt;
    let a = cam_angle.0;
    if let Ok(mut tf) = cam_q.single_mut() {
        let target = Vec3::new(a.cos() * MENU_CAM_RADIUS, MENU_CAM_HEIGHT, a.sin() * MENU_CAM_RADIUS);
        tf.translation = tf.translation.lerp(target, (dt * 3.0).min(1.0));
        tf.look_at(Vec3::ZERO, Vec3::Y);
    }

    // Window size for star wrapping
    let (w, h) = window_q.single()
        .map(|win| (win.width(), win.height()))
        .unwrap_or((1280.0, 800.0));

    // Animate stars
    for (mut star, mut node, mut bg) in star_q.iter_mut() {
        let vel = star.vel_px;
        let sa = star.alpha;
        star.pos_px += vel * dt;
        let pos = star.pos_px;
        // Wrap
        if star.pos_px.x < 0.0 { star.pos_px.x += w; }
        if star.pos_px.x > w { star.pos_px.x -= w; }
        if star.pos_px.y < 0.0 { star.pos_px.y += h; }
        if star.pos_px.y > h { star.pos_px.y -= h; }
        node.left = Val::Px(pos.x);
        node.top = Val::Px(pos.y);
        bg.0 = Color::srgba(0.78, 0.85, 1.0, sa * alpha);
    }

    // Fade in overlay, text, buttons
    for (mut bg, play_btn, quit_btn) in ui_q.iter_mut() {
        if play_btn.is_some() {
            bg.0 = Color::srgba(0.25, 0.45, 0.70, alpha * 0.85);
        } else if quit_btn.is_some() {
            // quit button stays transparent bg
        } else {
            // dim overlay
            bg.0 = Color::srgba(0.04, 0.04, 0.08, alpha * 0.62);
        }
    }
    for mut tc in text_q.iter_mut() {
        let [r, g, b, _] = tc.0.to_srgba().to_f32_array();
        tc.0 = Color::srgba(r, g, b, alpha);
    }
}

/// Handle Play / Quit button presses in the main menu.
pub fn menu_buttons(
    play_q: Query<&Interaction, (With<MenuPlayBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<MenuQuitBtn>, Changed<Interaction>)>,
    mut next: ResMut<NextState<PlayerPhase>>,
) {
    if play_q.iter().any(|i| *i == Interaction::Pressed) {
        next.set(PlayerPhase::Playing);
    }
    if quit_q.iter().any(|i| *i == Interaction::Pressed) {
        std::process::exit(0);
    }
}

/// Hover effect for Play button.
pub fn menu_btn_hover(
    mut play_q: Query<(&Interaction, &mut BackgroundColor), (With<MenuPlayBtn>, Without<MenuQuitBtn>, Changed<Interaction>)>,
    mut quit_q: Query<(&Interaction, &mut BackgroundColor), (With<MenuQuitBtn>, Without<MenuPlayBtn>, Changed<Interaction>)>,
) {
    for (interaction, mut bg) in play_q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered => Color::srgba(0.30, 0.52, 0.80, 0.95),
            Interaction::Pressed => Color::srgba(0.20, 0.38, 0.60, 1.0),
            _ => Color::srgba(0.25, 0.45, 0.70, 0.85),
        };
    }
    for (interaction, mut bg) in quit_q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered => Color::srgba(0.3, 0.3, 0.35, 0.3),
            _ => Color::NONE,
        };
    }
}

// ─── Cleanup ──────────────────────────────────────────────────────────────────

/// OnExit(MainMenu): despawn all menu UI entities and background board tiles.
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_ui: Query<Entity, With<MenuUi>>,
    bg_tiles: Query<Entity, With<MenuBgTile>>,
    tiles: Query<Entity, With<Tile>>,
) {
    for e in menu_ui.iter() { commands.entity(e).despawn(); }
    for e in bg_tiles.iter() { commands.entity(e).despawn(); }
    // Also despawn any existing tiles so setup_player can respawn them cleanly
    for e in tiles.iter() { commands.entity(e).despawn(); }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}
