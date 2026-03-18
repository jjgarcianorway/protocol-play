// SPDX-License-Identifier: GPL-3.0-or-later

//! Crew Manifest / Character Codex — data registry and systems.

use bevy::prelude::*;
use crate::save_state::GameState;
use super::codex_ui;
use super::types::MissionFont;

/// Static character entry in the codex registry.
pub struct CodexEntry {
    pub scene_id: &'static str,
    pub name: &'static str,
    pub pod: &'static str,
    pub role: &'static str,
    pub description: &'static str,
}

/// All known codex entries. Order determines display order.
pub const CODEX_ENTRIES: &[CodexEntry] = &[
    CodexEntry {
        scene_id: "amiras_water",
        name: "Dr. Amira Hassan",
        pod: "Pod 4,231",
        role: "Hydrologist",
        description: "Expert in water reclamation and purification systems.",
    },
    CodexEntry {
        scene_id: "viktors_confession",
        name: "Viktor Petrov",
        pod: "Pod 8,744",
        role: "Nuclear Engineer",
        description: "Maintains the ark's reactor core and power grid.",
    },
    CodexEntry {
        scene_id: "teachers_garden",
        name: "Mei-Lin Chen",
        pod: "Pod 2,891",
        role: "Schoolteacher",
        description: "Preserves knowledge and teaches the ship's children.",
    },
    CodexEntry {
        scene_id: "the_twins",
        name: "Kwame & Kofi Asante",
        pod: "Pod 6,100",
        role: "Structural Engineers",
        description: "Twin brothers who keep the hull integrity in check.",
    },
    CodexEntry {
        scene_id: "annas_song",
        name: "Anna",
        pod: "Ship AI",
        role: "Artificial Intelligence",
        description: "The heart and voice of the Aurora.",
    },
    CodexEntry {
        scene_id: "architects_city",
        name: "Yuki Tanabe",
        pod: "Pod 9,415",
        role: "Urban Planner",
        description: "Dreams of the cities humanity will build upon arrival.",
    },
    CodexEntry {
        scene_id: "last_broadcast",
        name: "Marcus Cole",
        pod: "Pod 5,776",
        role: "Journalist",
        description: "Records the truth of the journey for future generations.",
    },
    CodexEntry {
        scene_id: "geneticists_dilemma",
        name: "Dr. Aisha Okonkwo",
        pod: "Pod 10,302",
        role: "Geneticist",
        description: "Guardians of the genetic diversity vault.",
    },
    CodexEntry {
        scene_id: "composers_silence",
        name: "Tomas Herrera",
        pod: "Pod 7,891",
        role: "Composer",
        description: "Writes music no one may ever hear again.",
    },
    CodexEntry {
        scene_id: "immigrants_bread",
        name: "Carlos Mendoza",
        pod: "Pod 3,445",
        role: "Electrician",
        description: "Keeps the lights on — literally and figuratively.",
    },
    CodexEntry {
        scene_id: "believers_fire",
        name: "Sister Magdalena Santos",
        pod: "Pod 1,208",
        role: "Astrophysicist",
        description: "Finds faith in the stars and equations alike.",
    },
    CodexEntry {
        scene_id: "generals_mercy",
        name: "General Fatou Diallo",
        pod: "Pod 10,150",
        role: "Military Commander",
        description: "Enforces order when diplomacy fails.",
    },
    CodexEntry {
        scene_id: "coders_silence",
        name: "Priya Nair",
        pod: "Pod 6,891",
        role: "Software Engineer",
        description: "Patches the systems Anna cannot reach alone.",
    },
    CodexEntry {
        scene_id: "faction_whitfield",
        name: "James Whitfield",
        pod: "Pod 1,001",
        role: "Faction Leader — Preservationists",
        description: "Believes survival means holding on to the old ways.",
    },
    CodexEntry {
        scene_id: "faction_volkov",
        name: "Kira Volkov",
        pod: "Pod 5,200",
        role: "Faction Leader — Progressives",
        description: "Pushes for radical change to ensure the mission succeeds.",
    },
    CodexEntry {
        scene_id: "faction_rashidi",
        name: "Hassan al-Rashidi",
        pod: "Pod 7,500",
        role: "Faction Leader — Pragmatists",
        description: "Seeks balance and compromise between the factions.",
    },
    CodexEntry {
        scene_id: "mutiny_that_almost_was",
        name: "Dr. Nkechi Obi",
        pod: "Pod 3,205",
        role: "Constitutional Lawyer",
        description: "Led the mutiny that almost split the ark in two.",
    },
    CodexEntry {
        scene_id: "viktors_witness",
        name: "Dr. Sophia Marchand",
        pod: "Pod 9,012",
        role: "Trauma Surgeon",
        description: "Survived Viktor's weapons in Marseille and bears witness.",
    },
    CodexEntry {
        scene_id: "midgame_the_dreamer",
        name: "Dr. Priya Sharma",
        pod: "Pod 11,237",
        role: "Neuroscientist",
        description: "The Dreamer — her pod emits anomalous equations in sleep.",
    },
    CodexEntry {
        scene_id: "bright_radio_volunteers",
        name: "Marta",
        pod: "Pod 9,900",
        role: "Radio Volunteer",
        description: "Relayed forty-seven thousand messages before the silence.",
    },
    CodexEntry {
        scene_id: "bright_osaka_cherry",
        name: "Hanako Mori",
        pod: "Deceased",
        role: "Cherry Tree Planter",
        description: "Planted cherry trees in a dying Osaka — her seeds fly onward.",
    },
    CodexEntry {
        scene_id: "collapse_political_translators_burden",
        name: "Fatima al-Zahra",
        pod: "Pod 3,300",
        role: "UN Translator",
        description: "Heard every argument in every language for twenty-eight years.",
    },
    CodexEntry {
        scene_id: "collapse_economic_last_billionaire",
        name: "Chen Wei",
        pod: "Pod 12,001",
        role: "Last Billionaire",
        description: "Bought his family's seats — forty-seven candidates were bumped.",
    },
    CodexEntry {
        scene_id: "the_twins",
        name: "Adaeze",
        pod: "Pod 6,101",
        role: "Kofi's Replacement",
        description: "A stranger given Kofi's seat so the twins' sacrifice meant life.",
    },
    CodexEntry {
        scene_id: "amiras_daughter",
        name: "Leyla Hassan",
        pod: "Pod 4,232",
        role: "Amira's Daughter",
        description: "Smuggled aboard at age seven — never selected, always loved.",
    },
    CodexEntry {
        scene_id: "collapse_nuclear_alexei_daughter",
        name: "Alexei Volkov",
        pod: "Stayed on Earth",
        role: "Submarine Captain",
        description: "Kira's father — the man who launched first and never left Earth.",
    },
];

// === Component markers ===

/// Marker for the Crew Manifest button on the dashboard.
#[derive(Component)]
pub struct CrewManifestBtn;

/// Marker for the full-screen codex overlay.
#[derive(Component)]
pub struct CodexOverlay;

/// Marker for a single codex entry row (stores index into CODEX_ENTRIES).
#[derive(Component)]
pub struct CodexEntryRow(#[allow(dead_code)] pub usize);

/// Marker for the codex panel glow (animated).
#[derive(Component)]
pub struct CodexPanelGlow;

// === Constants ===

pub const CODEX_OVERLAY_BG: (f32, f32, f32, f32) = (0.02, 0.03, 0.06, 0.92);
pub const CODEX_PANEL_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const CODEX_PANEL_CORNER: f32 = 14.0;
pub const CODEX_MAX_WIDTH: f32 = 660.0;
pub const CODEX_MAX_HEIGHT: f32 = 80.0; // percent
pub const CODEX_PADDING: f32 = 30.0;

pub const CODEX_TITLE_FONT: f32 = 22.0;
pub const CODEX_TITLE_COLOR: (f32, f32, f32) = (0.7, 0.8, 0.95);
pub const CODEX_NAME_FONT: f32 = 16.0;
pub const CODEX_DETAIL_FONT: f32 = 13.0;
pub const CODEX_LOCKED_COLOR: (f32, f32, f32, f32) = (0.35, 0.38, 0.45, 0.6);
pub const CODEX_UNLOCKED_NAME_COLOR: (f32, f32, f32) = (0.9, 0.92, 1.0);
pub const CODEX_UNLOCKED_DETAIL_COLOR: (f32, f32, f32) = (0.6, 0.65, 0.75);
pub const CODEX_UNLOCKED_GLOW: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.08);
pub const CODEX_ENTRY_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.7);
pub const CODEX_ENTRY_CORNER: f32 = 8.0;
pub const CODEX_ENTRY_PAD: f32 = 12.0;
pub const CODEX_HINT_FONT: f32 = 12.0;
pub const CODEX_HINT_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.6);

pub const CODEX_GLOW_COLOR: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.12);
pub const CODEX_GLOW_BLUR: f32 = 25.0;
pub const CODEX_GLOW_SPREAD: f32 = 8.0;

pub const CODEX_BTN_BG: (f32, f32, f32, f32) = (0.15, 0.12, 0.08, 0.9);
pub const CODEX_BTN_BORDER: (f32, f32, f32, f32) = (0.5, 0.6, 0.8, 0.7);
pub const CODEX_BTN_TEXT_COLOR: (f32, f32, f32) = (0.75, 0.85, 1.0);
pub const CODEX_BTN_FONT: f32 = 16.0;

// === Systems ===

/// System: handle Crew Manifest button click — open codex overlay.
pub fn crew_manifest_click(
    query: Query<&Interaction, (Changed<Interaction>, With<CrewManifestBtn>)>,
    overlay_q: Query<Entity, With<CodexOverlay>>,
    mut commands: Commands,
    gs: Res<GameState>,
    font: Res<MissionFont>,
) {
    // Don't open if already open
    if !overlay_q.is_empty() {
        return;
    }
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            codex_ui::spawn_codex_overlay(&mut commands, &font.0, &gs);
        }
    }
}

/// System: dismiss codex overlay on click (outside entries) or ESC.
pub fn codex_dismiss(
    overlay_q: Query<(&Interaction, Entity), (With<CodexOverlay>, With<Button>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for (_, entity) in overlay_q.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }
    for (interaction, entity) in overlay_q.iter() {
        if *interaction == Interaction::Pressed {
            commands.entity(entity).despawn();
        }
    }
}

/// System: animate the codex panel glow with a gentle pulse.
pub fn animate_codex_glow(
    time: Res<Time>,
    mut query: Query<&mut BoxShadow, With<CodexPanelGlow>>,
    overlay_q: Query<Entity, With<CodexOverlay>>,
) {
    if overlay_q.is_empty() {
        return;
    }
    let t = time.elapsed_secs();
    let pulse = 0.08 + 0.05 * (t * 1.2).sin();
    for mut shadow in query.iter_mut() {
        *shadow = BoxShadow::new(
            Color::srgba(
                CODEX_GLOW_COLOR.0, CODEX_GLOW_COLOR.1,
                CODEX_GLOW_COLOR.2, pulse,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(CODEX_GLOW_SPREAD), Val::Px(CODEX_GLOW_BLUR),
        );
    }
}
