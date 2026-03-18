// SPDX-License-Identifier: GPL-3.0-or-later

//! Stats/Decision tracking screen — "Your Story" overlay.
//! Shows the player's journey: decisions made, game statistics, and crew manifest.

use bevy::prelude::*;

// === Component markers ===

/// Marker for the stats overlay root (full-screen backdrop).
#[derive(Component)]
pub struct StatsOverlay;

/// Marker for the stats panel glow (animated).
#[derive(Component)]
pub struct StatsPanelGlow;

/// Marker for the "Your Story" button in the main menu.
#[derive(Component)]
#[allow(dead_code)]
pub struct YourStoryBtn;

// === Constants ===

pub const STATS_OVERLAY_BG: (f32, f32, f32, f32) = (0.02, 0.03, 0.06, 0.92);
pub const STATS_PANEL_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const STATS_PANEL_CORNER: f32 = 14.0;
pub const STATS_MAX_WIDTH: f32 = 900.0;
pub const STATS_MAX_HEIGHT_PCT: f32 = 85.0;
pub const STATS_PADDING: f32 = 28.0;

pub const STATS_TITLE_FONT: f32 = 24.0;
pub const STATS_TITLE_COLOR: (f32, f32, f32) = (0.7, 0.8, 0.95);
pub const STATS_SECTION_FONT: f32 = 16.0;
pub const STATS_SECTION_COLOR: (f32, f32, f32) = (0.6, 0.7, 0.85);
pub const STATS_BODY_FONT: f32 = 14.0;
pub const STATS_BODY_COLOR: (f32, f32, f32) = (0.7, 0.75, 0.85);
pub const STATS_DIM_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.7);
pub const STATS_HIGHLIGHT_COLOR: (f32, f32, f32) = (0.85, 0.9, 1.0);
pub const STATS_DECISION_COLOR: (f32, f32, f32) = (0.75, 0.82, 0.95);
pub const STATS_HINT_FONT: f32 = 12.0;
pub const STATS_HINT_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.6);

pub const STATS_GLOW_COLOR: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.12);
pub const STATS_GLOW_BLUR: f32 = 25.0;
pub const STATS_GLOW_SPREAD: f32 = 8.0;

pub const STATS_COLUMN_GAP: f32 = 28.0;
pub const STATS_ROW_GAP: f32 = 6.0;

// === Decision mapping ===

/// Map a decision key to a human-readable description.
pub fn decision_description(key: &str) -> Option<&'static str> {
    match key {
        // Character decisions — Amira arc
        "amira_build" => Some("You chose to build Amira's water system"),
        "amira_delay" => Some("You delayed Amira's water project"),
        "amira_trust" => Some("You trusted Amira's expertise"),
        "amira_override" => Some("You overrode Amira's recommendation"),
        "amira_sacrifice" => Some("You let Amira sacrifice her comfort"),
        "amira_protect" => Some("You protected Amira from the council"),

        // Viktor arc
        "viktor_redeemed" => Some("You believed Viktor deserved redemption"),
        "viktor_punished" => Some("You chose to hold Viktor accountable"),
        "viktor_forgiven" => Some("You forgave Viktor's past mistakes"),
        "viktor_exiled" => Some("You exiled Viktor from the community"),
        "viktor_trust" => Some("You trusted Viktor with the reactor"),
        "viktor_restrict" => Some("You restricted Viktor's access"),

        // Mei-Lin / Seeds
        "seeds_protect" => Some("You chose to protect Mei-Lin's seeds"),
        "seeds_share" => Some("You shared the seed vault openly"),
        "seeds_ration" => Some("You rationed the seed supply"),
        "seeds_plant_now" => Some("You planted the seeds immediately"),
        "seeds_save" => Some("You saved the seeds for New Earth"),

        // Twins (Kwame & Kofi)
        "twins_together" => Some("You kept the twins working together"),
        "twins_separate" => Some("You separated the twins for efficiency"),
        "twins_lead" => Some("You let the twins lead hull repairs"),
        "twins_supervise" => Some("You supervised the twins' work"),

        // Anna
        "anna_trust" => Some("You placed full trust in Anna"),
        "anna_limit" => Some("You limited Anna's autonomy"),
        "anna_confide" => Some("You confided your fears to Anna"),
        "anna_distance" => Some("You kept emotional distance from Anna"),
        "anna_upgrade" => Some("You upgraded Anna's capabilities"),
        "anna_restrict_ai" => Some("You restricted Anna's growth"),

        // Faction choices
        "faction_founders" => Some("You aligned with the Founders"),
        "faction_pioneers" => Some("You aligned with the Pioneers"),
        "faction_keepers" => Some("You aligned with the Keepers"),
        "faction_unite" => Some("You united the factions"),
        "faction_neutral" => Some("You remained neutral between factions"),
        "faction_whitfield" => Some("You sided with Whitfield's Preservationists"),
        "faction_volkov" => Some("You sided with Volkov's Progressives"),
        "faction_rashidi" => Some("You sided with al-Rashidi's Pragmatists"),

        // Governance
        "constitution_democracy" => Some("You chose democratic governance"),
        "constitution_council" => Some("You chose council-based governance"),
        "constitution_meritocracy" => Some("You chose meritocratic governance"),
        "constitution_consensus" => Some("You chose consensus governance"),
        "constitution_captain" => Some("You chose to remain sole captain"),

        // First word
        "first_word_hope" => Some("Your first word was 'Hope'"),
        "first_word_survive" => Some("Your first word was 'Survive'"),
        "first_word_together" => Some("Your first word was 'Together'"),
        "first_word_remember" => Some("Your first word was 'Remember'"),
        "first_word_forward" => Some("Your first word was 'Forward'"),

        // Earth collapse (world seed)
        "collapse_climate" => Some("Earth fell to climate collapse"),
        "collapse_resource" => Some("Earth fell to resource wars"),
        "collapse_pandemic" => Some("Earth fell to a pandemic cascade"),
        "collapse_nuclear" => Some("Earth fell to nuclear exchange"),
        "collapse_ai" => Some("Earth fell to an AI uprising"),
        "collapse_political" => Some("Earth fell to political collapse"),
        "collapse_economic" => Some("Earth fell to economic meltdown"),

        // Severity
        "severity_mild" => Some("The collapse was gradual — some survived"),
        "severity_moderate" => Some("The collapse was devastating"),
        "severity_extreme" => Some("The collapse was near-total extinction"),

        // Key story decisions
        "revealed_truth" => Some("You revealed the truth to the crew"),
        "kept_secret" => Some("You kept the truth hidden"),
        "sacrificed_power" => Some("You sacrificed power for the crew"),
        "preserved_power" => Some("You preserved power at all costs"),
        "saved_stowaway" => Some("You saved the stowaway"),
        "reported_stowaway" => Some("You reported the stowaway"),
        "broadcast_sent" => Some("You sent the final broadcast to Earth"),
        "broadcast_silence" => Some("You chose silence toward Earth"),
        "children_teach" => Some("You taught the children the full truth"),
        "children_protect" => Some("You shielded the children from the truth"),
        "memorial_built" => Some("You built a memorial for the lost"),
        "orben_played" => Some("You learned to play Orben"),

        _ => None,
    }
}

/// Group label for categorizing decisions in the UI.
pub fn decision_group(key: &str) -> &'static str {
    if key.starts_with("amira") { return "Amira's Arc"; }
    if key.starts_with("viktor") { return "Viktor's Arc"; }
    if key.starts_with("seeds") || key.starts_with("meilin") { return "Mei-Lin's Seeds"; }
    if key.starts_with("twins") || key.starts_with("kwame") { return "The Twins"; }
    if key.starts_with("anna") { return "Anna"; }
    if key.starts_with("faction") { return "Faction Politics"; }
    if key.starts_with("constitution") { return "Governance"; }
    if key.starts_with("first_word") { return "First Word"; }
    if key.starts_with("collapse") || key.starts_with("severity") { return "Earth's Fate"; }
    "Key Moments"
}

// === Systems ===

/// System: dismiss stats overlay on ESC or background click.
pub fn stats_dismiss(
    overlay_q: Query<(&Interaction, Entity), (With<StatsOverlay>, With<Button>)>,
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

/// System: animate the stats panel glow with a gentle pulse.
pub fn animate_stats_glow(
    time: Res<Time>,
    mut query: Query<&mut BoxShadow, With<StatsPanelGlow>>,
    overlay_q: Query<Entity, With<StatsOverlay>>,
) {
    if overlay_q.is_empty() { return; }
    let t = time.elapsed_secs();
    let pulse = 0.08 + 0.05 * (t * 1.2).sin();
    for mut shadow in query.iter_mut() {
        *shadow = BoxShadow::new(
            Color::srgba(
                STATS_GLOW_COLOR.0, STATS_GLOW_COLOR.1,
                STATS_GLOW_COLOR.2, pulse,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(STATS_GLOW_SPREAD), Val::Px(STATS_GLOW_BLUR),
        );
    }
}

