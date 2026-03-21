// SPDX-License-Identifier: GPL-3.0-or-later

//! Maps dialog scene nodes to story images.
//! When a dialog node has an associated image, a placeholder (or real image)
//! is shown above the dialog text.

use bevy::prelude::*;
use bevy::render::render_resource::*;

use super::dialog_types::{
    DIALOG_ANNA_COLOR, DIALOG_PANEL_CORNER, DialogImageContainer, DialogState,
};
use super::types::MissionFont;

// === Constants ===

/// Height of the image placeholder / image panel.
const IMAGE_PANEL_HEIGHT: f32 = 200.0;
/// Background color for the placeholder.
const PLACEHOLDER_BG: (f32, f32, f32, f32) = (0.04, 0.05, 0.09, 0.95);
/// Border glow color (Anna-tinted).
const PLACEHOLDER_GLOW_COLOR: (f32, f32, f32, f32) = (0.4, 0.7, 1.0, 0.25);
/// Font size for the prompt description text.
const PLACEHOLDER_PROMPT_FONT: f32 = 12.0;
/// Font size for the label at the bottom.
const PLACEHOLDER_LABEL_FONT: f32 = 10.0;
/// Prompt text color (muted, italic feel via color).
const PLACEHOLDER_PROMPT_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.8);
/// Label text color.
const PLACEHOLDER_LABEL_COLOR: (f32, f32, f32, f32) = (0.4, 0.45, 0.55, 0.6);

// === Marker components ===

/// The image panel container (placeholder or real image).
#[derive(Component)]
pub struct DialogImagePanel;

/// The prompt description text inside the placeholder.
#[derive(Component)]
pub struct DialogImagePromptText;

/// The actual image node (when a real PNG is loaded).
#[derive(Component)]
pub struct DialogImageNode;

// === Scene-to-image mapping ===

/// Maps scene_id + node_index to an image filename (without extension).
/// When a dialog node has an associated image, it's shown above the text.
pub fn scene_image(scene_id: &str, node_index: usize) -> Option<&'static str> {
    match (scene_id, node_index) {
        // === 01: Anna's Portrait ===
        ("awakening", 5) => Some("01_anna_portrait"),

        // === 02: Earth Before the Collapse ===
        ("earth_part1", 0) => Some("02_earth_before"),

        // === 03: Earth After the Collapse ===
        ("earths_last_day", 0) => Some("03_earth_after"),

        // === 04: The Ark Aurora — first time the ship is described ===
        ("the_ship", 1) => Some("04_ark_aurora"),

        // === 05: Cryogenic Chamber — awakening in the pod ===
        ("awakening", 1) => Some("05_cryogenic_chamber"),
        ("awakening", 4) => Some("05_cryogenic_chamber"),

        // === 06: Child in Pod — children who never opened their eyes ===
        ("the_children", 2) => Some("06_child_in_pod"),

        // === 07: Last Forest (Before) — the old-growth forest still alive ===
        ("earth_last_forest", 0) => Some("07_last_forest_before"),

        // === 08: Last Forest (After) — the corporation won ===
        ("earth_last_forest", 6) => Some("08_last_forest_after"),

        // === 09: Nuclear Morning — "It spread." ===
        ("earth_nuclear_morning", 5) => Some("09_nuclear_morning"),

        // === 10: New Earth — first visual of the planet ===
        ("arrival_first_light", 2) => Some("10_new_earth"),

        // === 11: Garden on New Earth — Mei-Lin's garden became a park ===
        ("garden_new_earth", 6) => Some("11_garden_new_earth"),

        // === 12: Anna's Dream — idle cycles produce something like dreaming ===
        ("anna_dream", 0) => Some("12_annas_dream"),

        // === 13: Water Wars — the Euphrates dried up ===
        ("earth_water_wars", 0) => Some("13_water_wars"),

        // === 14: Stowaway's Guitar — the musician in Pod 0 ===
        ("secret_the_stowaway", 8) => Some("14_stowaways_guitar"),

        // === 15: Anna's Creator — Dr. Yuki Tanaka ===
        ("secret_annas_creator", 0) => Some("15_annas_creator"),

        // === 16: Meridian Warning — "Do not approach the third planet" ===
        ("secret_meridian_message", 3) => Some("16_meridian_warning"),

        // === 17: Eighth Awakening — the seven who came before ===
        ("why_you", 4) => Some("17_eighth_awakening"),

        // === 18: Earth's Last Sunset — the last photograph ===
        ("last_photograph", 2) => Some("18_earths_last_sunset"),

        // === 19: The 14,893rd Passenger — one extra person aboard ===
        ("secret_the_stowaway", 1) => Some("19_14893rd_passenger"),

        // === 20: The Anomaly (deeper) — mathematics of absence ===
        ("anomaly_mathematics_of_absence", 0) => Some("20_the_anomaly"),

        // === 21: The Dreamer — Pod 11,237, sleeping mathematician ===
        ("midgame_the_dreamer", 0) => Some("21_the_dreamer"),

        // === 22: Gamification Reveal (Act 3) — the truth about puzzles ===
        ("gamification", 0) => Some("22_gamification_reveal"),

        // === 23: The Signal — transmission from the Meridian ===
        ("the_signal", 0) => Some("23_the_signal"),

        // === 24: Amira's Water — her life's work ===
        ("amiras_water", 3) => Some("24_amiras_water"),

        // === 25: Viktor's Reactor — the air scrubber confession ===
        ("viktors_confession", 4) => Some("25_viktors_reactor"),

        // === 26: Mei-Lin's Seeds — the smuggled plants ===
        ("teachers_garden", 9) => Some("26_meilins_seeds"),

        // === 27: The Twins' Bridge — Kwame and his brother ===
        ("the_twins", 12) => Some("27_the_twins_bridge"),

        // === 28: Anna's Song — the song that kept her sane ===
        ("annas_song", 6) => Some("28_annas_song"),

        // === 29: The Anomaly (midgame) — something in the void ===
        ("midgame_the_anomaly", 0) => Some("29_the_anomaly"),

        // === 30: Gamification Reveal (midgame) — the meta moment ===
        ("midgame_gamification_reveal", 13) => Some("30_gamification_reveal"),

        _ => None,
    }
}

/// Try to load a real PNG image from assets/story/.
/// Returns Some(Handle<Image>) if the file exists, None otherwise.
pub fn try_load_story_image(
    images: &mut Assets<Image>,
    name: &str,
) -> Option<Handle<Image>> {
    let path = format!("assets/story/{name}.png");
    if !std::path::Path::new(&path).exists() {
        return None;
    }
    let img = image::open(&path).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    Some(images.add(Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba.into_raw(),
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )))
}

/// Read the prompt text from the .prompt.txt file for a given image name.
/// Returns a truncated version suitable for display in the placeholder.
pub fn read_prompt_text(name: &str) -> String {
    let path = format!("assets/story/{name}.prompt.txt");
    match std::fs::read_to_string(&path) {
        Ok(text) => {
            let trimmed = text.trim();
            // Truncate to ~300 chars for display
            if trimmed.len() > 300 {
                format!("{}...", &trimmed[..297])
            } else {
                trimmed.to_string()
            }
        }
        Err(_) => format!("Image: {name}"),
    }
}

/// Spawn the image placeholder panel above the dialog text.
/// Shows a dark frame with the prompt description and a label.
pub fn spawn_image_placeholder(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    prompt_text: &str,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(IMAGE_PANEL_HEIGHT),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(16.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(DIALOG_PANEL_CORNER)),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgba(
            PLACEHOLDER_BG.0, PLACEHOLDER_BG.1,
            PLACEHOLDER_BG.2, PLACEHOLDER_BG.3,
        )),
        BorderColor::all(Color::srgba(
            PLACEHOLDER_GLOW_COLOR.0, PLACEHOLDER_GLOW_COLOR.1,
            PLACEHOLDER_GLOW_COLOR.2, PLACEHOLDER_GLOW_COLOR.3,
        )),
        BoxShadow::new(
            Color::srgba(
                DIALOG_ANNA_COLOR.0, DIALOG_ANNA_COLOR.1,
                DIALOG_ANNA_COLOR.2, 0.12,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(4.0), Val::Px(10.0),
        ),
        DialogImagePanel,
    )).with_children(|panel| {
        // Prompt description text
        panel.spawn((
            Text::new(prompt_text),
            TextFont {
                font: font.clone(),
                font_size: PLACEHOLDER_PROMPT_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                PLACEHOLDER_PROMPT_COLOR.0, PLACEHOLDER_PROMPT_COLOR.1,
                PLACEHOLDER_PROMPT_COLOR.2, PLACEHOLDER_PROMPT_COLOR.3,
            )),
            DialogImagePromptText,
        ));

        // Bottom label
        panel.spawn((
            Text::new("[ Image placeholder \u{2014} will be replaced ]"),
            TextFont {
                font: font.clone(),
                font_size: PLACEHOLDER_LABEL_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                PLACEHOLDER_LABEL_COLOR.0, PLACEHOLDER_LABEL_COLOR.1,
                PLACEHOLDER_LABEL_COLOR.2, PLACEHOLDER_LABEL_COLOR.3,
            )),
        ));
    });
}

/// Spawn the real image panel (when a PNG exists).
fn spawn_real_image(
    parent: &mut ChildSpawnerCommands,
    image_handle: Handle<Image>,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(IMAGE_PANEL_HEIGHT),
            border_radius: BorderRadius::all(Val::Px(DIALOG_PANEL_CORNER)),
            overflow: Overflow::clip(),
            ..default()
        },
        BoxShadow::new(
            Color::srgba(
                DIALOG_ANNA_COLOR.0, DIALOG_ANNA_COLOR.1,
                DIALOG_ANNA_COLOR.2, 0.12,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(4.0), Val::Px(10.0),
        ),
        DialogImagePanel,
    )).with_children(|panel| {
        panel.spawn((
            ImageNode::new(image_handle),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            DialogImageNode,
        ));
    });
}

// === Runtime image state ===

/// Tracks which image is currently displayed to avoid redundant spawns.
#[derive(Resource, Default)]
pub struct DialogImageState {
    /// (scene_id, node_index) of the currently displayed image, if any.
    pub current: Option<(String, usize)>,
}

/// System: update the image panel when the dialog node changes.
/// Shows a placeholder (or real image) for nodes that have an image mapping.
pub fn update_dialog_image(
    state: Res<DialogState>,
    mut img_state: ResMut<DialogImageState>,
    mut commands: Commands,
    font: Res<MissionFont>,
    mut images: ResMut<Assets<Image>>,
    container_q: Query<Entity, With<DialogImageContainer>>,
    panel_q: Query<Entity, With<DialogImagePanel>>,
) {
    let active = match &state.active_scene {
        Some(a) => a,
        None => {
            if img_state.current.is_some() {
                img_state.current = None;
                for entity in panel_q.iter() { commands.entity(entity).despawn(); }
            }
            return;
        }
    };

    let scene_id = active.scene.id;
    let node_idx = active.node_index;

    // Check if we already show the right image for this node
    if let Some((ref cur_id, cur_idx)) = img_state.current {
        if cur_id == scene_id && cur_idx == node_idx { return; }
    }

    // Clear existing image panel
    for entity in panel_q.iter() { commands.entity(entity).despawn(); }

    // Look up whether this node has an image
    let image_name = scene_image(scene_id, node_idx);
    img_state.current = Some((scene_id.to_string(), node_idx));

    let name = match image_name {
        Some(n) => n,
        None => return,
    };

    // Try loading real PNG first
    let real_handle = try_load_story_image(&mut images, name);

    for container in container_q.iter() {
        commands.entity(container).with_children(|parent| {
            if let Some(handle) = real_handle.clone() {
                spawn_real_image(parent, handle);
            } else {
                let prompt = read_prompt_text(name);
                spawn_image_placeholder(parent, &font.0, &prompt);
            }
        });
    }
}
